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

### Linux (Official)

⚠️ Murmure currently has limited support on Wayland-based distributions (except Fedora, which can fall back to X11 for some apps).  
This appears to be related to Wayland’s sandbox restrictions for AppImages, the global shortcut to start recording will not work in this environment.  
No workaround is available yet. See #28

1. Download Murmure_amd64.AppImage from [release](https://github.com/Kieirra/murmure/releases) page
2. Make it executable: `chmod +x Murmure_amd64.AppImage`
3. Run the AppImage.

Murmure uses the [ALSA](https://www.alsa-project.org/wiki/Main_Page) API to
access your microphone, so if you're running Pipewire for your audio stack,
make sure that the ALSA API calls are routed through it (e.g. by installing
[the `pipewire-alsa`
package](https://archlinux.org/packages/extra/x86_64/pipewire-alsa/) on Arch
Linux), otherwise you'll have errors such as `ALSA lib
pcm_dsnoop.c:567:(snd_pcm_dsnoop_open) unable to open slave`.

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

P.S. : This version is experimental

## Usage

Murmure provides a clean and focused speech-to-text experience.
Once launched, simply start recording your voice. The text appears instantly, processed directly on your computer.

Typical use cases include:

- Dictating to any AI prompt (Cursor, ChatGPT, Mistral, etc.)
- Writing notes hands-free
- Capturing creative ideas or dictation

Because all computation is local, no network connection is required.

## Technology

Murmure uses the mode l[Parakeet TDT 0.6B v3](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3), a highly optimized, experimental transformer-based speech recognition model designed for low-latency, on-device inference. It combines fast transcription with strong accuracy across multiple languages, running efficiently on consumer GPUs or CPUs.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## 🗺️ Roadmap 
- [x] (1.7.0) feat(settings): Allow selecting the input microphone, thanks to @litel-fr
- [x] (1.7.0) feat(settings): Add configurable log verbosity levels (trace, debug, info, warn, error)
- [x] (1.7.0) feat(dictionary): import/export words from dictionary (medical preset and other)n thanks to @icristescu [#72](https://github.com/Kieirra/murmure/pull/72)
- [x] (1.7.0) fix(llm): fix full screen issue on Select Model page on macOS - https://github.com/Kieirra/murmure/issues/82
- [x] (1.7.0) fix(privacy): transcription should not be part of the logs by default [#88](https://github.com/Kieirra/murmure/issues/88)
- [x] (1.7.0) fix(privacy): temporary audio save in tmp folder and not app_dir in case of crash [#88](https://github.com/Kieirra/murmure/issues/88)
- [x] (1.7.0) fix(dictionary): Invalid word format error message
- [x] (1.7.0) refactor(sonar): fix main sonar issues
- [x] (1.7.0) ci(security): add Security Scanning with SonarQube
- [x] (1.7.0) ci(security): improve static analysis for contributions (test, compilation, linting, etc.)
- [ ] (1.7.0) feat(command): allow to select text and modify it with a custom command (eg. fix grammar, translate to English, etc.)
- [ ] (1.7.0) feat(llm) : Clarify in the documentation what the “dictionary” refers to (to avoid ambiguity with an official language dictionary).
- [ ] (1.7.0) feat(llm): Add support for multiple saved prompts, instead of a single customizable prompt.
- [ ] feat(llm): allow internal server host with vLLM for organization
- [ ] fix(overlay): Bug with freezing overlay ? (not reproduct yet)
- [ ] fix(overlay): prevent launching multiple Murmure App instances when clicked rapidly
- [ ] feat(overlay): be able to drag-n-drop the overlay to change position https://github.com/Kieirra/murmure/issues/64
- [ ] feat: be able to pin Murmure to the dock https://github.com/Kieirra/murmure/issues/64 
- [ ] fix(visualizer): dynamic or lower sensibility 
- [ ] fix(visualizer): the visualizer does not always reset at the end of a transcription  
- [ ] (under consideration) feat(advanced): audio pre-prompt https://github.com/Kieirra/murmure/issues/75
- [ ] refactor: Secure Settings Persistence (Migrate to tauri-plugin-store for atomic writes)
- [ ] poc: Portable version without installer (& admin password) for hospital
- [ ] feat(shortcut): Add a shortcut to add automatically a word in dictionnary after selecting it (copy selection > read word > add it to dictionary)
- [ ] fix(shortcuts): Improve available shortcuts on Linux & Windows
- [ ] feat: add .deb file and register it on debian/ubuntu/"linux mint" package manager
- [ ] (under consideration) feat(webhook): Send an HTTP request after `CTRL + SPACE`, opens up many interesting possibilities
- [ ] **Major:** Implement a plugin system  
- [ ] **Major(under consideration):** Drag & drop support for audio files (MP3, WAV) with automatic transcription (and maybe speaker diarization)
- [ ] **Major(under consideration):** Add real-time streaming (POC)


## Acknowledgments

- Thanks to NVIDIA for releasing the model [Parakeet TDT 0.6B v3](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3), [Tauri](https://github.com/tauri-apps/tauri) for being an amazing tool, and to the open‑source community for their tools and libraries.

## License

Murmure is free and open source, released under the GNU GPL v3 License.
You can inspect, modify, and redistribute it freely as long as derivative works remain open source.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

Reporting issues is done [on GitHub](https://github.com/Kieirra/murmure/issues/new).

## Support Development

If you like Murmure and want to support its development: [Support on Tipeee](https://fr.tipeee.com/murmure-al1x-ai/)
