# Installation Linux

!!! important "Pre-requis"
    - **X11, Wayland et XWayland sont supportes.** Sur KDE Plasma Wayland, Murmure utilise le backend `xdg-desktop-portal-kde` pour des raccourcis globaux systeme. Sur les autres compositeurs Wayland (GNOME, Sway, Hyprland, etc.), l'app bascule automatiquement sur XWayland et les raccourcis ne declenchent que lorsque Murmure a le focus — voir [depannage des raccourcis](../troubleshooting/shortcuts.md#sur-linux-wayland). **KDE Plasma est recommande sous Wayland** pour l'usage dictee-vers-autres-apps.

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

- **Echec d'enregistrement de raccourci sous KDE Wayland** : installez `xdg-desktop-portal-kde` via votre gestionnaire de paquets. La plupart des distributions Plasma le fournissent par defaut.
- **Mot declencheur "Submit" du Voice Mode** : non disponible sous Wayland (l'injection clavier dans la fenetre focalisee est bloquee par le protocole). Le toggle est desactive dans les parametres du Voice Mode lors d'une session Wayland.
- **xUbuntu** : Avertissement "fast text entry is not possible on X11" - cosmetique, ignorable
- **Diacritiques en mode Direct** : Certaines configurations Linux n'affichent pas correctement les caracteres accentues en mode "Direct (saisie texte)"

## Emplacement des parametres

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
