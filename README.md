# Murmure

A privacy-first, open-source speech-to-text application that runs entirely on your machine, powered by a neural network via NVIDIA’s [Parakeet TDT 0.6B v3 model](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3) for fast, local transcription. Murmure turns your voice into text with no internet connection and zero data collection, and supports 25 European languages.

Learn more on the [official website](https://murmure.al1x-ai.com/) | [Documentation](https://docs.murmure.app)

![demo](public/murmure-demo.webp)

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

> [!NOTE]
> **Wayland**: Two shortcut modes are available. KDE Plasma 6, Hyprland, and Sway use the `xdg-desktop-portal` GlobalShortcuts portal with no manual setup. GNOME defaults to CLI mode: you must configure a custom OS shortcut before using Murmure, and Push-to-talk is not available. See the [Linux installation guide](https://docs.murmure.app/getting-started/linux/) and the [shortcut configuration guide](https://docs.murmure.app/configure-shortcuts-on-linux/).

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

The same upgrade note from 1.6.0 applies. See the MacOS section above.

## CLI Import (1.8.0)

Murmure supports importing a `.murmure` configuration file via the command line (`murmure import config.murmure`), useful for mass deployment or sharing settings across machines. A `--strategy merge` option is available to keep existing settings. See the [CLI documentation](https://docs.murmure.app/features/cli/) for details.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## 🗺️ Roadmap
- [x] fix(packaging): Set the Linux desktop file category so Murmure appears under Utility instead of Other https://github.com/Kieirra/murmure/issues/385
- [x] feat(shortcuts): Support Pause and ScrollLock as shortcut keys on Windows and Linux https://github.com/Kieirra/murmure/issues/387
- [x] feat(shortcuts): Capture shortcut keys from the native backend so any detectable key can be recorded (fixes F13 shown as Unidentified and layout-dependent letter labels on X11) https://github.com/Kieirra/murmure/issues/388
- [x] fix(wake-word): Stop restarting the listener in a tight loop when the microphone is unavailable, which was flooding the log file https://github.com/Kieirra/murmure/discussions/381
- [x] fix(audio): Adapt VAD to microphone gain and background noise for wake word, chunking, and silence auto-stop https://github.com/Kieirra/murmure/discussions/381
- [x] feat(dictionary): Add a word counter with a green/yellow/red indicator and consider a soft limit https://github.com/Kieirra/murmure/issues/386
- [x] feat(dictionary): Support all characters in Parakeet's vocabulary (lift the letters-only frontend validation) and accept two-word entries, so expressions with a space or a hyphen can be added https://github.com/Kieirra/murmure/issues/386
- [x] feat(llm): Add a Transform shortcut per LLM mode (`Ctrl+Alt+Shift+1` to `Ctrl+Alt+Shift+4`) that applies the mode's saved prompt to the selected text, with no dictation, alongside Dictate and Command
- [x] fix(llm): Stop sending the temperature parameter to remote servers, OpenAI GPT-5 models only accept the default value and returned 400 Bad Request https://github.com/Kieirra/murmure/issues/397#issuecomment-5130959650
- [x] feat(shortcuts): Allow binding the fn / globe key as a shortcut on macOS (thank you @gregoryrivage) https://github.com/Kieirra/murmure/pull/399
- [x] fix(insert): Keep a copied image in the clipboard when dictating, instead of replacing it with empty text
- [x] feat(dictionary): Accept .txt import and document the expected format (one word per line) https://github.com/Kieirra/murmure/issues/386
- [x] fix(insert): Type accented characters natively in Direct mode on Wayland (extend the XKB char map to real layout keys, AltGr and dead keys) instead of ASCII folding https://github.com/Kieirra/murmure/issues/384
- [x] fix(llm): Allow typing a custom model name and stop the connection test from locking the Remote provider, so OpenAI compatible servers without a /models endpoint (such as the Claude API) can be used https://github.com/Kieirra/murmure/issues/397
- [x] feat(audio): Lower output volume while recording https://github.com/Kieirra/murmure/issues/364
- [x] fix(api): Route /api/transcribe through the same chunked transcription path as the keyboard and the CLI, so audio of any length works and the only limit left is the 100 MB request size, not the audio duration https://github.com/Kieirra/murmure/issues/401
- [x] feat(api): Cancel the running transcription when the client closes the HTTP connection, and free the queue as soon as the abandoned work has actually stopped https://github.com/Kieirra/murmure/discussions/411
- [x] fix(audio): Pad each chunk with silence before inference, to fix chunks the model cannot decode, which silently truncated the transcription
- [x] feat(packaging): Add a pacman package for Arch-based distros (CachyOS) to the release, and point the update button to the releases page for those installs, as the Tauri updater has no pacman installer https://github.com/Kieirra/murmure/issues/358#issuecomment-4811232712
- [x] feat(cli): Add a --hidden flag to launch Murmure without showing the window, combined with control commands such as --transcription or --llm-mode (replaces the internal --autostart workaround)
- [x] fix(logs): Check the log file size while the app is running and reset it above 1 MB, instead of only checking it at startup, and add an Off level to disable logging completely https://github.com/Kieirra/murmure/issues/417
- [x] fix(wake-word): Stop the microphone icon from flickering every 10 seconds during silence, the stream watchdog now gets a heartbeat from the audio callback so a silent stream is no longer presumed dead (thank you @nicob3y) https://github.com/Kieirra/murmure/issues/422
- [x] fix(audio): List Bluetooth microphones on Linux, the pactl enumeration required `device.class = "sound"` which PipeWire only sets on ALSA sources, so every `bluez_input` source was dropped https://github.com/Kieirra/murmure/issues/421
- [x] feat(audio): Make the start and stop sounds louder and add a volume slider, the two source files peak at -21 dBFS which left them barely audible https://github.com/Kieirra/murmure/issues/426
- [x] fix(llm): Wait up to 2 seconds for the modifiers to be released before copying the selection, keeping Ctrl+Alt+Shift held turned the synthetic Ctrl+C into Ctrl+Alt+Shift+C, so Transform reported an empty selection https://github.com/Kieirra/murmure/discussions/419#discussioncomment-18114383
- [x] fix(audio): Keep the output stream alive while a recording is running, past one minute of dictation it closed on the idle timeout and the stop sound played on a device that had gone back to sleep https://github.com/Kieirra/murmure/discussions/419#discussioncomment-18107863

### Backlog
- [ ] feat(command): Show transformed text in a preview popup (e.g., for translating selected sentences in non-editable text, such as software or websites)
- [ ] fix(api): Remove the experimental tag and consolidate the API
- [ ] fix(api): Implement LLM Connect service
- [ ] feat(llm): Built-in prompt preset for input anonymisation
- [ ] feat(updater): Opt-in setting to subscribe to beta (pre-release) updates
- [ ] feat(cli): Add a --quit flag to close Murmure completely from the command line, the tray menu is the only way today so tiling WM users have no keyboard path https://github.com/Kieirra/murmure/issues/429
- [ ] (under consideration) feat(draft): Draft Mode to review and edit a transcription before writing (medical use case)
- [ ] (under consideration) fix(packaging): Authenticode-sign the inner murmure.exe on Windows, not only the installers, so the app runs under Smart App Control (needs a second SignPath submission before bundling) https://github.com/Kieirra/murmure/issues/428

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
