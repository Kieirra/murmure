# Configure shortcuts on Linux

On Linux Wayland, global shortcuts can be handled in two ways: via the **XDG Portal** (Murmure registers shortcuts through the `xdg-desktop-portal` GlobalShortcuts interface) or via **CLI** (Murmure registers nothing, and you bind OS-level custom shortcuts that call the `murmure` binary directly).

The active mode is set in **Settings > System > Shortcut handling**. Changing it requires a restart.

| Mode | When to use |
| ---- | ----------- |
| **XDG Portal** | KDE Plasma 6, Hyprland, Sway. The portal works reliably on these compositors. |
| **CLI** | GNOME (default), or any compositor where portal shortcuts misbehave. |

On GNOME, CLI mode is the default because Mutter's portal implementation has known latency and reliability issues that make XDG Portal shortcuts unpredictable.

## CLI commands reference

When CLI mode is active, Murmure exposes the following commands. Each can be assigned to an OS-level custom shortcut.

| Command | Effect |
| ------- | ------ |
| `murmure --transcription` | Toggle standard transcription ON/OFF |
| `murmure --transcription-llm` | Toggle transcription in LLM mode |
| `murmure --transcription-command` | Toggle transcription in Command mode |
| `murmure --paste-last` | Paste the last transcription |
| `murmure --cancel` | Cancel the current recording and return to idle |
| `murmure --voice-mode` | Toggle Voice Mode ON/OFF |
| `murmure --llm-mode 1` | Switch to LLM mode 1 |
| `murmure --llm-mode 2` | Switch to LLM mode 2 |
| `murmure --llm-mode 3` | Switch to LLM mode 3 |
| `murmure --llm-mode 4` | Switch to LLM mode 4 |

!!! warning "Push-to-talk limitation"
    OS custom shortcuts fire on key press, not on key release. This means only **toggle mode** is usable. Push-to-talk (hold to record, release to stop) cannot be implemented with OS custom shortcuts.

## GNOME

GNOME uses Mutter as its compositor. Mutter's XDG GlobalShortcuts portal is unreliable (latency, dropped events), so Murmure defaults to CLI mode on GNOME.

### Set up a custom shortcut on GNOME

1. Open **Settings > Keyboard > View and Customize Shortcuts > Custom Shortcuts**.
2. Click the **+** button to add a new shortcut.
3. Fill in:
   - **Name**: `Murmure transcription` (or any label you prefer)
   - **Command**: `murmure --transcription`
   - **Shortcut**: press the key combination you want (e.g. `Ctrl+Super+Space`)
4. Click **Add**.

Repeat for any other commands you want to bind (for example `murmure --paste-last` on a second shortcut).

### Verify Murmure is in the PATH

If GNOME does not find the `murmure` binary, the shortcut will silently fail. Check that the binary is in your PATH:

```bash
which murmure
```

If it is not found, use the full path in the Command field, for example `/usr/local/bin/murmure --transcription`.

## KDE Plasma 6

KDE Plasma 6 ships a working XDG GlobalShortcuts portal backend. Murmure defaults to **XDG Portal** on KDE, which is the recommended setup, no manual configuration needed.

### If you want to use CLI mode on KDE

Some power users prefer CLI mode for more control. To switch:

1. In Murmure, go to **Settings > System > Shortcut handling** and select **CLI**.
2. Restart Murmure.
3. Open **System Settings > Shortcuts > Custom Shortcuts**.
4. Click **Edit > New > Global Shortcut > Command/URL**.
5. In the **Trigger** tab, assign your key combination.
6. In the **Action** tab, set the command to `murmure --transcription`.
7. Apply and close.

## Hyprland

Add bindings to `~/.config/hypr/hyprland.conf`. Replace `SUPER, Y` with your preferred modifier and key.

```ini
# ~/.config/hypr/hyprland.conf

bind = SUPER, Y, exec, murmure --transcription
bind = SUPER SHIFT, Y, exec, murmure --paste-last
bind = SUPER ALT, Y, exec, murmure --cancel
```

Reload Hyprland to apply (`hyprctl reload` or log out and back in). Hyprland supports the XDG GlobalShortcuts portal as well, so you can keep Murmure in **XDG Portal** mode if you prefer to manage shortcuts from within Murmure's Settings UI.

## Sway

Add bindings to `~/.config/sway/config`. Replace `$mod+y` with your preferred combination.

```
# ~/.config/sway/config

bindsym $mod+y exec murmure --transcription
bindsym $mod+Shift+y exec murmure --paste-last
bindsym $mod+Control+y exec murmure --cancel
```

Reload Sway to apply (`swaymsg reload`). Like Hyprland, Sway supports the XDG portal, so you can also use **XDG Portal** mode and manage shortcuts from within Murmure.

## Troubleshooting

### Shortcut does nothing on GNOME

- Verify that `murmure` is in the PATH (run `which murmure` in a terminal).
- Make sure Murmure is already running in the background before pressing the shortcut. The CLI commands communicate with the running instance.
- Check that no other application has claimed the same key combination in GNOME Settings > Keyboard.

### The shortcut fires but nothing happens in Murmure

- Open a terminal and run `murmure --transcription` manually. If it says "no running instance found", Murmure is not started. Launch it first (it starts in the tray).
- If it runs without error but transcription does not start, check that Murmure is in **CLI** mode in Settings > System > Shortcut handling.

### Escape hatch: force XWayland

If you need XWayland for any reason (for example an older compositor with no portal support), you can start Murmure with the `GDK_BACKEND` environment variable:

```bash
GDK_BACKEND=x11 murmure
```

This is a GTK-standard variable. Murmure no longer sets it automatically since version 1.10.0. In XWayland mode, global shortcuts only fire when the Murmure window has focus.

### XDG Portal shortcuts work on Hyprland but not after a reboot

The portal session may not be registered at login time. Make sure `xdg-desktop-portal-hyprland` is installed and started. Check with:

```bash
systemctl --user status xdg-desktop-portal-hyprland
```
