# Murmure Local HTTP API

## Overview

The local HTTP API allows other applications to send audio files to Murmure for transcription without using the graphical interface. This is useful for integrating Murmure as a backend Speech-to-Text engine in other applications.

## Enabling the API

1. Open Murmure
2. Go to **Settings** → **System**
3. Find **Local HTTP API (Experimental)**
4. Toggle it on
5. (Optional) Change the port number if needed (default: 4800)

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

## Notes

- The API only listens on localhost (127.0.0.1) for privacy and security
- Transcription requests are processed sequentially due to the single transcription engine
- The model must be available (downloaded) before transcription can work
- Custom dictionary settings are automatically applied to the transcription
