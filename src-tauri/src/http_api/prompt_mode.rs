use super::server::{run_transcription_job, short_request_id};
use super::types::{
    PromptModeErrorResponse, PromptModeForm, PromptModeListResponse, PromptModeRejection,
    PromptModeResponse, TranscribeState,
};
use crate::audio;
use crate::llm::{
    self, ApiLlmRequest, ApiLlmTranscription, CustomPromptError, LLMProvider, PromptSelectionError,
};
use axum::{
    body::Bytes,
    extract::{multipart::MultipartRejection, Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::info;
use std::collections::HashMap;

pub(super) async fn custom_prompt_handler(
    State(state): State<TranscribeState>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Response {
    let request = read_custom_prompt_request(&state, multipart).await;
    respond_to_prompt_request(&state, request).await
}

pub(super) async fn saved_prompt_handler(
    State(state): State<TranscribeState>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Response {
    let request = read_saved_prompt_request(&state, multipart).await;
    respond_to_prompt_request(&state, request).await
}

async fn respond_to_prompt_request(
    state: &TranscribeState,
    request: Result<(Bytes, ApiLlmRequest), PromptModeRejection>,
) -> Response {
    match request {
        Ok((audio, request)) => run_prompt_mode(state, audio, request).await,
        Err(rejection) => rejection.into_response(),
    }
}

pub(super) async fn prompt_mode_prompts_handler(State(state): State<TranscribeState>) -> Response {
    let settings = llm::load_llm_connect_settings(&state.app);
    let prompts = settings.mode_names();
    (StatusCode::OK, Json(PromptModeListResponse { prompts })).into_response()
}

async fn read_custom_prompt_request(
    state: &TranscribeState,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<(Bytes, ApiLlmRequest), PromptModeRejection> {
    let mut form = PromptModeForm::read(multipart, &["instruction", "provider", "model"]).await?;
    let audio = form.take_audio()?;
    let instruction = form.take_text("instruction")?;
    let provider = form.take_text("provider")?;
    let model = form.take_text("model")?;
    let provider = match provider.as_str() {
        "local" => LLMProvider::Local,
        "remote" => LLMProvider::Remote,
        _ => {
            return Err(invalid_request(
                "'provider' must be 'local' or 'remote'.".to_string(),
            ))
        }
    };

    let settings = llm::load_llm_connect_settings(&state.app);
    llm::check_api_custom_prompt(&settings, &provider, &instruction)
        .map_err(custom_prompt_rejection)?;

    Ok((
        audio,
        ApiLlmRequest::Custom {
            provider,
            model,
            instruction,
        },
    ))
}

async fn read_saved_prompt_request(
    state: &TranscribeState,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<(Bytes, ApiLlmRequest), PromptModeRejection> {
    let mut form = PromptModeForm::read(multipart, &["prompt"]).await?;
    let audio = form.take_audio()?;
    let name = form.take_text("prompt")?;

    let settings = llm::load_llm_connect_settings(&state.app);
    llm::resolve_api_prompt(&settings, &name).map_err(prompt_selection_rejection)?;

    Ok((audio, ApiLlmRequest::SavedPrompt { name }))
}

async fn run_prompt_mode(
    state: &TranscribeState,
    audio_bytes: Bytes,
    request: ApiLlmRequest,
) -> Response {
    let id = uuid::Uuid::new_v4();
    info!(
        "HTTP API prompt mode {}: request accepted",
        short_request_id(&id)
    );

    let job = run_transcription_job(state, id, audio_bytes, move |app, path, cancelled| {
        audio::transcribe_file_with_llm_cancellable(app, path, cancelled, &request)
    });
    match job.await {
        Ok(Some(ApiLlmTranscription {
            transcription,
            outcome: Ok(text),
        })) => (
            StatusCode::OK,
            Json(PromptModeResponse {
                text,
                transcription,
            }),
        )
            .into_response(),
        Ok(Some(ApiLlmTranscription {
            transcription,
            outcome: Err(error),
        })) => (
            StatusCode::BAD_GATEWAY,
            Json(PromptModeErrorResponse {
                error,
                code: "llm_failed",
                available: None,
                transcription: Some(transcription),
            }),
        )
            .into_response(),
        Ok(None) => transcription_failed("Transcription cancelled".to_string()).into_response(),
        Err(e) => transcription_failed(e).into_response(),
    }
}

impl PromptModeForm {
    async fn read(
        multipart: Result<Multipart, MultipartRejection>,
        text_fields: &[&'static str],
    ) -> Result<Self, PromptModeRejection> {
        let mut multipart = multipart.map_err(multipart_rejection)?;
        let mut form = PromptModeForm {
            audio: None,
            texts: HashMap::new(),
        };
        loop {
            match multipart.next_field().await {
                Ok(Some(field)) if field.name() == Some("audio") => match field.bytes().await {
                    Ok(b) => form.audio = Some(b),
                    Err(e) => {
                        return Err(invalid_request(format!("Failed to read audio file: {}", e)))
                    }
                },
                Ok(Some(field)) => {
                    let Some(name) = text_fields
                        .iter()
                        .find(|name| field.name() == Some(**name))
                        .copied()
                    else {
                        continue;
                    };
                    match field.text().await {
                        Ok(value) => {
                            form.texts.insert(name, value.trim().to_string());
                        }
                        Err(e) => {
                            return Err(invalid_request(format!(
                                "Failed to parse multipart: {}",
                                e
                            )))
                        }
                    }
                }
                Ok(None) => break,
                Err(e) => return Err(invalid_request(format!("Failed to parse multipart: {}", e))),
            }
        }
        Ok(form)
    }

    fn take_audio(&mut self) -> Result<Bytes, PromptModeRejection> {
        self.audio
            .take()
            .filter(|bytes| !bytes.is_empty())
            .ok_or_else(|| invalid_request("No 'audio' field in multipart request".to_string()))
    }

    fn take_text(&mut self, name: &str) -> Result<String, PromptModeRejection> {
        self.texts
            .remove(name)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| invalid_request(format!("Missing '{}' field.", name)))
    }
}

fn multipart_rejection(_: MultipartRejection) -> PromptModeRejection {
    invalid_request("The request must be multipart/form-data.".to_string())
}

fn prompt_mode_error(status: StatusCode, code: &'static str, error: String) -> PromptModeRejection {
    (
        status,
        Json(PromptModeErrorResponse {
            error,
            code,
            available: None,
            transcription: None,
        }),
    )
}

fn invalid_request(error: String) -> PromptModeRejection {
    prompt_mode_error(StatusCode::BAD_REQUEST, "invalid_request", error)
}

fn transcription_failed(error: String) -> PromptModeRejection {
    prompt_mode_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "transcription_failed",
        error,
    )
}

fn custom_prompt_rejection(error: CustomPromptError) -> PromptModeRejection {
    let (status, code) = match error {
        CustomPromptError::TooLong => (StatusCode::BAD_REQUEST, "instruction_too_long"),
        CustomPromptError::Disabled => (StatusCode::CONFLICT, "prompt_mode_disabled"),
        CustomPromptError::RemoteNotConfigured => {
            (StatusCode::UNPROCESSABLE_ENTITY, "provider_not_configured")
        }
    };
    prompt_mode_error(status, code, error.to_string())
}

fn prompt_selection_rejection(error: PromptSelectionError) -> PromptModeRejection {
    let message = error.to_string();
    match error {
        PromptSelectionError::Disabled => {
            prompt_mode_error(StatusCode::CONFLICT, "prompt_mode_disabled", message)
        }
        PromptSelectionError::NotFound { available, .. } => {
            let (status, Json(mut response)) =
                prompt_mode_error(StatusCode::NOT_FOUND, "prompt_not_found", message);
            response.available = Some(available);
            (status, Json(response))
        }
        PromptSelectionError::NotConfigured { .. } => prompt_mode_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "prompt_not_configured",
            message,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::PromptModeForm;
    use axum::{
        body::Body,
        extract::{FromRequest, Multipart, Request},
        http::StatusCode,
        Json,
    };

    #[tokio::test]
    async fn non_multipart_body_is_an_invalid_request() {
        let request = Request::builder().body(Body::empty()).unwrap();
        let multipart = Multipart::from_request(request, &()).await;
        let Err((status, Json(response))) = PromptModeForm::read(multipart, &[]).await else {
            panic!("a request without multipart body must be rejected");
        };
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.code, "invalid_request");
    }
}
