# Linux Installation

!!! important "Requirements"
    - **X11 sessions** are fully supported.
    - **Wayland sessions** are supported. Two shortcut modes are available depending on your desktop environment:
        - **KDE Plasma 5.27+/6.x, Hyprland, Sway**: global shortcuts work via XDG Portal with no manual configuration.
        - **GNOME 48+ Wayland**: Murmure defaults to CLI mode. You must configure a custom OS shortcut manually before using Murmure. Push-to-talk is not available in CLI mode (only toggle mode). See [Configure shortcuts on Linux](../configure-shortcuts-on-linux.md).
        - **Other compositors**: may work if the compositor supports the `xdg-desktop-portal` GlobalShortcuts portal backend.

## Installation Methods

=== "Quick Install (Debian-based)"

    Open a terminal and run:

    ```bash
    curl -fsSL https://raw.githubusercontent.com/Kieirra/murmure/main/install.sh | sh
    ```

    This script downloads and installs the latest version of Murmure for your system.

    !!! warning "Use native curl"
        If you have `curl` installed via Snap, this may fail due to Snap sandboxing. Use the system `curl` from apt instead: `sudo apt install curl`

=== "DEB Package"

    1. Download `Murmure_amd64.deb` from the [official website](https://murmure.al1x-ai.com/) (or [GitHub Releases](https://github.com/Kieirra/murmure/releases))
    2. Install:
    ```bash
    sudo dpkg -i Murmure_amd64.deb
    ```

    !!! note "GLIBC compatibility"
        The `.deb` package is built on Ubuntu 24.04 and requires GLIBC 2.38+. If you're on Ubuntu 22.04 or older, use the AppImage instead.

=== "AppImage"

    1. Download `Murmure_amd64.AppImage` from the [official website](https://murmure.al1x-ai.com/) (or [GitHub Releases](https://github.com/Kieirra/murmure/releases))
    2. Make it executable:
    ```bash
    chmod +x Murmure_amd64.AppImage
    ```
    3. Run it:
    ```bash
    ./Murmure_amd64.AppImage
    ```

    For Ubuntu 22.04, a legacy AppImage build (`Murmure_amd64.legacy.AppImage`) may be available in some releases.

## Wayland

Murmure runs natively on Wayland. Global shortcuts are handled by one of two modes, configurable in **Settings > System > Shortcut handling**. A restart is required after changing the mode.

| Mode | Description |
| ---- | ----------- |
| **XDG Portal** | Murmure registers shortcuts through the `xdg-desktop-portal` GlobalShortcuts interface. Works reliably on KDE Plasma 6, Hyprland, and Sway. Both Push-to-talk and toggle mode are available. |
| **CLI** | Murmure registers no shortcuts. You bind OS-level custom shortcuts that call the `murmure` binary. Default on GNOME because Mutter's portal implementation is unreliable. Only toggle mode is available (OS shortcuts fire on key press only, not on release). |

### Desktop defaults

| Desktop | Default mode | Notes |
| ------- | ------------ | ----- |
| KDE Plasma 5.27+ / 6.x (Wayland) | XDG Portal | Recommended. Global shortcuts work reliably. |
| GNOME 48+ (Wayland) | CLI | Mutter portal is unstable. Configure custom shortcuts in Settings > Keyboard. |
| Hyprland (Wayland) | XDG Portal | Portal works. CLI also available via `bind` in `hyprland.conf`. |
| Sway (Wayland) | XDG Portal | Portal works. CLI also available via `bindsym` in `sway/config`. |
| X11 (any desktop) | rdev | Fully supported, no Wayland-specific configuration needed. |

!!! note "Onboarding on Wayland"
    On first launch on Wayland, a notice informs you that Wayland support is experimental. If CLI mode is active, the notice includes a link to Settings > Shortcuts where the available commands are listed.

### Configuring shortcuts in CLI mode

See [Configure shortcuts on Linux](../configure-shortcuts-on-linux.md) for step-by-step instructions for GNOME, KDE, Hyprland, and Sway.

### Forcing XWayland

Murmure no longer forces XWayland automatically. If you need it, set the `GDK_BACKEND` environment variable before launching:

```bash
GDK_BACKEND=x11 murmure
```

In XWayland mode, global shortcuts only fire when the Murmure window has focus.

## Known Linux Issues

- **GNOME Wayland shortcuts**: Murmure defaults to CLI mode on GNOME. Configure a Custom Shortcut in GNOME Settings > Keyboard pointing to `murmure --transcription`. See [Configure shortcuts on Linux](../configure-shortcuts-on-linux.md).
- **Closing the window on Wayland**: the close button (X) may occasionally be unresponsive on Wayland, regardless of the compositor or shortcut mode. Right-click the Murmure icon in the taskbar or dock and choose "Close" instead.
- **xUbuntu**: "fast text entry is not possible on X11" warning from the Enigo library - this is cosmetic and can be ignored
- **Diacritics in Direct mode**: Some Linux configurations may not display accented characters correctly when using the "Direct (type text)" insertion mode
- **Fedora 44 KDE Wayland startup crash** (`Could not create default EGL display: EGL_BAD_PARAMETER`): launch Murmure with `WEBKIT_DISABLE_COMPOSITING_MODE=1 murmure`. Note that this disables WebKit GPU acceleration, so the UI may feel slow and the window can freeze when moved. Upstream WebKit/Mesa issue awaiting fix.

## Settings Location

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
