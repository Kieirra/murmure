# First Steps

After installing Murmure, here's how to get the most out of it.

## Your First Transcription

1. Open any text field (a browser, a text editor, a chat app)
2. Press `Ctrl+Space` (default shortcut)
3. Speak clearly into your microphone
4. Release the shortcut (or press it again in toggle mode)
5. Your text appears in the focused application

!!! tip
    The first transcription after launching Murmure is slightly slower because the AI model needs to warm up. Subsequent transcriptions are faster.

## Choose Your Recording Mode

![System Settings](../assets/settings-system.png)

Go to **Settings** > **System** to pick a recording mode:

| Mode | How It Works |
|---|---|
| **Push-to-talk** (default) | Hold the shortcut to record, release to stop |
| **Toggle-to-talk** | Press once to start, press again to stop |

For hands-free recording with wake words, see [Voice Mode](../features/voice-mode.md) (separate feature in Extensions).

## Select Your Microphone

By default, Murmure uses your system's default microphone. To choose a specific one:

1. Go to **Settings** > **System**
2. Under **Microphone**, select the device you want

!!! warning "Virtual microphones"
    If you use a virtual microphone (NVIDIA Broadcast, VB-Audio, etc.), make sure to select it explicitly. The "Automatic" option may not pick it up.

## Configure the Shortcut

![Shortcuts](../assets/settings-shortcuts.png)

The default shortcut `Ctrl+Space` may conflict with other apps. To change it:

1. Go to **Settings** > **Shortcuts**
2. Click the shortcut field and press your preferred key combination

**Recommended shortcuts by OS:**

| OS | Recommended | Avoid |
|---|---|---|
| Windows | `Ctrl+Space`, `Ctrl+Alt+M`, `F2`, side/extra mouse button | AltGr combos (interpreted as Ctrl+Alt) |
| macOS | `Ctrl+Option+M`, `F2`, `F3`, side/extra mouse button | Anything with Space or numbers |
| Linux (X11) | `Ctrl+Space`, `F2`, side/extra mouse button | - |

## Text Insertion Modes

If your transcribed text doesn't appear in some applications, you may need to change the text insertion mode:

Go to **Settings** > **System** > **Text Insertion Mode**:

| Mode | How It Works | Best For |
|---|---|---|
| **Standard** (Ctrl+V) | Copies text to clipboard and simulates Ctrl+V | Most applications |
| **Terminal** (Ctrl+Shift+V) | Uses terminal-style paste | Terminal emulators |
| **Direct** (type text) | Simulates individual keystrokes | LibreOffice, Git Bash, apps where Ctrl+V doesn't work |

!!! tip
    If text doesn't appear after transcription, try switching to **Direct** mode. See [Text Insertion Troubleshooting](../troubleshooting/text-insertion.md) for more details.

## Enable Start on Boot

Go to **Settings** > **System** and enable **Launch on startup**. Murmure will start minimized to the system tray on boot.

## What's Next?

- [Dictionary](../features/dictionary.md) - Add custom words for better recognition
- [Formatting Rules](../features/formatting-rules.md) - Auto-correct and transform text
- [LLM Connect](../features/llm-connect.md) - Post-process with a local AI
- [Voice Mode](../features/voice-mode.md) - Hands-free activation with wake words
- [Smart Speech Mic](../features/smart-speech-mic.md) - Use your phone as a wireless mic
