# Local HTTP API Implementation - Notes

## Summary of Changes

This implementation adds a local HTTP API endpoint to Murmure, allowing external applications to send audio files for transcription without using the GUI.

### Backend Changes (src-tauri/)

#### 1. Dependencies (`Cargo.toml`)
- Added `tokio` v1 with full features for async runtime
- Added `axum` v0.7 with `multipart` feature for HTTP server
- Added `tower` v0.4 and `tower-http` v0.5 for middleware
- Added `hyper` v1 for HTTP protocol
- Added `uuid` v1 with `v4` feature for temp file naming
- Added `parking_lot` v0.12 to non-Android/iOS targets (already used in audio.rs)

#### 2. New Module (`src/http_api.rs`)
- `start_http_api(app, port)` - Async function to start the HTTP server
- `transcribe_handler()` - Handles POST requests to `/api/transcribe`
- `TranscriptionResponse` - JSON response struct
- `ErrorResponse` - JSON error struct

Server listens on `http://127.0.0.1:{port}/api/transcribe`

#### 3. Settings (`src/settings.rs`)
- Added `api_enabled: bool` (default: false)
- Added `api_port: u16` (default: 4800)

#### 4. Commands (`src/commands.rs`)
- `get_api_enabled()` - Returns if API is enabled
- `set_api_enabled(enabled)` - Enable/disable API
- `get_api_port()` - Returns current port
- `set_api_port(port)` - Set port (validates 1024-65535)

#### 5. Audio (`src/audio.rs`)
- Made `transcribe_audio()` function public (was private)
- No other changes to existing functions

#### 6. Main App (`src/lib.rs`)
- Added `mod http_api`
- Integrated HTTP API startup in setup phase
- Spawns async tokio runtime in separate thread if `api_enabled`
- Added 4 new commands to invoke handler

### Frontend Changes (src/)

#### 1. New Hook (`src/features/settings/system/hooks/use-api-state.ts`)
- `useApiState()` - Manages API settings state
- Handles get/set for both `apiEnabled` and `apiPort`

#### 2. UI Component (`src/features/settings/system/system.tsx`)
- Added "Local HTTP API (Experimental)" toggle in System settings
- Conditional "API Port" input field (shows only when enabled)
- Displays current API URL

### Documentation

#### 1. API Usage Guide (`API_USAGE.md`)
- Complete API documentation
- Usage examples in Python, JavaScript, and bash/curl
- Requirements and response format

#### 2. Implementation Notes (`IMPLEMENTATION_NOTES.md`)
- This file - detailed change description

## Testing on Windows

### Prerequisites
- Rust installed via rustup
- Windows 10/11

### Build Steps
```bash
cd src-tauri
cargo check --lib        # Verify compilation
cargo build --release    # Full build (slow first time)
```

### Running the App
```bash
# From project root
cargo tauri dev          # Development mode
cargo tauri build        # Production build
```

### API Testing
Once the app is running with API enabled:

#### PowerShell
```powershell
# Create multipart form data and send
$filePath = "path\to\audio.wav"
$form = @{
    'audio' = Get-Item -Path $filePath
}

Invoke-RestMethod -Uri "http://localhost:4800/api/transcribe" `
    -Method Post `
    -Form $form
```

#### curl (if available)
```bash
curl -X POST http://localhost:4800/api/transcribe ^
  -F "audio=@C:\path\to\audio.wav"
```

#### Python
```python
import requests

with open('audio.wav', 'rb') as f:
    files = {'audio': f}
    response = requests.post('http://localhost:4800/api/transcribe', files=files)
    print(response.json())
```

## Key Design Decisions

1. **Localhost only** - For security/privacy, API only listens on 127.0.0.1
2. **Multipart form** - Standard way to send files over HTTP
3. **WAV format only** - Simplifies audio handling, leverages existing code
4. **Sequential processing** - Engine is single-threaded due to Mutex protection
5. **Async HTTP server** - Tokio in separate thread to not block GUI
6. **Settings persistence** - Port and enabled state saved to settings.json

## Potential Future Enhancements

- [ ] Real-time streaming support (requires Parakeet changes)
- [ ] Support for more audio formats (MP3, OGG, etc.)
- [ ] WebSocket endpoint for long-running requests
- [ ] Authentication/API key support
- [ ] Rate limiting
- [ ] Request queueing with status tracking

## Files Modified

```
src-tauri/
├── Cargo.toml (dependencies)
├── src/
│   ├── lib.rs (module + setup)
│   ├── http_api.rs (NEW)
│   ├── audio.rs (made transcribe_audio public)
│   ├── settings.rs (api_enabled, api_port fields)
│   └── commands.rs (4 new commands)

src/features/settings/system/
├── system.tsx (API UI)
└── hooks/
    └── use-api-state.ts (NEW)

API_USAGE.md (NEW)
IMPLEMENTATION_NOTES.md (NEW)
```

## Error Handling

- File too large (>100MB) → 413 Payload Too Large
- No audio field → 400 Bad Request
- Failed to read file → 400 Bad Request
- Model not available → 500 Internal Server Error
- Transcription failed → 500 Internal Server Error
- All errors return JSON: `{"error": "message"}`

## Performance Notes

- Transcription is synchronous due to engine design
- Requests queue on mutex lock
- No concurrent transcriptions (by design)
- Temp files cleaned up after each request
- Dictionary corrections applied automatically
