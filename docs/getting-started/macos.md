# macOS Installation

## Download

=== "Apple Silicon (M1/M2/M3/M4)"

    1. Download **Murmure_aarch64_darwin.dmg** from the [official website](https://murmure.al1x-ai.com/) (or [GitHub Releases](https://github.com/Kieirra/murmure/releases))
    2. Open the DMG and drag Murmure to your Applications folder
    3. Open Murmure from Applications

=== "Intel"

    1. Download **Murmure_x86_64_darwin.dmg** from the [official website](https://murmure.al1x-ai.com/) (or [GitHub Releases](https://github.com/Kieirra/murmure/releases))
    2. Open the DMG and drag Murmure to your Applications folder
    3. Open Murmure from Applications

## Required Permissions

Murmure needs three macOS permissions to function:

1. **Microphone** - To capture your voice
2. **Accessibility** - To simulate keyboard input and paste text
3. **Input Monitoring** - To detect keyboard shortcuts and prevent them from being sent to the active application

On first launch, macOS will automatically ask you to grant these permissions. After granting them, **restart Murmure** for the permissions to take effect.

!!! tip "Something not working?"
If shortcuts or transcription don't work, you may have accidentally declined a permission. Go to **System Settings** > **Privacy & Security** and check that Murmure is listed and enabled under:

    - Microphone
    - Accessibility
    - Input Monitoring

## Upgrading from 1.6.0

!!! warning "Important: Permission reset required"
If you're upgrading from version 1.6.0, the code signature changed between versions. macOS treats them as different applications, so you must completely reset permissions.

Follow these steps **in this exact order**:

1. **Remove** Murmure from System Settings > Privacy & Security > **Accessibility** (not just toggle off - remove it from the list entirely)
2. **Remove** Murmure from System Settings > Privacy & Security > **Input Monitoring**
3. **Install** the new version
4. **Launch** Murmure
5. **Grant** the Accessibility permission
6. **Grant** the Input Monitoring permission
7. **Restart** Murmure

This is a one-time procedure. Future updates will not require this.

## Recommended Shortcuts

The default shortcut `Ctrl+Space` conflicts with the macOS input source switcher. We recommend changing it to one of:

- `Ctrl+Option+M`
- `F2`, `F3`, or another function key
- A mouse button (if available)

!!! warning "Avoid shortcuts with Space or number keys"
On macOS, shortcuts containing `Space` or number keys may leak those characters into the active application. For example, `Shift+Space` will produce multiple space characters while held. Use modifier-only combos or function keys instead.

## macOS-Specific Notes

- **Developer name in notifications**: macOS may show the developer's personal name instead of "Murmure" in background app notifications. This is a macOS limitation for individual developer certificates.
- **Dock visibility**: You can hide Murmure from the Dock in Settings > System > "Show in Dock".
- **macOS Catalina (10.15)**: If the app doesn't appear in Privacy & Security, try manually browsing to it using the "+" button in the permissions list.

## Settings Location

```
~/Library/Application Support/com.al1x-ai.murmure/settings.json
```
