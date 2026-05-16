# Beta Testing

Thank you for joining the Murmure beta program! Your feedback is invaluable to make the application rock-solid before its official release.

## How to Get the Beta

Beta builds are published before each release. Head over to the [GitHub Releases](https://github.com/Kieirra/murmure/releases) page and download the latest pre-release version.

## What's New in 1.9.0

- Smart Speech Mic: use your smartphone as a microphone to drive Murmure
- Redesigned menus
- Overlay: streaming mode and new customization options
- macOS: the Escape key no longer blocks other applications
- Linux: Wayland support (smoother on KDE than on GNOME)
- LLM Connect shortcuts no longer appear in the shortcut list when the feature is disabled
- Logs are now displayed in your local timezone
- The tray icon now changes during recording
- Custom name for your formatting rules
- Voice Mode and Smart Mic are now included in Import/Export
- Warning when digits are used in the Dictionary

## Test Plan

Test whatever you can, no pressure. Every checked box helps us.

### Transcription

- [ ] Record and transcribe in push-to-talk
- [ ] Record and transcribe in toggle-to-talk
- [ ] Test on a short phrase (5 to 6 words)
- [ ] Test on a longer dictation (2 to 3 sentences)
- [ ] Test a transcription with LLM post-processing

### Overlay

- [ ] Verify the overlay appears during recording
- [ ] Enable streaming mode and verify it works correctly
- [ ] Customize the overlay and verify the settings are properly applied

### Voice Mode

- [ ] Enable voice mode from the Extensions menu
- [ ] Trigger a recording by saying the wake word
- [ ] Test auto-send with "Thanks alix" after a voice transcription
- [ ] Test with the silence delay set to Indefinite
- [ ] Verify voice mode toggles correctly via Ctrl+Shift+0

### Smart Speech Mic

- [ ] Enable Smart Mic and scan the QR code with your phone
- [ ] Verify that audio from the phone reaches Murmure
- [ ] Run a transcription using the phone microphone
- [ ] Test the left-click, delete and enter actions from the phone

### Settings Import/Export

- [ ] Export all settings
- [ ] Export only a selection of settings
- [ ] Change a setting, then re-import the exported file
- [ ] Verify that settings are properly restored
- [ ] Test CLI import: `murmure import <file>`

### Other

- [ ] (macOS) Verify Escape no longer blocks other apps from closing when no transcription is running
- [ ] Verify Escape cancels the ongoing recording
- [ ] Rename a custom formatting rule

## Reporting Bugs

No need to open a GitHub issue, just reply directly in the beta announcement conversation with:

- **OS**: Windows, macOS (Intel or Silicon) or Linux (with the distribution)
- **Version**: the beta version you used
- **Description**: what happened
- **Steps to reproduce**: how to trigger the bug
- **Logs**: enable debug mode in Settings > System, reproduce the bug, then attach the log file

Thank you for your contribution!
