# Murmure

A privacy-first, open-source speech-to-text application that runs entirely on your machine, powered by a neural network via NVIDIA’s [Parakeet TDT 0.6B v3 model](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3) for fast, local transcription. Murmure turns your voice into text with no internet connection and zero data collection, and supports 25 European languages.

Learn more on the [official website](https://murmure.al1x-ai.com/) | [Documentation](https://docs.murmure.app)

![demo](public/murmure-screenshot-beautiful.png)

## Table of Contents

- [Features](#features)
- [Usage](#usage)
- [Installation](#installation)
    - [Windows (Official)](#windows-official)
    - [Linux (Official)](#linux-official)
    - [MacOS (Official)](#macos-official)
    - [MacOS - Intel (Official)](#macos---intel-official)
- [CLI Import (1.8.0)](#cli-import-180)
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

> [!IMPORTANT]
> Murmure requires **Windows 10 or later**. Older versions (such as Windows 8.1) are not supported and may result in missing system libraries (e.g. `dxcore.dll`).

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

Multiple installation methods are available:

- Quick install via terminal (Debian-based distributions):

    ```sh
    curl -fsSL https://raw.githubusercontent.com/Kieirra/murmure/main/install.sh | sh
    ```

- Using a `.deb` file (Debian-based distributions):
    1. Go to the [release](https://github.com/Kieirra/murmure/releases) page and download the latest `Murmure_amd64.deb`.
    2. Install it: `sudo dpkg -i Murmure_amd64.deb`

- Using an AppImage:
    1. Download `Murmure_amd64.AppImage` from the [release](https://github.com/Kieirra/murmure/releases) page.
    2. Make it executable: `chmod +x Murmure_amd64.AppImage`
    3. Run the AppImage.

> [!IMPORTANT]
> Murmure currently has limited support on Wayland-based distributions (except Fedora, which can fall back to X11 for some apps).  
> This appears to be related to Wayland’s sandbox restrictions for AppImages, the global shortcut to start recording will not work in this environment.  
> No workaround is available yet. See #28

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

## CLI Import (1.8.0)

> [!NOTE]
> This feature is available starting from version **1.8.0**.

Murmure supports importing a `.murmure` configuration file via the command line, useful for sysadmin mass deployment or sharing settings across machines.

**Linux:**

```sh
murmure import config.murmure
```

**macOS:**

```sh
/Applications/murmure.app/Contents/MacOS/murmure import config.murmure
```

**Windows:**

```powershell
murmure.exe import config.murmure
```

You can also specify an import strategy (`replace` by default, or `merge` to keep existing settings and add new ones):

```sh
murmure import config.murmure --strategy merge
```

For more details, run `murmure import --help`.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## 🗺️ Roadmap

### 1.9.0

- [x] feat(virtual-mic): Smart Speech Mic - use your phone as a wireless microphone by scanning a QR code in Murmure, no installation required on the phone
- [x] style(layout): Add an Extensions section in the menu to better organize features
- [x] style(layout): Global UX/UI optimization to simplify the interface and prioritize benefits over information
- [x] docs: Official documentation for Murmure (configuration, limitations, tips, etc.) https://kieirra.github.io/murmure/
- [x] fix(sidebar): Disable mobile mode for sidebar to prevent it from disappearing on high-scaling displays
- [x] feat(overlay): Configure overlay size
- [x] style(overlay): Color-coded visualizer per recording mode (standard, LLM, command)
- [x] feat(overlay): Real-time streaming preview with configurable text width, font size and max lines
- [x] fix(overlay): Error messages disappearing too quickly and blocking subsequent recordings
- [x] fix(updater): macOS auto-update not working (wrong artifact format) https://github.com/Kieirra/murmure/issues/301
- [x] fix(shortcuts): MacOs Cancel shortcut (Escape) no longer blocks other apps when not recording https://github.com/Kieirra/murmure/issues/302
- [x] perf(bundle): Shrink Rust binary by 32% (66 MB → 45 MB) and trim unused dependencies (thanks @BorisLord) https://github.com/Kieirra/murmure/pull/310
- [x] fix(voice-mode): Wake word not detected when spoken mid-sentence without a pause
- [x] feat(linux): Support Wayland on Gnome 48+ distributions
- [ ] feat(overlay): Allow dragging the overlay to change its position https://github.com/Kieirra/murmure/issues/64
- [ ] feat(shortcuts) : do not display LLM Connect shortcut if not enabled
- [ ] feat(shortcuts): Add a shortcut to toggle Voice Mode on/off https://github.com/Kieirra/murmure/issues/279
- [ ] fix(typing): Direct mode too fast for some apps https://github.com/Kieirra/murmure/issues/285
- [ ] fix: Log time not displayed in the correct timezone
- [ ] feat(rules): Allow adding a custom name for personal formatting rules

### Backlog

- [ ] feat(shortcuts): Using delete should remove shortcuts
- [ ] fix(shortcuts): Do not allow adding duplicate shortcuts
- [ ] feat(dictionary): Virtualize dictionary to handle large dictionaries
- [ ] feat(llm): Automatically detect Ollama at first LLM Connect tutorial
- [ ] feat(dictionary): Improve detection https://github.com/Kieirra/murmure/issues/44
- [ ] fix(visualizer): Adjust sensitivity (dynamic or lower)
- [ ] fix(visualizer): Visualizer does not always reset at the end of a transcription
- [ ] refactor(settings): Secure settings persistence (migrate to tauri-plugin-store for atomic writes)
- [ ] feat(shortcuts): Add a shortcut to automatically add a selected word to the dictionary (copy selection -> read word -> add to dictionary)
- [ ] (under consideration) feat(advanced): Audio pre-prompt https://github.com/Kieirra/murmure/issues/75
- [ ] (under consideration) feat(webhook): Send an HTTP request after `CTRL + SPACE` (opens up many interesting possibilities)
- [ ] (under consideration) feat(accessibility): Real-time transcription for accessibility

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
