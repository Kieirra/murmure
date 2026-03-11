# Beta Tester Guide - Murmure v1.7.0

Thank you for participating in the Murmure beta testing program! Your contribution is essential to improve the quality of the application before its official release.

---

## How to Join the Beta Testing Program?

Send a message on LinkedIn to [Luc Marongiu](https://www.linkedin.com/in/luc-m-4b309aa8/) with your operating system (Windows, macOS or Linux).

You will then receive the download link for the beta version.

---

## Version 1.7.0 Features to Test

### System Settings

| Feature                     | Description                                                                                     | Issue |
| --------------------------- | ----------------------------------------------------------------------------------------------- | ----- |
| **Microphone selection**    | Choose the audio input device                                                                   | #81   |
| **Log verbosity levels**    | Configure log detail level (trace, debug, info, warn, error)                                    | #90   |
| **Open logs folder button** | Quick access to log files                                                                       | #130  |
| **Text insertion mode**     | Three modes: Standard (Ctrl+V), Terminal (Ctrl+Shift+V), Direct (character by character typing) | #121  |

### Custom Dictionary

| Feature               | Description                                            | Issue |
| --------------------- | ------------------------------------------------------ | ----- |
| **CSV Import/Export** | Export and import dictionaries in CSV format           | #72   |
| **Medical presets**   | Pre-configured dictionaries for specialized vocabulary | #72   |
| **Case preservation** | Support for case sensitivity in custom entries         | #109  |

### Text Commands

| Feature             | Description                                                               | Issue      |
| ------------------- | ------------------------------------------------------------------------- | ---------- |
| **Custom commands** | Select transcribed text and apply commands (fix grammar, translate, etc.) | #107, #122 |

### LLM Integration

| Feature                      | Description                                      | Issue |
| ---------------------------- | ------------------------------------------------ | ----- |
| **Multiple saved prompts**   | Create and manage multiple prompt configurations | #110  |
| **Mode switching shortcuts** | Switch between LLM modes via keyboard shortcuts  | #110  |

### Text Formatting

| Feature                        | Description                                                                      | Issue |
| ------------------------------ | -------------------------------------------------------------------------------- | ----- |
| **Digit conversion threshold** | Configure from which number words are converted to digits (e.g., "three" -> "3") | #106  |

### Technical Improvements

| Feature                    | Description                                   | Issue |
| -------------------------- | --------------------------------------------- | ----- |
| **Windows shortcuts**      | Fixed shortcut handling on Windows            | #128  |
| **Shortcuts refactoring**  | Optimized keyboard shortcut logic             | #123  |
| **Security updates**       | Updated dependencies to fix vulnerabilities   | #117  |
| **NSIS Windows installer** | Installation without administrator privileges | #96   |

---

## Test Plan

Test what you can, no pressure:

### Installation and Startup

- [ ] Download and install beta version 1.7.0
- [ ] Verify the application starts correctly
- [ ] Verify automatic AI model download (if first launch)
- [ ] Complete initial onboarding

### Microphone Selection (#81)

- [ ] Open Settings > System > Microphone
- [ ] Verify the list of available microphones is displayed
- [ ] Select a different microphone
- [ ] Test recording with the new microphone
- [ ] Verify the choice is preserved after restart

### Log Verbosity Levels (#90)

- [ ] Open Settings > System > Logs
- [ ] Change log level (trace, debug, info, warn, error)
- [ ] Verify warning for sensitive levels (debug/trace)
- [ ] Click the "Open logs folder" button
- [ ] Verify logs match the selected level

### Text Insertion Mode (#121)

- [ ] Open Settings > System > Insertion mode
- [ ] Test "Standard (Ctrl+V)" mode in a text editor
- [ ] Test "Terminal (Ctrl+Shift+V)" mode in a terminal
- [ ] Test "Direct (typing)" mode in an application

### Dictionary Import/Export (#72)

- [ ] Open Settings > Custom Dictionary
- [ ] Add a few words to the dictionary
- [ ] Export dictionary to CSV format
- [ ] Verify exported CSV file contents
- [ ] Delete words from dictionary
- [ ] Import the previously exported CSV file
- [ ] Verify words are restored

### Case Preservation (#109)

- [ ] Add a word with specific casing (e.g., "iPhone", "macOS")
- [ ] Make a transcription containing that word
- [ ] Verify the casing is respected in the result

### Custom Commands (#107, #122)

- [ ] Make a transcription
- [ ] Select part of the transcribed text
- [ ] Apply a custom command (translate to French)
- [ ] Verify command result

### Multiple LLM Prompts (#110)

- [ ] Open LLM configuration
- [ ] Create a first custom prompt
- [ ] Create a second different prompt
- [ ] Use keyboard shortcuts to switch between modes
- [ ] Verify the correct prompt is applied during transcription
- [ ] Verify prompts persist after restart

### Digit Conversion Threshold (#106)

- [ ] Open Settings > Formatting rules
- [ ] Threshold 0: "a dog and two cats" → "a dog and 2 cats"
- [ ] Threshold 3: "a dog and two cats" → "a dog and two cats"

---

## Beta Testing Report

After your tests, send a report with:

### Info

- **Username**:
- **OS**: Windows / macOS / Linux (version)

### Bugs Found

For each bug:

- **Description**: What happened?
- **How to reproduce**: Steps to reproduce the bug

### UX Improvements

If you noticed important UX improvements needed (not bugs, but blocking or frustrating points for users):

- ...

---

Thank you for your contribution!
