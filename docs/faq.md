# FAQ

## General

### Does Murmure need an internet connection?

No. All transcription happens locally on your machine. No data is ever sent to any server.

### Does Murmure collect any data?

No. Zero telemetry, zero analytics, zero tracking. See the [Privacy Policy](https://github.com/Kieirra/murmure/blob/main/PRIVACY_POLICY.md).

### Does Murmure need a GPU?

No. Murmure runs on CPU. A GPU is not required for transcription. However, if you use [LLM Connect](features/llm-connect.md), a GPU significantly improves the LLM inference speed.

### What languages does Murmure support?

25 European languages: Bulgarian, Croatian, Czech, Danish, Dutch, English, Estonian, Finnish, French, German, Greek, Hungarian, Italian, Latvian, Lithuanian, Maltese, Polish, Portuguese, Romanian, Slovak, Slovenian, Spanish, Swedish, Russian, Ukrainian.

### Can I force a specific language?

Not yet. Parakeet auto-detects the language from the audio. A language selector is on the roadmap.

### Is there a mobile app?

No. Murmure requires access to the clipboard and keyboard simulation, which mobile operating systems don't allow. You can, however, use the [Smart Speech Mic](features/smart-speech-mic.md) to use your phone as a wireless microphone.

### Is there a web version?

No, for the same reasons as mobile - browser sandboxing prevents clipboard and keyboard access.

## Transcription

### Why does Murmure transcribe in English when I speak French?

This is almost always a microphone quality issue. Poor audio, background noise, or low volume causes the model to default to English. See [Transcription Troubleshooting](troubleshooting/transcription.md) for a step-by-step fix.

### What is the maximum recording duration?

5 minutes. After that, recording stops automatically and transcription begins.

### Can Murmure transcribe meetings or multiple speakers?

Meeting transcription with speaker diarization (identifying who said what) is not currently supported. Murmure is designed for single-speaker dictation.

### Can Murmure transcribe audio files?

Yes, via the [Local API](features/api.md). Send a WAV file to the API endpoint and get the transcription back.

## Installation

### Where are the logs?

1. Go to **Settings** > **System**
2. Set **Log level** to **Debug**
3. Click the **folder icon** next to the log level selector
4. The log files are in that directory

### Where is the settings file?

| OS      | Path                                                              |
| ------- | ----------------------------------------------------------------- |
| Windows | `%APPDATA%\com.al1x-ai.murmure\settings.json`                     |
| macOS   | `~/Library/Application Support/com.al1x-ai.murmure/settings.json` |
| Linux   | `~/.local/share/com.al1x-ai.murmure/settings.json`                |

### How do I reset all settings?

Delete the `settings.json` file at the path above and restart Murmure.

### Is Murmure available via Flatpak?

Not currently. Flatpak sandboxing conflicts with global keyboard shortcuts. This may change with Wayland support in 1.9.0.

## Features

### What is the difference between Dictionary and Formatting Rules?

**Dictionary** uses phonetic matching to correct mis-recognized words (great for proper nouns).

**Formatting Rules** do text find-and-replace, including regex (great for voice commands, multi-word replacements, and entries with numbers or special characters).

See [Dictionary](features/dictionary.md) and [Formatting Rules](features/formatting-rules.md) for details.

### Can I use Murmure with ChatGPT/Claude/Cursor?

Yes. Murmure works with any application - it simply types text into whatever window is focused. Just open your AI chat, press the recording shortcut, speak, and the text appears in the chat input.

### Can I use a Stream Deck with Murmure?

Yes. Assign an F13-F24 key to your Stream Deck button, then set that key as the Murmure shortcut. F13-F24 support was added in version 1.8.0.

### How do I deploy Murmure on multiple workstations?

Use the [CLI](features/cli.md) with silent MSI install:

```powershell
msiexec /package Murmure_x64.msi /quiet
murmure.exe import company-config.murmure
```

## Troubleshooting

### Murmure shows "MSVCP140.dll not found"

Install the [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe).

### Text doesn't appear in LibreOffice

Change the text insertion mode to **Direct** in Settings > System. See [Text Insertion Troubleshooting](troubleshooting/text-insertion.md).

### Shortcuts don't work on macOS after updating

You need to reset permissions. See [macOS Upgrade Guide](getting-started/macos.md#upgrading-from-160).

### Shortcuts don't work on Linux

You're likely on Wayland. Murmure requires an X11 session. See [Linux Installation](getting-started/linux.md).
