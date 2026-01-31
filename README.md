# Murmure

A privacy-first, open-source speech-to-text application that runs entirely on your machine, powered by a neural network via NVIDIA’s [Parakeet TDT 0.6B v3 model](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3) for fast, local transcription. Murmure turns your voice into text with no internet connection and zero data collection, and supports 25 European languages.

Learn more on the on the [official website](https://murmure.al1x-ai.com/).

![demo](public/murmure-screenshot-beautiful.png)

## Features

- **Privacy First**: All processing happens locally on your device. No data ever leaves your computer.
- **No Telemetry**: Zero tracking, zero analytics. Your data stays yours, always.
- **Open Source**: Free and open source software. Inspect, modify, and contribute.
- **Powered by [Parakeet TDT 0.6B v3](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3)**: NVIDIA’s latest model state-of-the-art speech recognition model runs entirely on-device for fast, low-latency transcription.

## Supported Languages:

Bulgarian (bg), Croatian (hr), Czech (cs), Danish (da), Dutch (nl), English (en), Estonian (et), Finnish (fi), French (fr), German (de), Greek (el), Hungarian (hu), Italian (it), Latvian (lv), Lithuanian (lt), Maltese (mt), Polish (pl), Portuguese (pt), Romanian (ro), Slovak (sk), Slovenian (sl), Spanish (es), Swedish (sv), Russian (ru), Ukrainian (uk)

## Installation

### Windows (Official)

⚠️ Windows SmartScreen : This installer is **not signed with a commercial certificate** (which costs ~€200–€500/year).  
If you downloaded it from our **official GitHub releases**, you can safely continue.

🛡️ We guarantee the installer is safe, contains **no malware**, and you can verify the source code or even compile it yourself if you prefer.

1. Download Murmure_x64.msi from the [release](https://github.com/Kieirra/murmure/releases) page
2. Run the installer and follow the setup wizard.
3. Or install via WinGet (after package approval): `winget install Kieirra.Murmure`

> [!WARNING]
> Murmure requires the [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist) to work on Windows. This package is present on most computers, but if you encounter the error message `The code execution cannot proceed because MSVCP140.dll was not found. Reinstalling the program may fix this problem.`, download and install the package from the official page or use this direct download link: [https://aka.ms/vc14/vc_redist.x64.exe](https://aka.ms/vc14/vc_redist.x64.exe)

### Linux (Official)

⚠️ Murmure currently has limited support on Wayland-based distributions (except Fedora, which can fall back to X11 for some apps).  
This appears to be related to Wayland’s sandbox restrictions for AppImages, the global shortcut to start recording will not work in this environment.  
No workaround is available yet. See #28

1. Download Murmure_amd64.AppImage from [release](https://github.com/Kieirra/murmure/releases) page
2. Make it executable: `chmod +x Murmure_amd64.AppImage`
3. Run the AppImage.

### MacOS (Official)

⚠️ MacOS may show security warnings because Murmure **isn’t signed with a paid Apple certificate**. These warnings are expected for independent apps, and Murmure is safe to install.

🛡️ We guarantee the installer is safe, contains **no malware**, and you can verify the source code or even compile it yourself if you prefer.

1. Download Murmure_aarch64_darwin.dmg from the [release](https://github.com/Kieirra/murmure/releases) page
2. Open the DMG. If macOS blocks it, go to System Settings → Privacy & Security and click "Open Anyway".
3. Drag Murmure to the Applications folder, then open it from there.
4. If you see an "app is damaged" message, click Cancel, run `xattr -cr /Applications/Murmure.app` in Terminal, then reopen Murmure.

### MacOS - Intel (Official) - Experimental

⚠️ MacOS may show security warnings because Murmure **isn’t signed with a paid Apple certificate**. These warnings are expected for independent apps, and Murmure is safe to install.

🛡️ We guarantee the installer is safe, contains **no malware**, and you can verify the source code or even compile it yourself if you prefer.

1. Download Murmure_aarch64_darwin.dmg from the [release](https://github.com/Kieirra/murmure/releases) page
2. Open the DMG. If macOS blocks it, go to System Settings → Privacy & Security and click "Open Anyway".
3. Drag Murmure to the Applications folder, then open it from there.
4. If you see an "app is damaged" message, click Cancel, run `xattr -cr /Applications/Murmure.app` in Terminal, then reopen Murmure.

P.S. : This version is experimental and upgrading to the latest version need to be done manually.

## Usage

Murmure provides a clean and focused speech-to-text experience.
Once launched, simply start recording your voice. The text appears instantly, processed directly on your computer.

Typical use cases include:

- Dictating to any AI prompt (Cursor, ChatGPT, Mistral, Claude code, etc.)
- Writing notes hands-free
- Capturing creative ideas or dictation
- Post processing with a local LLM to translate, fix grammar, etc.

Because all computation is local, no network connection is required.

## Technology

Murmure uses the mode l[Parakeet TDT 0.6B v3](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3), a highly optimized, experimental transformer-based speech recognition model designed for low-latency, on-device inference. It combines fast transcription with strong accuracy across multiple languages, running efficiently on consumer GPUs or CPUs.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## 🗺️ Roadmap

- [x] (1.7.0) feat(settings): Allow selecting the input microphone, thanks to @litel-fr
- [x] (1.7.0) feat(settings): Add configurable log verbosity levels (trace, debug, info, warn, error)
- [x] (1.7.0) feat(dictionary): import/export words from dictionary (medical preset and other) thanks to @icristescu [#72](https://github.com/Kieirra/murmure/pull/72)
- [x] (1.7.0) feat(command): allow to select text and modify it with a custom command (eg. fix grammar, translate to English, etc.)
- [x] (1.7.0) feat(transcription): add text insertion mode setting (standard, terminal-compatible, or direct typing) [#119](https://github.com/Kieirra/murmure/issues/119)
- [x] (1.7.0) feat(llm): Add support for multiple saved prompts, instead of a single customizable prompt.
- [x] (1.7.0) feat(llm): enforce prompt instructions with anchor tags and add "Cursor Developer" preset
- [x] (1.7.0) feat(llm): syntax highlighting for prompt editor
- [x] (1.7.0) fix(llm): fix full screen issue on Select Model page on macOS - https://github.com/Kieirra/murmure/issues/82
- [x] (1.7.0) feat(logs): Add a button to open the logs folder & improve logs printing
- [x] (1.7.0) fix(privacy): transcription should not be part of the logs by default [#88](https://github.com/Kieirra/murmure/issues/88)
- [x] (1.7.0) fix(privacy): temporary audio save in tmp folder and not app_dir in case of crash [#88](https://github.com/Kieirra/murmure/issues/88)
- [x] (1.7.0) fix(dictionary): Invalid word format error message
- [x] (1.7.0) fix(format): allow digit conversion threshold to be 0
- [x] (1.7.0) fix(security): update dependencies to fix security vulnerabilities
- [x] (1.7.0) refactor(sonar): fix main sonar issues
- [x] (1.7.0) ci(security): add Security Scanning with SonarQube
- [x] (1.7.0) ci(security): improve static analysis for contributions (test, compilation, linting, etc.)
- [x] (1.7.0) ci(build): nsis exe installer without administration privilege
- [x] (1.7.0) refactor(shortcuts): streamline logic and optimize event lookup
- [x] (R&D) Investigate the possibility of using MedASR (medical‑specialized ASR) in Murmure - KO see https://github.com/Kieirra/murmure/discussions/111
- [ ] feat(rules): Add regex support for custom rules https://github.com/Kieirra/murmure/issues/105
- [ ] feat(ui): Add a “?” helper in the “Replacement text” field (explain natural language input and real line breaks instead of `\n`)
- [ ] feat(formatting): Allow rule reordering https://github.com/Kieirra/murmure/issues/104
- [ ] feat(formatting): Improve rules label to make sentences https://github.com/Kieirra/murmure/issues/101
- [ ] feat(llm): Allow llm mode reordering https://github.com/Kieirra/murmure/issues/104
- [ ] feat(llm): Automatically detect Ollama at first LLM Connect tutorial.
- [ ] feat(overlay): Allow dragging the overlay to change its position https://github.com/Kieirra/murmure/issues/64
- [ ] feat(dictionary): Virtualize dictionary to handle large dictionaries
- [ ] feat(dictionary): Add an option to clear all dictionary entries
- [ ] feat(dictionary): Improve detection https://github.com/Kieirra/murmure/issues/44
- [ ] feat: Allow pinning Murmure to the dock on linux https://github.com/Kieirra/murmure/issues/64
- [ ] fix(visualizer): Adjust sensitivity (dynamic or lower)
- [ ] fix(visualizer): Visualizer does not always reset at the end of a transcription
- [ ] refactor(settings): Secure settings persistence (migrate to tauri-plugin-store for atomic writes)
- [ ] feat(shortcuts): Add a shortcut to automatically add a selected word to the dictionary (copy selection → read word → add to dictionary)
- [ ] feat(packaging): Add a `.deb` package and register it for Debian / Ubuntu / Linux Mint
- [ ] fix(shortcuts): Improve shortcut support on Linux and Windows
- [ ] fix(overlay): Overlay may freeze under certain conditions (not reproducible yet)
- [ ] fix(overlay): Prevent launching multiple Murmure instances when clicking rapidly (not reproducible yet)
- [ ] (under consideration) feat(advanced): Audio pre-prompt https://github.com/Kieirra/murmure/issues/75
- [ ] (under consideration) feat(webhook): Send an HTTP request after `CTRL + SPACE` (opens up many interesting possibilities)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

Reporting issues is done [on GitHub](https://github.com/Kieirra/murmure/issues/new).

### Privacy Policy

See [PRIVACY_POLICY.md](./PRIVACY_POLICY.md).

## Code Signing Policy

Free code signing provided by [SignPath.io](https://about.signpath.io/), certificate by [SignPath Foundation](https://signpath.org/).

| Role                   | Team Members                          |
| ---------------------- | ------------------------------------- |
| Committers & reviewers | [Kieirra](https://github.com/Kieirra) |
| Approvers              | [Kieirra](https://github.com/Kieirra) |

## Support Development

If you like Murmure and want to support its development: [Support on Tipeee](https://fr.tipeee.com/murmure-al1x-ai/)

## License

Murmure is free and open source, released under the GNU GPL v3 License.
You can inspect, modify, and redistribute it freely as long as derivative works remain open source.

## Acknowledgments

- Thanks to NVIDIA for releasing the model [Parakeet TDT 0.6B v3](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3), [Tauri](https://github.com/tauri-apps/tauri) for being an amazing tool, and to the open‑source community for their tools and libraries.
