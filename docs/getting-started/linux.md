# Linux Installation

!!! important "Requirements"
    - **X11, Wayland and XWayland are supported.** On KDE Plasma Wayland, Murmure uses the `xdg-desktop-portal-kde` backend for true system-wide shortcuts. On other Wayland compositors (GNOME, Sway, Hyprland, etc.), the app auto-falls back to XWayland and shortcuts only fire while Murmure has focus — see [shortcut troubleshooting](../troubleshooting/shortcuts.md#on-linux-wayland). **KDE Plasma is recommended on Wayland** for the dictate-into-other-apps workflow.

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

## Known Linux Issues

- **Shortcut fails to register on KDE Wayland**: install `xdg-desktop-portal-kde` via your package manager. Most Plasma distros ship it by default.
- **Voice Mode "Submit" wake word**: not available under Wayland (keyboard injection into the focused window is blocked by the protocol). The toggle is disabled in Voice Mode settings when running on a Wayland session.
- **xUbuntu**: "fast text entry is not possible on X11" warning from the Enigo library - this is cosmetic and can be ignored
- **Diacritics in Direct mode**: Some Linux configurations may not display accented characters correctly when using the "Direct (type text)" insertion mode

## Settings Location

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
