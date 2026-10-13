# Beta Testing

Thank you for joining the Murmure beta program! Your feedback is invaluable to make the application rock-solid before its official release.

## How to Get the Beta

Beta builds are published before each release. Head over to the [GitHub Releases](https://github.com/Kieirra/murmure/releases) page and download the latest pre-release version.

## What's New in 2.0.0

### New transcription model

- Murmure now uses Parakeet ultra, a fine-tuned version of Parakeet made by Moondream, which makes fewer mistakes
- The model is the int8 export by @thiswillbeyourgithub, which uses about 380 MB less RAM than the previous one

### New interface

- Every page was redesigned, with new fields, tabs and buttons
- Settings now comes before Extensions in the sidebar
- The getting started checklist moved to a card at the bottom of the sidebar
- Each extension page has its own color, and its on/off switch is now next to the page title
- In Personalize > Dictionary, Import, Export and Clear are now visible buttons, and a removed word can be restored with **Undo**
- Personalize > Formatting Rules has two tabs, **General rules** and **Custom rules**
- Linux Wayland: the setup steps now open in a window on the Home page

### Result after dictation

- The overlay can stay on screen after a dictation, with the text and a copy button. Choose when in Settings > System > **Show result after dictation** (Off, Command, Command and LLM, All dictations), and choose how long it stays
- Move the mouse over the result to keep it on screen

### Automatic insert

- New **Automatic insert** switch in Settings > System, separate from the insertion method. When it is off, Murmure never types the text by itself. The text stays in the history, and in the clipboard if **Copy to Clipboard** is on
- The **Paste last transcript** shortcut always pastes, even when Automatic insert is off
- If you used the insertion method "None (manual paste)", it is converted for you. Automatic insert is off and your other settings are kept

### Prompt Mode and Command Mode

- LLM Connect is now called Prompt Mode
- Command Mode is a separate extension, with its own model. Enable it in Extensions > Command Mode
- With nothing selected, a command now answers your question instead of repeating it

### Local API

- The local API is no longer experimental. It has its own page in Extensions > Local API
- New endpoints apply a Prompt Mode prompt to the transcription and return the final text
- Each endpoint has a **Try it** button to test it with a WAV file, without writing any code

### Other

- Hesitation sound removal ("euh", "um") is now optional and off by default, because it also removed real words such as the German "um". Turn it back on in Personalize > Formatting Rules
- The tray menu is translated into your language
- New `--quit` command line flag to close Murmure
- Transcriptions no longer contain a stray `<unk>` token
- macOS: clicking the Dock icon reopens the window
- Linux: the volume of audio started during a dictation, such as a new browser tab, is restored at the end
- Updates downloaded as `.deb` keep the right file extension
- Several security improvements

## Test Plan

Do what you can, even one box helps. Start with the four essentials, they take about five minutes.

### The essentials

- [ ] Dictate a few sentences like you normally do, and check the text lands correctly and is accurate
- [ ] In Settings > System, set **Show result after dictation** to **All dictations**, dictate, and check the result appears with a copy button, then disappears after the chosen duration
- [ ] Turn off **Automatic insert**, dictate, and check that nothing is typed but the result still appears. Then use the **Paste last transcript** shortcut and check the text is pasted
- [ ] If you updated from an older version, check that your settings, shortcuts and dictionary are still there

### If you have more time

- [ ] Open every page once, and tell us if something looks broken, cut or hard to read
- [ ] On the extension pages you use, turn the extension off and on with the switch next to the title, then restart Murmure and check it stayed as you left it
- [ ] In Personalize > Dictionary, remove a word, click **Undo**, and check the word comes back
- [ ] If you use Prompt Mode or Command Mode: enable Command Mode, then press the Command shortcut with nothing selected and ask a question. Then select a sentence and say "translate to English"
- [ ] Open Extensions > Local API, enable it, and use **Try it** with a short WAV file
- [ ] Dictate something long, over a minute, and check nothing is missing at the end
- [ ] Open the tray menu and check it is in your language

### Only the line matching your setup

- [ ] macOS: close the Murmure window, then click the Dock icon, and check the window comes back
- [ ] Linux: turn on the volume reduction in Settings > System, start a video in a new browser tab while you dictate, and check its volume comes back at the end
- [ ] Linux: run `murmure --quit` in a terminal and check Murmure closes
- [ ] Linux Wayland: follow the steps of the setup window on the Home page, dictate once with your shortcut, and check the setup banner is gone

## Reporting Bugs

No need to open a GitHub issue, just reply in the beta announcement conversation. Tell us what broke and on which OS, that is already enough.

If you can, add the steps to reproduce it and the log file (enable debug mode in Settings > System, then reproduce the bug).

Thank you for your contribution!
