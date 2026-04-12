# Local API

!!! info "Experimental"
The local HTTP API is experimental. The API design may change in future releases.

The local API allows other applications to send audio files to Murmure for transcription without using the GUI.

## Quick Start

1. Open Murmure
2. Go to **Settings** > **System**
3. Find **Local API (Experimental)** and toggle it **ON**
4. The API starts on `http://localhost:4800`
5. (Optional) Change the port if needed

The API runs as long as Murmure is open.

## Endpoint

**POST** `http://localhost:4800/api/transcribe`

Send a multipart form with a WAV file:

```bash
curl -X POST http://127.0.0.1:4800/api/transcribe \
  -F "audio=@recording.wav" \
  | jq '.text'
```

### Response

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

## Code Examples

=== "Python"

    ```python
    import requests

    with open('audio.wav', 'rb') as f:
        response = requests.post(
            'http://localhost:4800/api/transcribe',
            files={'audio': f}
        )
        print(response.json()['text'])
    ```

=== "JavaScript"

    ```javascript
    const fs = require('fs');
    const FormData = require('form-data');
    const axios = require('axios');

    const form = new FormData();
    form.append('audio', fs.createReadStream('recording.wav'));

    const response = await axios.post(
      'http://localhost:4800/api/transcribe',
      form,
      { headers: form.getHeaders() }
    );
    console.log(response.data.text);
    ```

=== "Bash"

    ```bash
    curl -X POST http://localhost:4800/api/transcribe \
      -F "audio=@recording.wav" \
      | jq '.text'
    ```

## Limitations

| Constraint          | Value                             |
| ------------------- | --------------------------------- |
| Audio format        | WAV only                          |
| Max file size       | 100 MB                            |
| Optimal sample rate | 16kHz mono (others are resampled) |
| Real-time streaming | Not supported                     |
| Concurrent requests | Sequential only (queued)          |
| Network access      | localhost / 127.0.0.1 only        |
| CORS                | Disabled                          |

## Notes

- The custom dictionary is automatically applied to API transcriptions
- Language is auto-detected (no way to force a language)
- The first request is slower (model warmup)
- The port can be configured between 1024 and 65535
