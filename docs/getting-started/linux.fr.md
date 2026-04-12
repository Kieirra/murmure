# Installation Linux

!!! important "Pre-requis" - **Session X11 obligatoire** - Wayland n'est pas supporte pour le moment (la compatibilite Wayland est en cours de developpement pour la version 1.9.0)

## Methodes d'installation

=== "Installation rapide (Debian)"

    Ouvrez un terminal et executez :

    ```bash
    curl -fsSL https://raw.githubusercontent.com/Kieirra/murmure/main/install.sh | sh
    ```

    Ce script telecharge et installe la derniere version de Murmure pour votre systeme.

    !!! warning "Utilisez curl natif"
        Si `curl` est installe via Snap, cela peut echouer a cause du sandboxing. Utilisez le `curl` systeme : `sudo apt install curl`

=== "Paquet DEB"

    1. Telechargez `Murmure_amd64.deb` depuis le [site officiel](https://murmure.al1x-ai.com/) (ou [GitHub Releases](https://github.com/Kieirra/murmure/releases))
    2. Installez :
    ```bash
    sudo dpkg -i Murmure_amd64.deb
    ```

    !!! note "Compatibilite GLIBC"
        Le paquet `.deb` est compile sur Ubuntu 24.04 et necessite GLIBC 2.38+. Pour Ubuntu 22.04 ou plus ancien, utilisez l'AppImage.

=== "AppImage"

    1. Telechargez `Murmure_amd64.AppImage` depuis le [site officiel](https://murmure.al1x-ai.com/) (ou [GitHub Releases](https://github.com/Kieirra/murmure/releases))
    2. Rendez-le executable :
    ```bash
    chmod +x Murmure_amd64.AppImage
    ```
    3. Lancez-le :
    ```bash
    ./Murmure_amd64.AppImage
    ```

## Problemes connus sous Linux

- **Wayland** : Non supporte. Murmure necessite une session X11. La compatibilite Wayland est en cours de developpement pour la version 1.9.0.
- **xUbuntu** : Avertissement "fast text entry is not possible on X11" - cosmetique, ignorable
- **Diacritiques en mode Direct** : Certaines configurations Linux n'affichent pas correctement les caracteres accentues en mode "Direct (saisie texte)"

## Emplacement des parametres

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
