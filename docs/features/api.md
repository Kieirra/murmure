# Local API

The local API allows other applications to send audio files to Murmure for transcription without using the GUI. It can also apply a Prompt Mode prompt to the transcription and return the final text.

## Quick Start

1. Open Murmure
2. Go to **Extensions** > **Local API**
3. Click **Enable Local API**
4. The API starts on `http://127.0.0.1:4800`
5. (Optional) Change the port in the **Server** block

The API runs as long as Murmure is open. The page shows the server status. If the port is already used by another application, choose another port and the server restarts on its own.

The same page documents each endpoint with its `curl` command, its fields and an example response. A **Try it** button sends a real request to the API from the page, so you can test an endpoint with a WAV file without writing any code. The result of a test is not saved and disappears when you leave the page.

## Transcribe

**POST** `http://127.0.0.1:4800/api/transcribe`

Send a multipart form with a WAV file:

```bash
curl -X POST http://127.0.0.1:4800/api/transcribe \
  -F "audio=@recording.wav" \
  | jq '.text'
```

### Transcribe response

**Success (200):**

```json
{
    "text": "Hello everyone, here is the complete transcript..."
}
```

**Error (4xx/5xx):**

```json
{
    "error": "Error message describing what went wrong"
}
```

## Prompt Mode

Two endpoints transcribe the audio, send the text to a language model and return the final text. A third one lists your saved prompts.

| Endpoint                           | What it does                                                                |
| ---------------------------------- | --------------------------------------------------------------------------- |
| `POST /api/prompt-mode/custom`     | Applies a prompt that you send in the request, with the provider and model you choose |
| `POST /api/prompt-mode`            | Applies one of your saved Prompt Mode prompts, selected by name             |
| `GET /api/prompt-mode/prompts`     | Returns the names of your saved prompts                                     |

Before you use them, note the following points:

- [Prompt Mode](llm-connect.md) must be enabled in **Extensions** > **Prompt Mode**. If it is disabled, both POST endpoints return a `409` error.
- The API uses the connection already configured in Murmure. The Ollama URL, the remote server URL and the API key are never sent in a request.
- An API call has no effect in Murmure. There is no overlay, sound, notification or result card. Nothing is added to the history or the statistics, nothing is pasted or copied, and the active prompt and your settings do not change.
- Requests are handled one at a time, like `/api/transcribe`. The call to the language model is part of the request, so the response arrives after both steps.
- The Ollama server or the remote server can be slow on its first call. The timeouts are 120 seconds for Ollama (local) and 60 seconds for a remote server.

### Custom prompt

**POST** `http://127.0.0.1:4800/api/prompt-mode/custom`

All fields are required, sent as `multipart/form-data`:

| Field         | Description                                                                        |
| ------------- | ---------------------------------------------------------------------------------- |
| `audio`       | WAV file                                                                           |
| `instruction` | Your prompt, 4000 characters maximum                                               |
| `provider`    | `local` (Ollama) or `remote` (your OpenAI-compatible server)                       |
| `model`       | Model name on this provider                                                        |

```bash
curl -X POST http://127.0.0.1:4800/api/prompt-mode/custom \
  -F "audio=@recording.wav" \
  -F "instruction=Summarize in three bullet points" \
  -F "provider=local" \
  -F "model=qwen3:8b"
```

The instruction is sent as the system prompt and the transcription as the user prompt. Prompt variables such as `{{TRANSCRIPT}}` are not replaced. Murmure does not check that the model exists before the call. An unknown model returns a `502` error.

!!! warning "Remote provider"
    With the custom prompt, any program on your computer can use the language model configured in Murmure while the API is enabled. This includes a remote server and the API key stored for it. The API is disabled by default and only listens on `127.0.0.1`. As usual, Murmure refuses to send the API key to a public server over plain HTTP.

### Saved prompt

**POST** `http://127.0.0.1:4800/api/prompt-mode`

All fields are required, sent as `multipart/form-data`:

| Field    | Description                                                                   |
| -------- | ----------------------------------------------------------------------------- |
| `audio`  | WAV file                                                                      |
| `prompt` | Exact name of a saved prompt. The match is case sensitive and spaces around the name are ignored |

```bash
curl -X POST http://127.0.0.1:4800/api/prompt-mode \
  -F "audio=@recording.wav" \
  -F "prompt=Email"
```

The prompt text, the provider and the model saved in Murmure are used, as with the Prompt Mode shortcut. The active prompt is neither used nor changed. If two prompts have the same name, the first one is used.

### List the prompts

**GET** `http://127.0.0.1:4800/api/prompt-mode/prompts`

```bash
curl http://127.0.0.1:4800/api/prompt-mode/prompts
```

```json
{
    "prompts": ["General", "Email"]
}
```

Only the names are returned. This endpoint also works when Prompt Mode is disabled.

### Prompt Mode response

**Success (200):**

```json
{
    "text": "- First point\n- Second point\n- Third point",
    "transcription": "Hello everyone, here is the complete transcript..."
}
```

- `text` is the output of the language model, with your formatting rules applied
- `transcription` is the text that `/api/transcribe` returns for the same audio

If the audio is silent, both fields are empty and the language model is not called.

### Prompt Mode errors

Errors have this shape:

```json
{
    "error": "Message describing what went wrong",
    "code": "prompt_mode_disabled"
}
```

| Status | `code`                    | When                                                                                           |
| ------ | ------------------------- | ---------------------------------------------------------------------------------------------- |
| 400    | `invalid_request`         | The request is not a valid multipart form, a field is missing or empty, or `provider` is not `local` or `remote` |
| 400    | `instruction_too_long`    | The instruction is longer than 4000 characters                                                 |
| 404    | `prompt_not_found`        | No saved prompt has this name                                                                  |
| 409    | `prompt_mode_disabled`    | Prompt Mode is disabled                                                                        |
| 422    | `prompt_not_configured`   | The saved prompt has no prompt text or no model                                                |
| 422    | `provider_not_configured` | `provider` is `remote` but no remote server is configured in Prompt Mode                       |
| 500    | `transcription_failed`    | The transcription failed, for the same reasons as `/api/transcribe`                            |
| 502    | `llm_failed`              | The language model call failed, for example Ollama is stopped or the model does not exist      |

The checks for `400`, `404`, `409` and `422` are done before the transcription, so a rejected request costs no transcription time.

Two errors carry an extra field:

- `404 prompt_not_found` adds `available`, the list of saved prompt names
- `502 llm_failed` adds `transcription`, so you do not lose the transcript when only the language model failed

```json
{
    "error": "Prompt \"Mail\" not found.",
    "code": "prompt_not_found",
    "available": ["General", "Email"]
}
```

## Code Examples

=== "Python"

    ```python
    import requests

    with open('audio.wav', 'rb') as f:
        response = requests.post(
            'http://127.0.0.1:4800/api/transcribe',
            files={'audio': f}
        )
        print(response.json()['text'])
    ```

    With Prompt Mode:

    ```python
    import requests

    with open('audio.wav', 'rb') as f:
        response = requests.post(
            'http://127.0.0.1:4800/api/prompt-mode/custom',
            files={'audio': f},
            data={
                'instruction': 'Summarize in three bullet points',
                'provider': 'local',
                'model': 'qwen3:8b',
            },
        )

    result = response.json()
    if response.ok:
        print(result['text'])
    else:
        print(result['code'], result['error'])
    ```

=== "JavaScript"

    ```javascript
    const fs = require('fs');
    const FormData = require('form-data');
    const axios = require('axios');

    const form = new FormData();
    form.append('audio', fs.createReadStream('recording.wav'));

    const response = await axios.post(
      'http://127.0.0.1:4800/api/transcribe',
      form,
      { headers: form.getHeaders() }
    );
    console.log(response.data.text);
    ```

=== "Bash"

    ```bash
    curl -X POST http://127.0.0.1:4800/api/transcribe \
      -F "audio=@recording.wav" \
      | jq '.text'
    ```

## Limitations

| Constraint          | Value                                                                                     |
| ------------------- | ----------------------------------------------------------------------------------------- |
| Audio format        | WAV only                                                                                  |
| Max file size       | 100 MB                                                                                    |
| Audio length        | No limit, only the 100 MB file size applies                                               |
| Cancellation        | Close the connection. Stops at the end of the current audio segment                       |
| Optimal sample rate | 16kHz mono (others are resampled)                                                         |
| Real-time streaming | Not supported                                                                             |
| Concurrent requests | Sequential only (queued). A cancelled request frees its slot once it has actually stopped |
| Network access      | 127.0.0.1 only                                                                            |
| Host header         | Requests with a `Host` header other than `localhost` or a loopback IP (such as `127.0.0.1`) are rejected with a `403` error |
| CORS                | Disabled. Requests that send a non-localhost `Origin` are rejected. curl (no Origin) is allowed. Web pages cannot read the responses. Only the Murmure window gets CORS headers, for the **Try it** button. |

## Notes

- The custom dictionary is automatically applied to API transcriptions
- Language is auto-detected (no way to force a language)
- The first request is slower (model warmup)
- The port can be configured between 1024 and 65535
- If you already used the API before it moved to Extensions, it stays enabled with the same port
- Long audio is split into segments before transcription, the same way the keyboard shortcut and the CLI do it. There is no duration limit.
- The response is synchronous. A long file keeps the connection open for several minutes, so disable the timeout in your HTTP client or set it well above the expected duration: a timeout that fires cancels the transcription.
- Filler removal and formatting rules are applied to the result, so the API returns the same text as the keyboard shortcut for the same audio.
