# Installation Linux

!!! important "Pre-requis"
    - **Les sessions X11** sont entierement supportees.
    - **Les sessions Wayland** sont supportees en mode experimental.
        - **KDE Plasma 5.27+/6.x Wayland** est le bureau Wayland recommande pour la meilleure experience.
        - **GNOME 48+ Wayland** est supporte mais encore immature : les raccourcis peuvent presenter une latence variable (dizaines a centaines de ms) et des inconsistances ponctuelles.
        - **Sway, Hyprland et autres compositeurs Wayland** peuvent fonctionner selon que le compositeur integre un backend portal GlobalShortcuts compatible avec `xdg-desktop-portal`.

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

## Wayland

Murmure utilise le portal GlobalShortcuts de `xdg-desktop-portal` pour enregistrer les raccourcis globaux nativement sous Wayland. Aucun mode XWayland ni contournement n'est necessaire par defaut.

Sur une session Wayland, un toggle **"Use Wayland portal for global shortcuts"** est disponible dans **Parametres > Systeme**. Il est active par defaut. Si vous le desactivez, Murmure redemarrera en mode XWayland et les raccourcis ne fonctionneront que lorsque la fenetre Murmure est au premier plan. Un redemarrage de Murmure est necessaire apres avoir modifie ce reglage.

!!! note "Onboarding sous Wayland"
    Le tutoriel de premiere utilisation est remplace par un court message : le support Wayland est experimental, et la transcription est automatiquement copiee dans le presse-papier pour etre collee avec Ctrl+V n'importe ou.

### Compatibilite par bureau

| Bureau | Statut |
| ------ | ------ |
| KDE Plasma 5.27+ / 6.x (Wayland) | Recommande. Les raccourcis globaux fonctionnent de facon fiable via le portal. |
| GNOME 48+ (Wayland) | Supporte mais immature. Le portal passe par Mutter RemoteDesktop, ce qui ajoute une latence variable et des evenements parfois perdus. Un indicateur de partage d'ecran persistant apparait dans la barre superieure, par design GNOME. |
| Sway, Hyprland et autres | Peut fonctionner si le compositeur integre un backend portal compatible. Non teste officiellement. |
| X11 (tout bureau) | Entierement supporte, aucun changement. |

## Problemes connus sous Linux

- **Raccourcis GNOME Wayland** : Latence variable et inconsistances attendues. Voir [Depannage raccourcis sous Linux Wayland](../troubleshooting/shortcuts.fr.md#sur-linux-wayland) pour les options disponibles.
- **xUbuntu** : Avertissement "fast text entry is not possible on X11" - cosmetique, ignorable
- **Diacritiques en mode Direct** : Certaines configurations Linux n'affichent pas correctement les caracteres accentues en mode "Direct (saisie texte)"

## Emplacement des parametres

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
