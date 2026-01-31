# Beta Tester Guide - Murmure v1.7.0

Thank you for participating in the Murmure beta testing program! Your contribution is essential to improve the quality of the application before its official release.

---

## How to Join the Beta Testing Program?

Simply send a message with:
- Your operating system (Windows, macOS or Linux)

### Via LinkedIn
Send a direct message to the Murmure team on LinkedIn.

### Via GitHub Discussions
Post a message on [GitHub Discussions](https://github.com/Kieirra/murmure/discussions).

Once registered, you will receive the download link for the beta version.

---

## Version 1.7.0 Features to Test

### System Settings

| Feature | Description | Issue |
|---------|-------------|-------|
| **Microphone selection** | Choose the audio input device | #81 |
| **Log verbosity levels** | Configure log detail level (trace, debug, info, warn, error) | #90 |
| **Open logs folder button** | Quick access to log files | #130 |
| **Text insertion mode** | Three modes: Standard (Ctrl+V), Terminal (Ctrl+Shift+V), Direct (character by character typing) | #121 |

### Custom Dictionary

| Feature | Description | Issue |
|---------|-------------|-------|
| **CSV Import/Export** | Export and import dictionaries in CSV format | #72 |
| **Medical presets** | Pre-configured dictionaries for specialized vocabulary | #72 |
| **Case preservation** | Support for case sensitivity in custom entries | #109 |

### Text Commands

| Feature | Description | Issue |
|---------|-------------|-------|
| **Custom commands** | Select transcribed text and apply commands (fix grammar, translate, etc.) | #107, #122 |

### LLM Integration

| Feature | Description | Issue |
|---------|-------------|-------|
| **Multiple saved prompts** | Create and manage multiple prompt configurations | #110 |
| **Mode switching shortcuts** | Switch between LLM modes via keyboard shortcuts | #110 |

### Text Formatting

| Feature | Description | Issue |
|---------|-------------|-------|
| **Digit conversion threshold** | Configure from which number words are converted to digits (e.g., "three" -> "3") | #106 |

### Technical Improvements

| Feature | Description | Issue |
|---------|-------------|-------|
| **Windows shortcuts** | Fixed shortcut handling on Windows | #128 |
| **Shortcuts refactoring** | Optimized keyboard shortcut logic | #123 |
| **Security updates** | Updated dependencies to fix vulnerabilities | #117 |
| **NSIS Windows installer** | Installation without administrator privileges | #96 |

---

## Test Plan (Todo List)

Check each item after testing:

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
- [ ] Compare speed and reliability of each mode

### Dictionary Import/Export (#72)
- [ ] Open Settings > Custom Dictionary
- [ ] Add a few words to the dictionary
- [ ] Export dictionary to CSV format
- [ ] Verify exported CSV file contents
- [ ] Delete words from dictionary
- [ ] Import the previously exported CSV file
- [ ] Verify words are restored
- [ ] Test medical preset import (if available)

### Case Preservation (#109)
- [ ] Add a word with specific casing (e.g., "iPhone", "macOS")
- [ ] Make a transcription containing that word
- [ ] Verify the casing is respected in the result

### Custom Commands (#107, #122)
- [ ] Make a transcription
- [ ] Select part of the transcribed text
- [ ] Apply a custom command (fix grammar)
- [ ] Verify command result
- [ ] Test translation command (if LLM configured)

### Multiple LLM Prompts (#110)
- [ ] Open LLM configuration
- [ ] Create a first custom prompt and save it
- [ ] Create a second different prompt and save it
- [ ] Use keyboard shortcuts to switch between modes
- [ ] Verify the correct prompt is applied during transcription
- [ ] Verify prompts persist after restart

### Digit Conversion Threshold (#106)
- [ ] Open Settings > Formatting rules
- [ ] Set threshold to 0: say "one two three" -> should display "1 2 3"
- [ ] Set threshold to 10: say "three" -> "three", say "fifteen" -> "15"
- [ ] Verify the threshold is respected in transcriptions

### Multi-platform Tests

#### Windows
- [ ] Test global keyboard shortcuts
- [ ] Verify NSIS installation (without admin)
- [ ] Test with different applications (Word, Notepad, VS Code, Terminal)

#### macOS
- [ ] Test global keyboard shortcuts
- [ ] Verify microphone permissions
- [ ] Test with different applications

#### Linux
- [ ] Test under X11
- [ ] Test under Wayland (known limitations)
- [ ] Verify overlay behavior

### Performance Tests
- [ ] Measure transcription time for 30 seconds of audio
- [ ] Check memory usage during transcription
- [ ] Test with long recordings (5+ minutes)
- [ ] Verify no memory leaks after multiple transcriptions

### Stability Tests
- [ ] Use the application for a full day
- [ ] Make repeated transcriptions
- [ ] Test edge cases (no microphone, no network for LLM)
- [ ] Verify crash/recovery behavior

---

## Beta Testing Report Format

After your testing phase, please send a report with the following information:

### General Information

```
Name/Username:
Test date:
Version tested: 1.7.0-beta
Operating system: [Windows 10/11, macOS version, Linux distribution]
Architecture: [x64, ARM64]
Audio hardware: [Microphone used]
```

### Test Summary

```
Number of features tested: X / Y
Total test duration: X hours
Number of bugs found: X
Number of suggestions: X
```

### Bugs Found

For each bug, provide:

```
## Bug #1: [Short title]

**Severity**: [Critical / Major / Minor / Cosmetic]

**Feature concerned**: [e.g., Microphone selection]

**Steps to reproduce**:
1.
2.
3.

**Expected behavior**:


**Observed behavior**:


**Frequency**: [Always / Often / Sometimes / Rare]

**Screenshot/Video**: [Link or attachment]

**Logs**: [Copy relevant logs from logs folder]
```

### Improvement Suggestions

```
## Suggestion #1: [Title]

**Feature concerned**:

**Improvement description**:

**Expected benefit**:
```

### Overall Evaluation

```
Overall rating (1-10):

Strengths:
-
-

Areas for improvement:
-
-

Would you recommend Murmure? [Yes / No / Maybe]

Free comments:

```

---

## How to Submit Your Report?

### Option 1: GitHub Issues
Create an issue on [GitHub](https://github.com/Kieirra/murmure/issues) with the `beta-testing` label for each bug found.

### Option 2: GitHub Discussions
Post your complete report in the dedicated beta testing discussion.

### Option 3: Email/LinkedIn
Send your report directly to the team via the communication channels established during registration.

---

## Tips for Good Beta Testing

1. **Test in real conditions**: Use Murmure as you would in daily use
2. **Document everything**: Take notes, screenshots, and logs
3. **Be precise**: The more detailed your reports, the more useful they are
4. **Test edge cases**: Try unusual scenarios
5. **Compare with previous version**: Note improvements and regressions

---

## FAQ

**Q: Can I use the beta version for daily work?**
A: Yes, but keep in mind that bugs may occur. Always have a backup solution.

**Q: How to get help during testing?**
A: Use GitHub Discussions or contact the team via LinkedIn.

**Q: Is my data safe?**
A: Yes, Murmure works 100% locally. No data is sent to external servers (unless you use a remote LLM).

**Q: How long does the beta testing phase last?**
A: Duration will be communicated during registration. Usually 1 to 2 weeks.

---

Thank you for your contribution to the development of Murmure!

*The Murmure Team*
