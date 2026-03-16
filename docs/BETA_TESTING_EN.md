# Beta Tester Guide - Murmure v1.8.0

Thank you for participating in the Murmure beta testing program! Your contribution is essential to improve the quality of the application before its official release.

---

## How to Join the Beta Testing Program?

Send a message on LinkedIn to [Luc Marongiu](https://www.linkedin.com/in/luc-m-4b309aa8/) with your operating system (Windows, macOS or Linux).

You will then receive the download link for the beta version.

---

## Test Plan

Test what you can, no pressure:

### Installation and Startup

- [ ] Download and install beta version 1.8.0
- [ ] Verify the application starts correctly
- [ ] Complete initial onboarding

### Voice Mode (#171, #178)

- [ ] Enable voice mode in settings
- [ ] Say the wake word to trigger a recording
- [ ] Test auto-send Enter after voice transcription (#156)
- [ ] Start a recording via keyboard, then use voice words to validate/cancel
- [ ] Verify that voice mode disables/re-enables correctly

### LLM Connect: Remote Server

- [ ] Configure a connection to a remote server (OpenAI-compatible API)
- [ ] Test a transcription with remote LLM post-processing
- [ ] Create multiple LLM modes with different providers (local Ollama + remote)
- [ ] Reorder LLM modes via drag and drop (#104)
- [ ] Verify the correct provider is used for each mode
- [ ] Test with a reasoning model (Qwen 3.5, Ministral) and verify that the response speed is acceptable

### Settings Import/Export

- [ ] Open Settings > Import/Export
- [ ] Export all settings
- [ ] Export only specific settings (partial export)
- [ ] Change a setting, then import the previously exported file
- [ ] Verify settings are restored correctly
- [ ] (Linux/macOS) Test CLI import: `murmure import <file>` (#223)
- [ ] (Windows) Test CLI import: `murmure.exe import <file>` (#223)

### Shortcuts

- [ ] Assign a mouse button as a shortcut (#158)
- [ ] Verify the mouse button triggers the action correctly
- [ ] Test the cancel recording shortcut in the overlay (#161)
- [ ] Assign an F13-F24 key as a shortcut (#189)
- [ ] Assign an OEM key (e.g., -, =, [, ;) as a shortcut

### Formatting Rules

- [ ] Create a rule with a regular expression (#105)
- [ ] Verify the regex is correctly applied to the transcription
- [ ] Check the new rule labels (readable sentences) (#101)
- [ ] Hover over the "?" icon in the replacement text field and verify the help
- [ ] Test short text correction: dictate a single word, verify lowercase and no trailing punctuation
- [ ] Reorder rules via drag and drop (#170)
- [ ] Verify the application order matches the new order

### Interface and System

- [ ] Disable autostart, re-enable it, then restart and verify the app starts minimized to tray (#201)
- [ ] (macOS) Configure show/hide in Dock (#226)
- [ ] Visit the "About" page and verify the new interface (#198)
- [ ] Check dark mode color consistency
- [ ] Click the "Release notes" link in the sidebar
- [ ] Unplug a selected microphone, verify the choice is preserved

---

## Beta Testing Report

After your tests, send a report with:

### Info

- **Username / Name**:
- **OS**: Windows / macOS / Linux (version)

### Tests Completed

- [ ] Installation and Startup
- [ ] Voice Mode
- [ ] LLM Connect: Remote Server
- [ ] Settings Import/Export
- [ ] Shortcuts
- [ ] Formatting Rules
- [ ] Interface and System

### Bugs Found

For each bug:

- **Description**: What happened?
- **How to reproduce**: Steps to reproduce the bug

---

Thank you for your contribution!
