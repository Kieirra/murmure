# Testing HTTP API Port Blocking Scenario

This guide explains how to test the HTTP API error handling when a port is already in use.

## Prerequisites

- Murmure application compiled and ready to run (`npm run tauri dev`)
- Python 3.x installed (for the port blocking script)
- A terminal/command prompt

## Test Scenario 1: Block Port with Python Script

### Setup

1. Open a terminal and navigate to the project root:

2. Start the port blocking script on port 4800:
   ```bash
   python3 test_port_blocking.py 4800
   ```

   You should see:
   ```
   Blocking port 4800
   Press Ctrl+C to stop
   ```

3. Leave this terminal open and running

### Test Steps

1. Start Murmure in another terminal:
   ```bash
   npm run tauri dev
   ```

2. Once Murmure is open, go to **Settings → System**

3. In the "API Port" field, ensure it's set to **4800**

4. Toggle **"Local HTTP API (Experimental)"** to **ON**

### Expected Behavior

You should see an error dialog with the message:
```
Failed to start HTTP API on port 4800.

The port is already in use by another application.

Please change the port in Settings → System → API Port to an available port (1024-65535).
```

### Verification Steps

1. Click **OK** to close the error dialog
2. Change the API Port to **4801** (or any available port)
3. Toggle **"Local HTTP API (Experimental)"** to **ON** again
4. The server should start successfully on port 4801
5. You should **NOT** see an error dialog this time

### Cleanup

1. In the first terminal running the port blocking script, press **Ctrl+C** to stop it
2. The port 4800 is now free
3. You can optionally toggle the API to verify it still works

## Test Scenario 2: Windows PowerShell Method (Alternative)

If you prefer not to use Python, you can use PowerShell to check ports:

### Check what's using a port

```powershell
netstat -ano | findstr :4800
```

### Find an application blocking the port

```powershell
# List all listening ports
netstat -ano | findstr LISTENING

# Kill a specific process (replace PID with actual process ID)
taskkill /PID 1234 /F
```

## Test Scenario 3: Server Shutdown Test

### Steps

1. Ensure the API is enabled and running in Murmure settings
2. Open Settings → System
3. Toggle **"Local HTTP API (Experimental)"** to **OFF**
4. The server should stop gracefully
5. Toggle it back to **ON** and verify it starts again

## Success Criteria

All tests pass when:

- ✅ Error dialog appears when port is already in use
- ✅ Error message clearly explains the problem and solution
- ✅ Changing the port and re-enabling API works correctly
- ✅ Server starts normally when no port conflict exists
- ✅ Server stops gracefully when toggled OFF
- ✅ Server restarts correctly after being stopped

## Notes

- The error handling in `src-tauri/src/commands.rs` lines 198-217 detects port conflicts
- The `HttpApiState` manages the server lifecycle via oneshot channels
- Graceful shutdown is implemented using `tokio::select!` in `src-tauri/src/http_api.rs` lines 52-59
