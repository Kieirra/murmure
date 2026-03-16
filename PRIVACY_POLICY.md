# Privacy Policy (Murmure)

**Last updated:** March 16, 2026

Murmure is a privacy-first, open-source speech-to-text desktop application. It is designed to work without an internet connection and to keep your speech data on your device.

## Summary

- Murmure does **not** include analytics or telemetry.
- Your audio and transcriptions are processed **locally** on your device.
- Murmure stores only what it needs to operate (settings, custom words/rules). No transcription is written to disk by default.
- On startup, Murmure checks for updates by contacting the configured updater endpoint (currently hosted on GitHub). Other network features are opt-in (downloading updates, enabling the local HTTP API, or connecting to an LLM endpoint you configured).

## What Murmure does not do

- It does not collect or sell user data.
- It does not upload your recordings or transcriptions to a cloud service by default.
- It does not track your usage with analytics/telemetry.

## Data stored on your device

Murmure stores the following locally in the app's data directory:

- **Settings** (shortcuts, display settings, language)
- **Custom dictionary words**
- **Formatting rules**
- **LLM Connect configuration** (endpoint URLs, mode names, prompts). API keys are stored separately in your operating system's secure keyring (Apple Keychain, Windows Credential Manager, or Linux Secret Service) and are **never written to configuration files**.
- **Application logs** (do not contain transcriptions by default)

By default, **no transcription is persisted to disk**. Transcription history is kept in memory only and is cleared when the application closes. If you explicitly enable the "Persist history" option, the last 5 transcriptions will be saved on disk in the app's data directory.

This local data is not transmitted by default.

## Voice mode

Voice mode allows Murmure to listen for a wake word to start recording hands-free. This feature is **disabled by default** and must be explicitly enabled by the user.

When enabled, voice mode maintains a continuous audio stream from your microphone to detect speech. Only detected speech segments are transcribed locally to check for wake word matches. The raw audio is **immediately discarded** after processing. All processing happens on-device — no audio or transcription data leaves your computer.

When disabled (default), Murmure does not listen to your microphone outside of an active recording session.

## Settings import/export

Murmure allows exporting and importing settings as `.murmure` files. These files may contain: system settings, shortcuts, formatting rules, LLM Connect configuration (endpoint URLs, modes, prompts), and dictionary words.

**API keys are never included** in exported files. They remain in your operating system's secure keyring.

## Network usage

**This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it.**

When you use network-related features, standard network metadata (such as your IP address) will necessarily be visible to the service you contact.

Murmure may access the network only for:

- **Checking for updates** (runs automatically on launch; currently not disableable in-app)
- **Downloading and installing updates** (only when you choose to install an update)
- **Connecting to an LLM endpoint** (e.g., a local Ollama server, or a remote OpenAI-compatible server you configured)
- **Serving a local HTTP API** (only if you enable it; it listens on `127.0.0.1` on the configured port)

The update check does not include your audio or transcriptions.

When connecting to an LLM endpoint, Murmure sends the text you ask it to process (e.g., transcriptions and your prompt) to the URL you configured. If that URL points to a remote server, your transcription data will be sent to that server. To protect your data:

- Murmure **blocks sending API keys over unencrypted HTTP** connections — only HTTPS or local/private addresses are allowed when an API key is configured.
- During setup, Murmure displays a **clear warning** that your transcriptions will be sent to the remote server for processing.
- The recommended default setup uses a **local** LLM server (e.g., Ollama) where no data leaves your device.

If you enable the local HTTP API, it can be accessed by other software running on your device.

Third-party policy (GitHub): [GitHub Privacy Statement](https://docs.github.com/en/site-policy/privacy-policies/github-privacy-statement)

## Clipboard access

To insert transcriptions into other applications, Murmure may:

- temporarily write the transcription to the system clipboard,
- simulate a paste action,
- restore the previous clipboard content unless you enabled an option to keep the transcription in the clipboard.

Clipboard content is not transmitted by Murmure.

## Security practices

We use static analysis tools such as **SonarQube** to help detect potential vulnerabilities and security issues. Automated analysis does not cover 100% of the code or all scenarios, and it is not a guarantee of security.

## Open source

Murmure is released under the GNU AGPL v3 License. The source code can be audited by anyone:

[https://github.com/Kieirra/murmure](https://github.com/Kieirra/murmure)

## Disclaimer of warranty (legal)

**THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED.**

To the maximum extent permitted by law, the authors and contributors shall not be liable for any claim, damages, or other liability arising from, out of, or in connection with the software or the use of the software.
