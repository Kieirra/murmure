 # Murmure

A privacy-first, open-source speech-to-text application that runs entirely on your machine, powered by a neural network via NVIDIA’s [Parakeet TDT 0.6B v3 model](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3) for fast, local transcription. Murmure turns your voice into text with no internet connection and zero data collection, and supports 25 European languages.

Learn more on the [official website](https://murmure.al1x-ai.com/).

![demo](public/murmure-screenshot-beautiful.png)

## Table of Contents

- [Features](#features)
- [Usage](#usage)
- [Installation](#installation)
  - [Windows (Official)](#windows-official)
  - [Linux (Official)](#linux-official)
  - [MacOS (Official)](#macos-official)
  - [MacOS - Intel (Official)](#macos---intel-official)
- [Changelog](#changelog)
- [🗺️ Roadmap](#️-roadmap)
- [Contributing](#contributing)
- [Privacy Policy](#privacy-policy)
- [Sponsors](#sponsors)
- [Support Development](#support-development)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Features

- **Privacy First**: All processing happens locally on your device. No data ever leaves your computer.
- **No Telemetry**: Zero tracking, zero analytics. Your data stays yours, always.
- **Open Source**: Free and open source software. Inspect, modify, and contribute.
- **Powered by [Parakeet TDT 0.6B v3](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3)**: NVIDIA’s latest state-of-the-art speech recognition model runs entirely on-device for fast, low-latency transcription.
- **Multilingual**: Supports 25 languages!

<details>
<summary>List of supported languages</summary>
Bulgarian (bg), Croatian (hr), Czech (cs), Danish (da), Dutch (nl), English (en), Estonian (et), Finnish (fi), French (fr), German (de), Greek (el), Hungarian (hu), Italian (it), Latvian (lv), Lithuanian (lt), Maltese (mt), Polish (pl), Portuguese (pt), Romanian (ro), Slovak (sk), Slovenian (sl), Spanish (es), Swedish (sv), Russian (ru), Ukrainian (uk)
</details>

## Usage

Murmure provides a clean and focused speech-to-text experience.
Once launched, simply start recording your voice. The text appears instantly, processed directly on your computer.

Typical use cases include:
- Dictating to any AI prompt (Cursor, ChatGPT, Mistral, Claude code, etc.)
- Writing notes hands-free
- Capturing creative ideas or dictation
- Post processing with a local LLM to translate, fix grammar, etc.

Because all computation is local, no network connection is required.

## Installation

### Windows (Official)

Multiple installation methods are available:

- Using a `.msi` or `setup.exe` file:
    1. Go to the [release](https://github.com/Kieirra/murmure/releases) page and download the latest Murmure_x64.msi (or Murmure_x64-setup.exe).
    2. Run the installer and follow the setup wizard.

- Via WinGet:
    1. Open the `Console` app via the Windows start menu.
    2. Inside the console, paste `winget install Kieirra.Murmure` and follow the instructions. (`--scope user` will be available in the future)

> [!IMPORTANT]
> Murmure requires the [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist) to work on Windows. This package is present on most computers, but if you encounter the error message `The code execution cannot proceed because MSVCP140.dll was not found. Reinstalling the program may fix this problem.`, download and install the package from the official page or use this direct download link: [https://aka.ms/vc14/vc_redist.x64.exe](https://aka.ms/vc14/vc_redist.x64.exe)

> ⚠️ Antivirus Notice : Some users reported that Kaspersky may block Murmure. If needed, please add Murmure as an exclusion in your antivirus settings.

### Linux (Official)

1. Download Murmure_amd64.AppImage from [release](https://github.com/Kieirra/murmure/releases) page
2. Make it executable: `chmod +x Murmure_amd64.AppImage`
3. Run the AppImage.

> [!IMPORTANT]
> Murmure currently has limited support on Wayland-based distributions (except Fedora, which can fall back to X11 for some apps).  
This appears to be related to Wayland’s sandbox restrictions for AppImages, the global shortcut to start recording will not work in this environment.  
No workaround is available yet. See #28

### MacOS (Official)

1. Download **Murmure_aarch64_darwin.dmg** from the [release](https://github.com/Kieirra/murmure/releases) page
2. Drag Murmure to the Applications folder, then open it from there.
3. Murmure should ask for permissions to access your microphone and accessibility.
4. Restart Murmure for the permissions to take effect.

> [!IMPORTANT]
> **Updating Murmure on macOS from 1.6.0:** If you experience issues with Murmure and the shortcuts are not working, please do this exactly in this order, (and "Remove" means not only un-toggling but really removing completely Murmure from the list) :
1. Remove Murmure from System Settings → Privacy & Security → Accessibility.
2. Remove Murmure from System Settings → Privacy & Security → Input monitoring.
3. Install the last version
4. Launch Murmure.
5. Re-grant the Accessibility
6. Re-grant the Input monitoring permission
7. Restart Murmure.

it should work. It's a bit painful but you will not do it again with the next version, it's because 1.6.0 have the same name but is not detected as the same application... so macos is lost.

### MacOS - Intel (Official)

1. Download **Murmure_x86_64_darwin.dmg** from the [release](https://github.com/Kieirra/murmure/releases) page
2. Drag Murmure to the Applications folder, then open it from there.
3. Murmure should ask for permissions to access your microphone and accessibility.
4. Restart Murmure for the permissions to take effect.

> [!IMPORTANT]
> **Updating Murmure on macOS from 1.6.0:** If you experience issues with Murmure and the shortcuts are not working, please do this exactly in this order, (and "Remove" means not only un-toggling but really removing completely Murmure from the list) :
1. Remove Murmure from System Settings → Privacy & Security → Accessibility.
2. Remove Murmure from System Settings → Privacy & Security → Input monitoring.
3. Install the last version
4. Launch Murmure.
5. Re-grant the Accessibility
6. Re-grant the Input monitoring permission
7. Restart Murmure.

it should work. It's a bit painful but you will not do it again with the next version, it's because 1.6.0 have the same name but is not detected as the same application... so macos is lost.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## 🗺️ Roadmap
- [x] (1.8.0) **MAJOR** feat(voice-mode): Add voice activation (voice mode) to trigger Murmure without keyboard shortcuts https://github.com/Kieirra/murmure/pull/178 https://github.com/Kieirra/murmure/issues/171
- [x] (1.8.0) **MAJOR** feat(llm): Add remote server support for LLM Connect (OpenAI-compatible API, per-mode provider switching, onboarding flow)
- [x] (1.8.0) feat(voice-mode): Add auto-send enter with voice mode (https://github.com/Kieirra/murmure/pull/156)
- [x] (1.8.0) feat(voice-mode): Wake word listener stays active: validate/cancel words work during keyboard-triggered recording
- [x] (1.8.0) feat(shortcuts): Add support for mouse buttons (https://github.com/Kieirra/murmure/pull/158)
- [x] (1.8.0) feat(dictionary): Add an option to clear all dictionary entries (https://github.com/Kieirra/murmure/pull/159)
- [x] (1.8.0) feat(shortcuts): Add a shortcut in overlay to cancel recording https://github.com/Kieirra/murmure/pull/161
- [x] (1.8.0) feat(rules): Add regex support for custom rules https://github.com/Kieirra/murmure/pull/163 https://github.com/Kieirra/murmure/issues/105
- [x] (1.8.0) feat(rules): Improve rules label to make sentences https://github.com/Kieirra/murmure/pull/163 https://github.com/Kieirra/murmure/issues/101
- [x] (1.8.0) feat(rules): Add a “?” helper in the “Replacement text” field (explain natural language input and real line breaks instead of `\n`) https://github.com/Kieirra/murmure/pull/163
- [x] (1.8.0) feat(rules): Short text correction — auto-lowercase and remove trailing punctuation for short transcriptions (1-2 words)
- [x] (1.8.0) feat(rules): Allow rule reordering https://github.com/Kieirra/murmure/pull/170
- [x] (1.8.0) fix(overlay): remove scrollbar on some configurations on windows https://github.com/Kieirra/murmure/pull/169 
- [x] (1.8.0) feat(ci): Add a `.deb` package and `install.sh` one-liner script for Debian/Ubuntu/Linux Mint
- [x] (1.8.0) feat(update): Update `tauri-plugin-updater` to 2.10.0 for multi-format support (.deb, .exe, macOS Intel)
- [x] (1.8.0) fix(update): Fix double percentage display during download
- [x] (1.8.0) feat(sidebar): Add "Release notes" link in sidebar footer
- [x] (1.8.0) fix(overlay): Prevent overlay from stealing window focus on show
- [x] (1.8.0) feat(llm): Allow llm mode reordering https://github.com/Kieirra/murmure/issues/104
- [x] (1.8.0) feat(settings): Improve default settings for new user (transcription in RAM, trailing space after each transcription)
- [x] (1.8.0) feat(shortcuts): Add support for <>, ², F13-F24 https://github.com/Kieirra/murmure/pull/194 https://github.com/Kieirra/murmure/issues/189
- [x] (1.8.0) feat(about): Improve About page UI https://github.com/Kieirra/murmure/pull/198
- [x] (1.8.0) fix(ui): Use dark mode colors consistently
- [x] (1.8.0) feat(autostart): start minimized to tray on autostart https://github.com/Kieirra/murmure/pull/201
- [ ] (1.8.0) fix(settings): improve microphone list and keep manually selected device when disconnected
- [ ] (1.8.0) feat(overlay): Configure overlay size
- [ ] feat(shortcuts): using delete should remove shortcuts
- [ ] fix(shortcuts): Do not allow adding duplicate shortcuts
- [ ] feat(dictionary): Virtualize dictionary to handle large dictionaries
- [ ] feat(llm): Automatically detect Ollama at first LLM Connect tutorial.
- [ ] feat(overlay): Allow dragging the overlay to change its position https://github.com/Kieirra/murmure/issues/64
- [ ] feat(dictionary): Improve detection https://github.com/Kieirra/murmure/issues/44
- [ ] fix(visualizer): Adjust sensitivity (dynamic or lower)
- [ ] fix(visualizer): Visualizer does not always reset at the end of a transcription
- [ ] refactor(settings): Secure settings persistence (migrate to tauri-plugin-store for atomic writes)
- [ ] feat(shortcuts): Add a shortcut to automatically add a selected word to the dictionary (copy selection → read word → add to dictionary)
- [ ] (under consideration) feat(advanced): Audio pre-prompt https://github.com/Kieirra/murmure/issues/75
- [ ] (under consideration) feat(webhook): Send an HTTP request after `CTRL + SPACE` (opens up many interesting possibilities)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

Reporting issues is done [on GitHub](https://github.com/Kieirra/murmure/issues/new).

### Privacy Policy

See [PRIVACY_POLICY.md](./PRIVACY_POLICY.md).

## Sponsors

<table>
  <tr>
    <td><img src="https://signpath.org/assets/favicon-50x50.png" alt="SignPath" width="40"></td>
    <td>Free code signing on Windows provided by <a href="https://about.signpath.io/">SignPath.io</a>, certificate by <a href="https://signpath.org/">SignPath Foundation</a></td>
  </tr>
</table>

## Support Development

If you like Murmure and want to support its development: [Support on Tipeee](https://fr.tipeee.com/murmure-al1x-ai/)

## License

Murmure is free and open source, released under the GNU AGPL v3 License.
You can inspect, modify, and redistribute it freely as long as derivative works remain open source.

## Acknowledgments

- Thanks to NVIDIA for releasing the model [Parakeet TDT 0.6B v3](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3)
- [Tauri](https://github.com/tauri-apps/tauri) for being an amazing tool
- The open‑source community for their tools and libraries.
