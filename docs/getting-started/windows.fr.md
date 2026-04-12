# Installation Windows

!!! important "Pre-requis" - **Windows 10 ou superieur** - Les anciennes versions (Windows 8.1, 7) ne sont pas supportees - **Visual C++ Redistributable** - Runtime necessaire (voir ci-dessous)

## Methodes d'installation

=== "WinGet (recommande)"

    Ouvrez un terminal et executez :

    ```powershell
    winget install Kieirra.Murmure
    ```

=== "MSI / EXE"

    1. Telechargez le dernier installateur depuis le [site officiel](https://murmure.al1x-ai.com/)
    2. Lancez `Murmure_x64.msi` (ou `Murmure_x64-setup.exe`) et suivez l'assistant

    Les installateurs sont aussi disponibles sur la page [GitHub Releases](https://github.com/Kieirra/murmure/releases).

=== "Installation silencieuse (deploiement IT)"

    Pour un deploiement en masse :

    ```powershell
    msiexec /package Murmure_x64.msi /quiet
    ```

## Visual C++ Redistributable

Murmure necessite le Microsoft Visual C++ Redistributable. La plupart des ordinateurs l'ont deja, mais si vous voyez cette erreur :

> The code execution cannot proceed because MSVCP140.dll was not found.

Telechargez et installez-le :

- [Lien de telechargement direct (x64)](https://aka.ms/vs/17/release/vc_redist.x64.exe)
- [Page officielle Microsoft](https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist)

!!! note
Ce composant ne peut pas etre inclus dans l'installateur MSI pour des raisons techniques.

## Avertissement antivirus

Certains antivirus peuvent bloquer Murmure car il utilise un ecouteur de raccourcis globaux (`GetAsyncKeyState`).

**Kaspersky** : Ajoutez Murmure en exclusion dans les parametres de Kaspersky.

**Windows Defender / SmartScreen** : Les binaires Murmure sont signes via [SignPath.io](https://about.signpath.io/). Si SmartScreen affiche un avertissement, cliquez sur "Plus d'informations" puis "Executer quand meme".

## Problemes connus sous Windows

- **Veille/hibernation desactivee** - Windows peut ne pas se mettre en veille tant que Murmure tourne. ([#272](https://github.com/Kieirra/murmure/issues/272))

## Emplacement des parametres

```
%APPDATA%\com.al1x-ai.murmure\settings.json
```

Pour reinitialiser, supprimez ce fichier et relancez Murmure.
