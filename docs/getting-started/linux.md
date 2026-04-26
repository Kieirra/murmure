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

Murmure can route global shortcuts either through the `xdg-desktop-portal` GlobalShortcuts portal (Wayland native) or through XWayland (rdev). The mode is selected in **Settings > Advanced > Wayland integration** and takes effect after restarting Murmure.

Defaults:

- **KDE Plasma Wayland**: native portal (works reliably).
- **GNOME Wayland**: XWayland (the GNOME portal is unstable, latency and dropped events).
- **Sway, Hyprland and other compositors**: native portal (compatibility depends on the compositor's portal backend).

!!! note "Onboarding on Wayland"
    The first-run tutorial is replaced by a short notice depending on the active mode. Native portal mode points to Voice Mode for reliability, XWayland mode emphasizes that shortcuts only work when Murmure is focused.

### Using Murmure in XWayland mode

When **Wayland integration** is set to **XWayland** (default on GNOME):

- Global shortcuts **only fire when the Murmure window is focused**. To trigger transcription while another app is in use, **use Voice Mode**, it is the only way to start a recording hands-free.
- Verify in **Settings > Advanced > Copy transcription to clipboard** that the option is enabled (default on Wayland). The transcription stays in the clipboard so you can paste it anywhere with `Ctrl+V`.

### Desktop compatibility

| Desktop | Default mode | Notes |
| ------- | ------ | ------ |
| KDE Plasma 5.27+ / 6.x (Wayland) | Native portal | Recommended. Global shortcuts work reliably. |
| GNOME 48+ (Wayland) | XWayland | Native portal available via the toggle but unstable. Voice Mode recommended for hands-free use. |
| Sway, Hyprland and others (Wayland) | Native portal | Depends on the compositor's portal backend. Switch to XWayland if shortcuts do not register. |
| X11 (any desktop) | rdev | Fully supported, no changes. |

## Known Linux Issues

- **GNOME Wayland shortcuts**: Variable latency and inconsistencies are expected. See [Troubleshooting shortcuts on Linux Wayland](../troubleshooting/shortcuts.md#on-linux-wayland) for options.
- **xUbuntu**: "fast text entry is not possible on X11" warning from the Enigo library - this is cosmetic and can be ignored
- **Diacritics in Direct mode**: Some Linux configurations may not display accented characters correctly when using the "Direct (type text)" insertion mode

## Settings Location

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
