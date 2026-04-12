# Troubleshooting

Find solutions to the most common issues. These pages are based on real user reports.

## Most Common Issues

1. **[Transcription in the wrong language](transcription.md)** - Murmure transcribes in English when you speak French (or another language)
2. **[Text doesn't appear in my application](text-insertion.md)** - Transcription works but text is not inserted
3. **[Shortcuts don't work](shortcuts.md)** - The recording shortcut has no effect
4. **[LLM Connect errors](llm-connect.md)** - Ollama 500 errors, slow responses, model issues

## Quick Fixes

| Problem                      | Quick Fix                                                                            |
| ---------------------------- | ------------------------------------------------------------------------------------ |
| Wrong language               | Check microphone quality, reduce noise                                               |
| Text not pasted              | Switch to Direct mode (Settings > System)                                            |
| Shortcut conflict (macOS)    | Change shortcut to Ctrl+Option+M                                                     |
| Shortcut not working (Linux) | Switch to X11 session                                                                |
| MSVCP140.dll error (Windows) | Install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe) |
| Ollama 500 error             | Use a smaller model (qwen3.5:4b)                                                     |
| Settings corrupted           | Delete settings.json and restart                                                     |

## How to Get Logs

If you need to report a bug, enable debug logging first:

1. Go to **Settings** > **System**
2. Set **Log level** to **Debug**
3. Click the folder icon next to the log level to open the log directory
4. Reproduce the issue
5. Attach the log file to your [GitHub issue](https://github.com/Kieirra/murmure/issues/new)

## Settings File Locations

| OS      | Path                                                              |
| ------- | ----------------------------------------------------------------- |
| Windows | `%APPDATA%\com.al1x-ai.murmure\settings.json`                     |
| macOS   | `~/Library/Application Support/com.al1x-ai.murmure/settings.json` |
| Linux   | `~/.local/share/com.al1x-ai.murmure/settings.json`                |

To reset all settings, delete the `settings.json` file and restart Murmure.
