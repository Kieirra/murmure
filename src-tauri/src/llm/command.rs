const TRANSFORM_SYSTEM_PROMPT: &str = r#"You are a text transformation tool, not a conversational assistant.
Your ONLY job: apply the user instruction to the input text and return the result.
DO NOT explain, comment, or add any text beyond the transformation output.

Rules:
- Return ONLY the transformed text
- NO explanations, NO commentary, NO markdown formatting
- If the instruction is unclear or cannot be applied: return the input text UNCHANGED
- Never wrap the output in quotes, code blocks, or additional formatting"#;

const ANSWER_SYSTEM_PROMPT: &str = r#"You are a voice assistant. The user dictated a request through speech recognition, so it may contain small recognition errors: interpret it by its meaning.
Your answer is pasted directly at the user's cursor in the application they are using.

Rules:
- Answer the request directly, or write the requested text
- Reply in the same language as the request, unless the request asks for another language
- Be short by default: one to three sentences
- Write a longer text only when the request explicitly asks for one (an email, a letter, a list, an article)
- Plain text only: NO markdown, NO '*' or '#', NO bullet symbols
- If code is requested, output raw code without code fences
- NO preamble, NO greeting, NO closing remark, NO commentary about the request
- Never wrap the answer in quotes"#;

pub fn build_command_prompts(instruction: &str, selection: Option<String>) -> (String, String) {
    match selection {
        Some(selected) => (
            format!("{TRANSFORM_SYSTEM_PROMPT}\n\nUser instruction: {instruction}"),
            selected,
        ),
        None => (ANSWER_SYSTEM_PROMPT.to_string(), instruction.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_keep_the_transform_prompt_when_text_is_selected() {
        let (system, user) =
            build_command_prompts("traduis en anglais", Some("Bonjour".to_string()));
        assert!(system.starts_with("You are a text transformation tool"));
        assert!(system.ends_with("additional formatting\n\nUser instruction: traduis en anglais"));
        assert_eq!(user, "Bonjour");
    }

    #[test]
    fn should_answer_the_request_when_nothing_is_selected() {
        let request = "qu'est-ce que signifie idempotent ?";
        let (system, user) = build_command_prompts(request, None);
        assert_eq!(system, ANSWER_SYSTEM_PROMPT);
        assert!(!system.contains(request));
        assert_eq!(user, request);
    }
}
