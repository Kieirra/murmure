# Shortcut Issues

## Shortcut Doesn't Work

### On macOS

**Check permissions**: Go to System Settings > Privacy & Security and verify that Murmure is enabled in both **Accessibility** and **Input Monitoring**.

**After upgrading from 1.6.0**: You must completely reset permissions. See the [macOS installation guide](../getting-started/macos.md#upgrading-from-160) for the exact steps.

**System shortcut conflict**: `Ctrl+Space` is used by macOS for switching input sources. Change your Murmure shortcut to something else (e.g., `Ctrl+Option+M`, `F2`).

**"Failed to save shortcut"**: Another application is already using that shortcut. Try a different combination.

### On Linux (Wayland)

Murmure does not register any global shortcut itself on Wayland. You bind OS-level custom shortcuts that call the `murmure` binary directly. See [Configure shortcuts on Linux](../configure-shortcuts-on-linux.md) for the per-compositor walkthrough (GNOME, KDE, Hyprland, Sway).

If a shortcut does not trigger:

- Verify `murmure` is in the PATH (`which murmure`).
- Make sure Murmure is already running in the background. The CLI commands communicate with the running instance.
- Check that no other application has claimed the same key combination in your OS keyboard settings.

### On Linux (X11)

If the shortcut doesn't respond at all:

1. Make sure no other application has claimed the same global shortcut
2. Try a different shortcut
3. Check the logs (Settings > System > Debug mode > folder icon)

### On Windows

If the shortcut doesn't work:

1. Check if another application is using the same shortcut
2. Check if your antivirus (especially Kaspersky) is blocking the global shortcut listener
3. Try running Murmure as administrator (temporary test only)

## Shortcut Toggles Rapidly (Linux)

(X11 only, push-to-talk is disabled on Wayland.)

On Linux X11, holding a shortcut in Push-to-talk mode may toggle recording on/off very quickly instead of holding steady.

**Cause**: X11 sends repeated key events while a key is held down (auto-repeat).

**Fix**: This is handled internally by a cooldown mechanism since version 1.9.0. If you still observe rapid toggling, check that you are running the latest version.

## F13-F24 Keys Not Recognized

Support for extended function keys (F13-F24), keypad keys, and OEM keys was added in version **1.8.0**. Update Murmure if you're on an older version.

These keys are useful for Stream Deck and custom keyboard users.

## Mouse Buttons

Mouse button shortcuts are supported since v1.8.0. This can be a good alternative to keyboard shortcuts, especially on macOS where keyboard shortcuts have more quirks.

## Recommended Shortcuts by OS

| OS                  | Recommended                               | Avoid                           |
| ------------------- | ----------------------------------------- | ------------------------------- |
| **Windows**         | `Ctrl+Space`, `Ctrl+Alt+M`, `F2`          | AltGr combos (AltGr = Ctrl+Alt) |
| **macOS**           | `Ctrl+Option+M`, `F2`, `F3`, mouse button | Space, numbers, letters         |
| **Linux (X11)**     | `Ctrl+Space`, `F2`, `Ctrl+Alt+M`          | -                               |
| **Linux (Wayland)** | Bind any combination at OS level via `murmure --transcription`. See Configure shortcuts on Linux. | -                               |
