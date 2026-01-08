# Privacy Policy (Murmure)

**Last updated:** January 8, 2026

Murmure is a privacy-first, open-source speech-to-text desktop application. It is designed to work without an internet connection and to keep your speech data on your device.

## Summary
- Murmure does **not** include analytics or telemetry.
- Your audio and transcriptions are processed **locally** on your device.
- Murmure stores only what it needs to operate (settings, custom words/rules, and the last 5 transcriptions).
- On startup, Murmure checks for updates by contacting the configured updater endpoint (currently hosted on GitHub). Other network features are opt-in (downloading updates, enabling the local HTTP API, or connecting to an LLM endpoint you configured).

## What Murmure does not do
- It does not collect or sell user data.
- It does not upload your recordings or transcriptions to a cloud service by default.
- It does not track your usage with analytics/telemetry.

## Data stored on your device
Murmure stores the following locally in the app’s data directory:
- **Settings** (shortcuts, display settings, language)
- **Custom dictionary words**
- **Formatting rules**
- **Up to 5 most recent transcriptions** (can be cleared; depending on your settings, this history may be persisted on disk or kept only in memory)

This local data is not transmitted by default.

## Network usage
**This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it.**

When you use network-related features, standard network metadata (such as your IP address) will necessarily be visible to the service you contact.

Murmure may access the network only for:
- **Checking for updates** (runs automatically on launch; currently not disableable in-app)
- **Downloading and installing updates** (only when you choose to install an update)
- **Connecting to an LLM endpoint** (e.g., a local Ollama server, or another endpoint you configured)
- **Serving a local HTTP API** (only if you enable it; it listens on `127.0.0.1` on the configured port)

The update check does not include your audio or transcriptions.

When connecting to an LLM endpoint, Murmure sends the text you ask it to process (e.g., transcriptions and your prompt) to the URL you configured. If that URL points to a remote server, data will be sent to that server.

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
Murmure is released under the GNU GPL v3 License. The source code can be audited by anyone:

[https://github.com/Kieirra/murmure](https://github.com/Kieirra/murmure)

## Disclaimer of warranty (legal)
**THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED.**

To the maximum extent permitted by law, the authors and contributors shall not be liable for any claim, damages, or other liability arising from, out of, or in connection with the software or the use of the software.
