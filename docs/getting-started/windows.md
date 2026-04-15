# Windows Installation

!!! important "Requirements"
    - **Windows 10 or later** - Older versions (Windows 8.1, 7) are not supported
    - **Visual C++ Redistributable** - Required runtime (see below)

## Installation Methods

=== "WinGet (recommended)"

    Open a terminal and run:

    ```powershell
    winget install Kieirra.Murmure
    ```

=== "MSI / EXE Installer"

    1. Download the latest installer from the [official website](https://murmure.al1x-ai.com/)
    2. Run `Murmure_x64.msi` (or `Murmure_x64-setup.exe`) and follow the setup wizard

    Alternatively, installers are available on the [GitHub Releases](https://github.com/Kieirra/murmure/releases) page.

=== "Silent Install (IT deployment)"

    For mass deployment on workstations:

    ```powershell
    msiexec /package Murmure_x64.msi /quiet
    ```

## Visual C++ Redistributable

Murmure requires the Microsoft Visual C++ Redistributable. Most computers already have it, but if you see this error:

> The code execution cannot proceed because MSVCP140.dll was not found.

Download and install it:

- [Direct download link (x64)](https://aka.ms/vs/17/release/vc_redist.x64.exe)
- [Official Microsoft page](https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist)

!!! note
    This cannot be bundled with the MSI installer due to technical limitations. The EXE installer would require admin rights to bundle it.

## Antivirus Notice

Some antivirus software may flag Murmure because it uses a global shortcut listener (`GetAsyncKeyState`).

**Kaspersky**: Add Murmure as an exclusion in your Kaspersky settings. The global shortcut listener is detected as suspicious by Kaspersky endpoint security.

**Windows Defender / SmartScreen**: Murmure binaries are code-signed via [SignPath.io](https://about.signpath.io/). If SmartScreen shows a warning, click "More info" then "Run anyway".

## Known Windows Issues

- **Sleep/hibernate disabled** - Windows may not go to sleep while Murmure is running. The global shortcut listener thread can keep the system awake. (Tracked in [#272](https://github.com/Kieirra/murmure/issues/272))

## Settings Location

Murmure stores its configuration at:

```
%APPDATA%\com.al1x-ai.murmure\settings.json
```

If you need to reset your settings, delete this file and restart Murmure.
