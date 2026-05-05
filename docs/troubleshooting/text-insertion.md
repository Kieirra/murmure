# Text Insertion Issues

## Text Doesn't Appear After Transcription

The transcription works (you can see it in Murmure's history) but the text doesn't appear in your target application.

### Cause

By default, Murmure inserts text by copying to the clipboard and simulating `Ctrl+V`. Some applications handle clipboard paste differently or block it entirely.

### Fix: Change Text Insertion Mode

Go to **Settings** > **System** > **Text Insertion Mode** and try a different mode:

| Mode         | Shortcut       | Best For                                              |
| ------------ | -------------- | ----------------------------------------------------- |
| **Standard** | Ctrl+V         | Most desktop applications, browsers, editors          |
| **Terminal** | Ctrl+Shift+V   | Terminal emulators (GNOME Terminal, Konsole, etc.)    |
| **Direct**   | Key simulation | LibreOffice, Git Bash, apps where Ctrl+V doesn't work |

### Applications Known to Need Direct Mode

- **LibreOffice** (Writer, Calc, Impress)
- **Git Bash** on Windows
- **Some Linux terminal emulators**
- **Electron apps** that intercept clipboard events

!!! note "Direct mode limitations on Linux"
    On some Linux configurations, Direct mode may not display diacritics (accented characters like e, a, u) correctly. If you encounter this, try Standard or Terminal mode instead.

## Text Appears in the Wrong Place

Make sure the target application is focused (in the foreground) when you stop recording. Murmure pastes into whatever window is focused at the moment the transcription finishes.

## Clipboard Content is Overwritten

Standard mode uses the clipboard to insert text. This means your previous clipboard content is replaced. If this is a problem, consider using **Direct** mode which simulates keystrokes without touching the clipboard.

## Linux (Wayland) — Paste Doesn't Work

On Wayland, Murmure needs one-time access to a system device to paste text into other applications. The `.deb` package sets this up automatically. The AppImage can't, so you run a short command once.

### DEB package

Already configured. If paste still doesn't work right after installing, log out and back in so the change takes effect.

### AppImage

If you see the **"Wayland keystroke injection is unavailable"** notification, open a terminal and run the command shown in its **Copy command** button (reproduced below):

```bash
sudo tee /etc/udev/rules.d/60-murmure-uinput.rules > /dev/null <<'EOF'
KERNEL=="uinput", SUBSYSTEM=="misc", OPTIONS+="static_node=uinput", GROUP="input", MODE="0660", TAG+="uaccess"
EOF
sudo udevadm control --reload-rules
sudo udevadm trigger --property-match=DEVNAME=/dev/uinput
```

Then log out and back in. The notification will stop appearing and paste will work.

## AltGr Triggers Recording (Windows)

On Windows, `AltGr` is interpreted as `Ctrl+Alt`. If your recording shortcut is `Ctrl+Alt+something`, pressing `AltGr` may accidentally trigger recording.

**Fix**: Choose a shortcut that doesn't involve `Ctrl+Alt`, or use a function key.

## Extra Characters Appear (macOS)

On macOS, shortcuts containing `Space` or number keys may "leak" those characters into the active application while held. For example, `Shift+Space` produces multiple space characters.

**Fix**: Use modifier-only combos like `Ctrl+Option+M` or function keys (`F2`, `F3`). Avoid shortcuts containing Space, numbers, or letter keys on macOS.
