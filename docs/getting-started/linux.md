# Linux Installation

!!! important "Requirements"
    - **X11 sessions** are fully supported.
    - **Wayland sessions** are supported. Global shortcuts are bound at the OS level via custom shortcuts that call the `murmure` binary. You must configure them manually before using Murmure. Push-to-talk is not available on Wayland (only toggle mode). See [Configure shortcuts on Linux](../configure-shortcuts-on-linux.md).

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

=== "RPM Package (Fedora)"

    1. Download `Murmure_amd64.rpm` from [GitHub Releases](https://github.com/Kieirra/murmure/releases)
    2. Install:
    ```bash
    sudo rpm -i Murmure_amd64.rpm
    ```
    Or with dnf:
    ```bash
    sudo dnf install ./Murmure_amd64.rpm
    ```

    !!! note "Fedora 44 KDE Wayland"
        If you encounter a startup crash (`Could not create default EGL display: EGL_BAD_PARAMETER`), see the Known Issues section below.

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

Murmure runs natively on Wayland. Global shortcuts are bound at the OS level: you configure custom shortcuts in your desktop's keyboard settings (or compositor config) that call the `murmure` binary directly. This works on every compositor and survives reboots without any extra setup.

Only toggle mode is available on Wayland because OS custom shortcuts fire on key press only, not on key release.

!!! note "Onboarding on Wayland"
    On first launch on Wayland, a notice informs you that Wayland support is experimental and points you to Settings > Shortcuts where the available commands are listed.

### Configuring shortcuts

See [Configure shortcuts on Linux](../configure-shortcuts-on-linux.md) for step-by-step instructions for GNOME, KDE, Hyprland, and Sway.

### Forcing XWayland

Murmure no longer forces XWayland automatically. If you need it, set the `GDK_BACKEND` environment variable before launching:

```bash
GDK_BACKEND=x11 murmure
```

In XWayland mode, global shortcuts only fire when the Murmure window has focus.

## Known Linux Issues

- **Wayland shortcuts setup**: shortcuts must be bound at the OS level on every Wayland compositor. See [Configure shortcuts on Linux](../configure-shortcuts-on-linux.md) for per-desktop instructions.
- **Closing the window on Wayland**: the close button (X) may occasionally be unresponsive on Wayland, regardless of the compositor. Right-click the Murmure icon in the taskbar or dock and choose "Close" instead.
- **xUbuntu**: "fast text entry is not possible on X11" warning from the Enigo library - this is cosmetic and can be ignored
- **Diacritics in Direct mode**: Some Linux configurations may not display accented characters correctly when using the "Direct (type text)" insertion mode
- **Fedora 44 KDE Wayland startup crash** (`Could not create default EGL display: EGL_BAD_PARAMETER`): launch Murmure with `WEBKIT_DISABLE_COMPOSITING_MODE=1 murmure`. Note that this disables WebKit GPU acceleration, so the UI may feel slow and the window can freeze when moved. Upstream WebKit/Mesa issue awaiting fix.

## Settings Location

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
