# Linux Installation

!!! important "Requirements"
    - **Both X11 and Wayland sessions are supported.** Wayland requires an `xdg-desktop-portal` backend on your desktop, which most modern Linux desktops (GNOME 48+, KDE Plasma 6.x, Hyprland) ship by default.

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

- **Wayland global shortcuts**: if Murmure reports it couldn't register a shortcut, your desktop is likely missing an `xdg-desktop-portal` backend. Switch to an X11 session or install the backend via your package manager.
- **Voice Mode "Submit" wake word**: not available under Wayland (keyboard injection into the focused window is blocked by the protocol). The toggle is disabled in Voice Mode settings when running on a Wayland session.
- **xUbuntu**: "fast text entry is not possible on X11" warning from the Enigo library - this is cosmetic and can be ignored
- **Diacritics in Direct mode**: Some Linux configurations may not display accented characters correctly when using the "Direct (type text)" insertion mode

## Settings Location

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
