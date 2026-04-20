# Shortcut Issues

## Shortcut Doesn't Work

### On macOS

**Check permissions**: Go to System Settings > Privacy & Security and verify that Murmure is enabled in both **Accessibility** and **Input Monitoring**.

**After upgrading from 1.6.0**: You must completely reset permissions. See the [macOS installation guide](../getting-started/macos.md#upgrading-from-160) for the exact steps.

**System shortcut conflict**: `Ctrl+Space` is used by macOS for switching input sources. Change your Murmure shortcut to something else (e.g., `Ctrl+Option+M`, `F2`).

**"Failed to save shortcut"**: Another application is already using that shortcut. Try a different combination.

### On Linux (Wayland)

Global shortcuts on Wayland go through your desktop's **GlobalShortcuts portal**, which is part of `xdg-desktop-portal`. Most modern Linux desktops ship it by default (GNOME 48+, KDE Plasma 6.x, Hyprland, etc.).

If Murmure reports it couldn't register a shortcut, install the portal backend for your desktop:

```bash
# GNOME
sudo apt install xdg-desktop-portal-gnome

# KDE
sudo apt install xdg-desktop-portal-kde

# Hyprland
sudo apt install xdg-desktop-portal-hyprland
```

Then restart Murmure.

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
| **Linux (Wayland)** | `Ctrl+Space`, `F2`, `Ctrl+Alt+M`          | -                               |
