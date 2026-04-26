# Linux Installation

!!! important "Requirements"
    - **X11 sessions** are fully supported.
    - **Wayland sessions** are supported in experimental mode.
        - **KDE Plasma 5.27+/6.x Wayland** is the recommended Wayland desktop for the smoothest experience.
        - **GNOME 48+ Wayland** is supported but currently immature: shortcuts may exhibit latency (tens to hundreds of ms) and occasional inconsistencies.
        - **Sway, Hyprland, and other Wayland compositors** may work depending on whether the compositor supports the `xdg-desktop-portal` GlobalShortcuts portal backend.

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

Murmure uses the `xdg-desktop-portal` GlobalShortcuts portal to register global shortcuts natively on Wayland. No XWayland or workaround is required by default.

On a Wayland session, a toggle **"Use Wayland portal for global shortcuts"** is available in **Settings > System**. It is enabled by default. If you disable it, Murmure restarts in XWayland mode and shortcuts will only work when the Murmure window has focus. Restart Murmure after changing this setting for it to take effect.

!!! note "Onboarding on Wayland"
    The first-run tutorial is replaced by a short notice: Wayland support is experimental, and your transcription is automatically copied to the clipboard so you can paste it with Ctrl+V anywhere.

### Desktop compatibility

| Desktop | Status |
| ------- | ------ |
| KDE Plasma 5.27+ / 6.x (Wayland) | Recommended. Global shortcuts work reliably via the portal. |
| GNOME 48+ (Wayland) | Supported but immature. The portal routes through Mutter RemoteDesktop, which adds variable latency and occasional dropped events. A persistent screen-sharing indicator appears in the top bar by GNOME design. |
| Sway, Hyprland and others | May work if the compositor ships a compatible portal backend. Not officially tested. |
| X11 (any desktop) | Fully supported, no changes. |

## Known Linux Issues

- **GNOME Wayland shortcuts**: Variable latency and inconsistencies are expected. See [Troubleshooting shortcuts on Linux Wayland](../troubleshooting/shortcuts.md#on-linux-wayland) for options.
- **xUbuntu**: "fast text entry is not possible on X11" warning from the Enigo library - this is cosmetic and can be ignored
- **Diacritics in Direct mode**: Some Linux configurations may not display accented characters correctly when using the "Direct (type text)" insertion mode

## Settings Location

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
