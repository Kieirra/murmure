# Beta Testing

Thank you for participating in the Murmure beta testing program! Your contribution is essential to improve the quality of the application before its official release.

## How to Get the Beta

Beta builds are shared before each release. Check the [GitHub Releases](https://github.com/Kieirra/murmure/releases) page for pre-release versions.

## Test Plan

Test what you can, no pressure:

### Installation and Startup

- [ ] Download and install the beta version
- [ ] Verify the application starts correctly
- [ ] Complete initial onboarding

### Core Transcription

- [ ] Record and transcribe with push-to-talk
- [ ] Record and transcribe with toggle-to-talk
- [ ] Verify the correct microphone is used
- [ ] Test with a short phrase (1-2 words)
- [ ] Test with a long dictation (2+ minutes)

### Voice Mode

- [ ] Enable voice mode in settings
- [ ] Say the wake word to trigger a recording
- [ ] Test auto-send Enter after voice transcription
- [ ] Start a recording via keyboard, then use voice words to validate/cancel
- [ ] Verify that voice mode disables/re-enables correctly

### LLM Connect

- [ ] Configure a connection to a local Ollama server
- [ ] Configure a connection to a remote server (OpenAI-compatible API)
- [ ] Test a transcription with LLM post-processing
- [ ] Create multiple LLM modes with different providers
- [ ] Reorder LLM modes via drag and drop
- [ ] Verify the correct model is used for each mode

### Smart Speech Mic

- [ ] Enable Smart Mic and scan the QR code with your phone
- [ ] Verify audio streams from the phone to Murmure
- [ ] Test a transcription using the phone microphone

### Settings Import/Export

- [ ] Export all settings
- [ ] Export only specific settings (partial export)
- [ ] Change a setting, then import the previously exported file
- [ ] Verify settings are restored correctly
- [ ] Test CLI import: `murmure import <file>`

### Shortcuts

- [ ] Assign a mouse button as a shortcut
- [ ] Assign an F13-F24 key as a shortcut
- [ ] Assign an OEM key (e.g., -, =, [, ;) as a shortcut
- [ ] Test the cancel recording shortcut

### Formatting Rules

- [ ] Create a rule with a regular expression
- [ ] Verify the regex is correctly applied to the transcription
- [ ] Reorder rules via drag and drop
- [ ] Verify the application order matches the new order
- [ ] Test short text correction: dictate a single word, verify lowercase and no trailing punctuation

### Dictionary

- [ ] Add a custom word and verify it's corrected in transcription
- [ ] Import/export the dictionary
- [ ] Clear all entries

### Interface and System

- [ ] Disable autostart, re-enable it, then restart and verify the app starts minimized to tray
- [ ] Check dark mode color consistency
- [ ] Click the "Release notes" link in the sidebar
- [ ] Unplug a selected microphone, verify the choice is preserved

## Reporting Bugs

After your tests, [open a GitHub issue](https://github.com/Kieirra/murmure/issues/new) with:

- **OS**: Windows / macOS (Intel/Silicon) / Linux (distribution)
- **Version**: The beta version you tested
- **Description**: What happened?
- **Steps to reproduce**: How to trigger the bug
- **Logs**: Enable debug mode in Settings > System, reproduce the bug, then attach the log file (click the folder icon next to the log level)

Thank you for your contribution!
