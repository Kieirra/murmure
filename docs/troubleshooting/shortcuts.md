# Shortcut Issues

## Shortcut Doesn't Work

### On macOS

**Check permissions**: Go to System Settings > Privacy & Security and verify that Murmure is enabled in both **Accessibility** and **Input Monitoring**.

**After upgrading from 1.6.0**: You must completely reset permissions. See the [macOS installation guide](../getting-started/macos.md#upgrading-from-160) for the exact steps.

**System shortcut conflict**: `Ctrl+Space` is used by macOS for switching input sources. Change your Murmure shortcut to something else (e.g., `Ctrl+Option+M`, `F2`).

**"Failed to save shortcut"**: Another application is already using that shortcut. Try a different combination.

### On Linux (Wayland)

Murmure exposes a **Wayland integration** setting in **Settings > Advanced** with two modes: native portal (`xdg-desktop-portal` GlobalShortcuts) or XWayland (rdev). The mode is picked automatically per desktop and can be changed manually. Restart Murmure after any change.

**KDE Plasma 5.27+/6.x** (default: native portal): shortcuts work reliably. If a shortcut does not trigger, check that no other application has claimed it.

**GNOME 48+** (default: XWayland): the GNOME portal routes shortcuts through Mutter RemoteDesktop, with variable latency (tens to hundreds of milliseconds) and occasional dropped events. We default to XWayland on GNOME for reliability. In XWayland mode, **global shortcuts only fire when the Murmure window has focus**, so for hands-free recording use **Voice Mode**, and make sure **Settings > Advanced > Copy transcription to clipboard** stays enabled so you can paste with `Ctrl+V`.

**Sway, Hyprland and other compositors** (default: native portal): behavior depends on the portal backend available on your system. If shortcuts do not register, switch to XWayland mode in Settings.

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

On Linux, holding a shortcut in Push-to-talk mode may toggle recording on/off very quickly instead of holding steady.

**Cause**: X11 sends repeated key events while a key is held down (auto-repeat). Wayland portals can also emit bursts of events for a single press.

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
| **Linux (Wayland)** | `Ctrl+Shift+Space`, `F2`, mouse button    | -                               |
