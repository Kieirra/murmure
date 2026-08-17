# Beta Testing

Thank you for joining the Murmure beta program! Your feedback is invaluable to make the application rock-solid before its official release.

## How to Get the Beta

Beta builds are published before each release. Head over to the [GitHub Releases](https://github.com/Kieirra/murmure/releases) page and download the latest pre-release version.

## What's New in 1.11.0

### LLM Connect

- **Transform**: select text in any application, press `Ctrl+Alt+Shift+1` to `Ctrl+Alt+Shift+4`, and the mode's saved prompt runs on your selection, with no dictation at all
- Remote providers: you can now type a model name by hand, and a failed connection test no longer locks the provider, so servers without a `/models` endpoint (such as the Claude API) work
- The temperature parameter is no longer sent to remote servers, which fixes the 400 Bad Request returned by OpenAI GPT-5 models

### Dictionary

- Light redesign, with a word count indicator: green up to 50 words, yellow from 51 to 100, red above 100
- Two-word entries are now accepted, so expressions with a space or a hyphen can be added
- All characters from Parakeet's vocabulary are allowed, not only letters
- Export your dictionary as a `.txt` file, and import one with a documented format

### Shortcuts

- Pause and ScrollLock can be used as shortcut keys on Windows and Linux
- The fn key, marked with a globe icon on recent Macs, can now be bound on macOS
- Keys are read from the native backend, so `F13` is no longer shown as "Unidentified" and letter labels match your real keyboard layout on X11

### Audio

- The system volume can go down while you record, so you hear yourself better
- Microphone level detection adapts to your gain and to background noise
- Each chunk is padded with silence before transcription, which fixes results that were silently cut

### Linux

- New pacman package for Arch based distributions, including CachyOS
- Wayland: accented characters are typed natively in direct insert mode
- Murmure now appears under Utility in application menus

### API and CLI

- The local API accepts audio of any length, as long as the request stays under 100 MB, and stops transcribing as soon as the client disconnects
- New `--hidden` flag to start Murmure without showing the window

### Other

- Logs no longer grow forever: the file is reset when it goes above 1 MB while the app runs, and a new Off level disables logging completely
- A copied image stays in your clipboard when you dictate, instead of being replaced
- The wake word listener no longer restarts in a loop when the microphone is unavailable

## Test Plan

Do what you can, even one box helps. Start with the four essentials, they take about five minutes.

### The essentials

- [ ] Dictate a sentence like you normally do, and check the text lands correctly
- [ ] Select text in any app, press `Ctrl+Alt+Shift+1`, and check the prompt runs on your selection (set up mode 1 in LLM Connect first if you never did)
- [ ] Add a two-word entry to your dictionary, like a first and last name, then dictate it
- [ ] Turn on the volume reduction in Settings > System, play some music, then dictate

### If you have more time

- [ ] Change one of your shortcuts in Settings, then use it
- [ ] Copy an image, then dictate, and check the image is still in your clipboard
- [ ] Dictate something long, over a minute, and check nothing is missing at the end
- [ ] Open the dictionary and check the word count indicator (green up to 50 words, yellow from 51 to 100, red above 100)

### Only the line matching your setup

- [ ] macOS: bind the fn key (the one with the globe icon) to a shortcut, then use it
- [ ] Linux Wayland: dictate a sentence with accented characters in direct insert mode
- [ ] Arch or CachyOS: install the `.pkg.tar.zst` package and start the app

## Reporting Bugs

No need to open a GitHub issue, just reply in the beta announcement conversation. Tell us what broke and on which OS, that is already enough.

If you can, add the steps to reproduce it and the log file (enable debug mode in Settings > System, then reproduce the bug).

Thank you for your contribution!
