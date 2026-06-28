# Beta Testing

Thank you for joining the Murmure beta program! Your feedback is invaluable to make the application rock-solid before its official release.

## How to Get the Beta

Beta builds are published before each release. Head over to the [GitHub Releases](https://github.com/Kieirra/murmure/releases) page and download the latest pre-release version.

## What's New in 1.10.0

- Audio chunking: long dictations no longer hit the 5-minute limit and chunks transcribe in the background while you keep speaking, so long dictations finish much faster
- New custom Parakeet model for better accuracy and fewer unwanted switches to English
- Higher quality audio resampling improves accuracy, especially on low-end microphones
- Dictionary: improved algorithm for better accuracy, alphabetical sorting and a redesigned UI
- Redesigned home page
- CLI: transcribe audio directly from the terminal with the `murmure` command
- Overlay: close button to cancel an ongoing transcription
- Text Insert Mode: new None option to disable auto-insertion
- Monochrome tray icons (idle and recording) on Linux and macOS
- Shortcuts: the Delete key removes the selected shortcut, and duplicates are now prevented
- Debug option to keep the last five audio recordings in the temp folder, with a button to open it
- Fixes: crackling/robotic recordings on some Linux setups, Bluetooth devices kept active, recording restarting after a Ctrl/Shift-only shortcut, Smart Mic chunking with a 20-minute limit
- Security fixes flagged by dependency audits

## Test Plan

Test whatever you can, no pressure. Every checked box helps us.

### Transcription

- [ ] Record and transcribe in push-to-talk
- [ ] Record and transcribe in toggle-to-talk
- [ ] Test on a short phrase (5 to 6 words)
- [ ] Test a transcription with LLM post-processing
- [ ] (Optional) Test a long dictation over 5 minutes and verify there is no cutoff

### Dictionary

- [ ] Add custom words and verify the improved dictionary algorithm picks them up better during transcription

### Overlay

- [ ] Verify the overlay appears during recording
- [ ] Use the close button to cancel an ongoing transcription

### CLI

- [ ] Transcribe an audio file from the terminal with the `murmure` command

### Shortcuts

- [ ] Remove a shortcut with the Delete key
- [ ] Try to add a duplicate shortcut and verify it is prevented

### Smart Speech Mic

- [ ] Test the Smart Speech Mic

### Settings

- [ ] Set Text Insert Mode to None and verify nothing is auto-inserted
- [ ] Enable the debug option to keep the last five recordings, then open the temp folder

### Other

- [ ] Verify the monochrome tray icons (idle and recording) on Linux and macOS
- [ ] (Linux) Verify recordings are no longer crackling or robotic
- [ ] Verify Bluetooth audio devices are released when idle

## Reporting Bugs

No need to open a GitHub issue, just reply directly in the beta announcement conversation with:

- **OS**: Windows, macOS (Intel or Silicon) or Linux (with the distribution)
- **Version**: the beta version you used
- **Description**: what happened
- **Steps to reproduce**: how to trigger the bug
- **Logs**: enable debug mode in Settings > System, reproduce the bug, then attach the log file

Thank you for your contribution!
