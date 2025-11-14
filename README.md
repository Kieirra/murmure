# Murmure

A privacy-first, open-source speech-to-text application that runs entirely on your machine, powered by a neural network via NVIDIA’s Parakeet model for fast, local transcription. Murmure turns your voice into text with no internet connection and zero data collection, and supports 25 European languages.

Learn more on the on the [official website](https://murmure.al1x-ai.com/).

![demo](public/murmure-screenshot-beautiful.png)

## Features

- **Privacy First**: All processing happens locally on your device. No data ever leaves your computer.
- **No Telemetry**: Zero tracking, zero analytics. Your data stays yours, always.
- **Open Source**: Free and open source software. Inspect, modify, and contribute.
- **Powered by Parakeet**: NVIDIA’s state-of-the-art speech recognition model runs entirely on-device for fast, low-latency transcription.

## Supported Languages:

Bulgarian (bg), Croatian (hr), Czech (cs), Danish (da), Dutch (nl), English (en), Estonian (et), Finnish (fi), French (fr), German (de), Greek (el), Hungarian (hu), Italian (it), Latvian (lv), Lithuanian (lt), Maltese (mt), Polish (pl), Portuguese (pt), Romanian (ro), Slovak (sk), Slovenian (sl), Spanish (es), Swedish (sv), Russian (ru), Ukrainian (uk)

## Installation

### Windows (Official)

⚠️ Windows SmartScreen : This installer is **not signed with a commercial certificate** (which costs ~€200–€500/year).  
If you downloaded it from our **official GitHub releases**, you can safely continue.

🛡️ We guarantee the installer is safe, contains **no malware**, and you can verify the source code or even compile it yourself if you prefer.

1. Download murmure\_{version}\_x64_en-US.msi from the [release](https://github.com/Kieirra/murmure/releases) page
2. Run the installer and follow the setup wizard.

### Linux (Official)

⚠️ Murmure currently has limited support on Wayland-based distributions (except Fedora, which can fall back to X11 for some apps).  
This appears to be related to Wayland’s sandbox restrictions for AppImages, the global shortcut to start recording will not work in this environment.  
No workaround is available yet. See #28

1. Download murmure\_{version}\_amd64.AppImage from [release](https://github.com/Kieirra/murmure/releases) page
2. Make it executable: `chmod +x murmure-x86_64.AppImage`
3. Run the AppImage.

Murmure uses the [ALSA](https://www.alsa-project.org/wiki/Main_Page) API to
access your microphone, so if you're running Pipewire for your audio stack,
make sure that the ALSA API calls are routed through it (e.g. by installing
[the `pipewire-alsa`
package](https://archlinux.org/packages/extra/x86_64/pipewire-alsa/) on Arch
Linux), otherwise you'll have errors such as `ALSA lib
pcm_dsnoop.c:567:(snd_pcm_dsnoop_open) unable to open slave`.

### MacOS (Official)

⚠️ Just like Windows, this installer is **not signed with an Apple developer account** (which costs ~€100/year).

Therefore you might get a "This app is damaged and can't be opened. You should move it to the Bin" error.

🛡️ Again, we guarantee the installer is safe, contains no malware, and you can verify the source code or even compile it yourself if you prefer.

- Download Murmure.dmg from [release](https://github.com/Kieirra/murmure/releases) page
- Try to open the DMG file. You might get an error saying: "Apple could not verify Murmure.dmg is free from malware that may harm your Mac or compromise your privacy."

![dmg error](public/murmure-dmg-error.png)

- Click on "Done"
- Go to Privacy & Security, scroll to "Security" section and click on "Open Anyway".

![dmg open anyway](public/murmure-dmg-open-anyway.png)

- Then click on "Open Anyway"

![dmg open anyway confirm](public/murmure-dmg-open-anyway-confirm.png)

- Once the app is opened, you can drag it to your Applications folder.
- Once the app is in the Applications ‡older, open this folder and double-click on murmure to launch it.
- It might display an other error message

![dmg app damaged error](public/murmure-dmg-app-damaged.png)

- click on "Cancel"
- Open the terminal and run `xattr -cr /Applications/murmure.app` it will tell mac that the app is safe to run.
- Go back to the Applications folder and double-click on murmure to launch it.
- It should now work.

🎤 Microphone Access

If you want to use the microphone, you need to grant access to the app. The app should ask for permission when you start recording. If it doesn't, you can grant access manually:

1. Open the System Preferences
2. Go to Privacy & Security
3. Click on Microphone
4. Enable "murmure"

🔑 Accessibility Access

To use the keyboard shortcuts & clipboard, you need to grant access to the app. The app should ask for permission when you start recording. If it doesn't, you can grant access manually:

1. Open the System Preferences
2. Go to Accessibility
3. If "murmure" is not listed, click on the "+" button to add it.
4. Enable "murmure"

#### Arch Linux (Community)

⚠️ Community builds are maintained by kind contributors on a best-effort basis.
They do their best to keep them up to date, but there’s no guarantee they will always be.
If you encounter a bug with one of these packages, please open an issue in the corresponding community repository instead.

Community repository: https://github.com/Horgix/aur-package_murmure_mirror

On Arch Linux, you can install [the `murmure` package directly from the
AUR](https://aur.archlinux.org/packages/murmure) using your favorite helper:

```sh
aura -A murmure
# Or
yay -S murmure
# Or
paru -S murmure
```

## Usage

Murmure provides a clean and focused speech-to-text experience.
Once launched, simply start recording your voice. The text appears instantly, processed directly on your computer.

Typical use cases include:

- Dictating to any AI prompt (Cursor, ChatGPT, Mistral, etc.)
- Writing notes hands-free
- Capturing creative ideas or dictation

Because all computation is local, no network connection is required.

## Technology

Murmure uses NVIDIA’s Parakeet TDT, a highly optimized, experimental transformer-based speech recognition model designed for low-latency, on-device inference. It combines fast transcription with strong accuracy across multiple languages, running efficiently on consumer GPUs or CPUs.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## 🗺️ Roadmap

- [x] (1.5.0) feat(stats): display words per minute, MB saved (no cloud), and total words
- [x] (1.5.0) feat(platform): MacOS version
- [x] (1.5.0) feat(history): Option to not make the last transcription persistent in history and remove it at each shutdown
- [x] (1.5.0) feat(i18n): FR/EN translation of the application
- [x] (1.5.0) fix(ui): prevent menu from scrolling on long pages
- [x] (1.5.0) fix(updates): restart application after uploading to new version
- [x] (1.5.0) fix(overlay): Display the overlay on the active screen
- [x] (1.5.0) fix(overlay): Scale overlay based on screen DPI or add a zoom option
- [x] (1.5.0) feat(onboarding): Add onboarding
- [ ] fix(visualizer): the visualizer does not always reset at the end of a transcription
- [ ] feat(settings): Allow selecting the input microphone
- [ ] feat(shortcuts): Add keyboard shortcuts to start and stop recording (outside push-to-talk mode)
- [ ] feat(webhook): Send an HTTP request after `CTRL + SPACE`, opens up many interesting possibilities
- [ ] feat(overlay): Add a size option
- [ ] feat(overlay): Add a song option when starting recording
- [ ] refactor(dictionary): Improve the custom dictionary algorithm (performance and algo)
- [ ] feat(dictionary): import/export words from dictionary
- [ ] feat(dictionary): choose language for each word
- [ ] fix(shortcuts): Improve available shortcuts on Linux
- [ ] feat: Allow uploading an audio file and outputting a `.txt` transcript
- [ ] feat: Create an API to fetch the latest transcription
- [ ] feat: Support MP3 files in addition to WAV (since some recorders use MP3)
- [ ] **Major:** Add real-time streaming (POC)
- [ ] **Major:** Integrate an LLM to enhance or modify transcriptions (post-processing)
- [ ] **Major:** Implement a plugin system

## Acknowledgments

- Thanks to NVIDIA for the Parakeet TDT model, Tauri for being an amazing tool, and to the open‑source community for their tools and libraries.

## License

Murmure is free and open source, released under the GNU GPL v3 License.
You can inspect, modify, and redistribute it freely as long as derivative works remain open source.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

Reporting issues is done [on GitHub](https://github.com/Kieirra/murmure/issues/new).

## Support Development

If you like Murmure and want to support its development: [Support on Tipeee](https://fr.tipeee.com/murmure-al1x-ai/)
