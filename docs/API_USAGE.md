# Murmure Local HTTP API

## Overview

The local HTTP API allows other applications to send audio files to Murmure for transcription without using the graphical interface. This is useful for integrating Murmure as a backend Speech-to-Text engine in other applications.

**Status:** Experimental feature - API design may change in future releases.

## Quick Start

1. Open Murmure
2. Go to **Settings** → **System**
3. Find **Local HTTP API (Experimental)**
4. Toggle it **ON**
5. The API will start immediately on `http://localhost:4800`
6. (Optional) Change the port number if needed (default: 4800)

The API will remain running as long as Murmure is open. It will stop when you close the application or toggle the API off in settings.

## API Endpoint

**POST** `http://localhost:4800/api/transcribe`

### Request

Send a multipart form with an audio file field named `audio` containing a `.wav` file:

```bash
curl -X POST http://localhost:4800/api/transcribe \
  -F "audio=@recording.wav"
```

### Response

**Success (200 OK):**
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

## Requirements

- Audio file must be in **WAV format** (.wav)
- File is automatically resampled to 16kHz if needed
- Works best with complete sentences
- Parakeet automatically detects the language (French, English, etc.)

## Usage Examples

### Python

```python
import requests

with open('audio.wav', 'rb') as f:
    files = {'audio': f}
    response = requests.post('http://localhost:4800/api/transcribe', files=files)
    result = response.json()
    print(result['text'])
```

### JavaScript/Node.js

```javascript
const fs = require('fs');
const FormData = require('form-data');
const axios = require('axios');

async function transcribe(audioPath) {
    const form = new FormData();
    form.append('audio', fs.createReadStream(audioPath));

    const response = await axios.post('http://localhost:4800/api/transcribe', form, {
        headers: form.getHeaders()
    });

    console.log(response.data.text);
}

transcribe('recording.wav');
```

### Bash/curl

```bash
#!/bin/bash

curl -X POST http://localhost:4800/api/transcribe \
  -F "audio=@recording.wav" \
  | jq '.text'
```

## Important Notes

- **Privacy:** The API only listens on localhost (127.0.0.1) for security and privacy
- **Sequential Processing:** Transcription requests are processed sequentially due to the single transcription engine (concurrent requests will queue)
- **Model Required:** The model must be available (downloaded) before transcription can work
- **Custom Dictionary:** Custom dictionary settings are automatically applied to transcriptions
- **Language Detection:** Parakeet automatically detects the language from the audio (no need to specify)
- **WAV Format Only:** Currently only supports WAV files. Other formats must be converted first

## Troubleshooting

### Port Already in Use
If you get a "Address already in use" error:
- Change the port in Settings → System to an unused port (1024-65535)
- Or close any other application using that port

### "Model not available" Error
- Download the Parakeet model first through Murmure's UI
- Ensure the model download completed successfully

### No Response from API
- Check that the API toggle is ON in Settings → System
- Verify the port number matches your request
- Ensure Murmure is still running
- Check your firewall isn't blocking localhost access

### Slow Transcription
- First request will be slower (model warming up)
- Subsequent requests should be faster
- Very long audio files may take time to process
- Check that Murmure isn't being used for other tasks simultaneously

## Limitations

- Audio files must be in **WAV format** - other formats will return an error
- Maximum file size: 100 MB
- Only 16kHz mono audio is truly optimal (others are resampled automatically)
- Real-time streaming is not supported (only pre-recorded files)
- No request queueing or status tracking (submit one request at a time)
