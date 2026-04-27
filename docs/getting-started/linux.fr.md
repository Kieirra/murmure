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

Murmure peut router les raccourcis globaux soit via le portal GlobalShortcuts de `xdg-desktop-portal` (Wayland natif), soit via XWayland (rdev). Le mode se choisit dans **Parametres > Avance > Integration Wayland** et prend effet apres redemarrage de Murmure.

Valeurs par defaut :

- **KDE Plasma Wayland** : portal natif (fiable).
- **GNOME Wayland** : XWayland (le portal GNOME est instable, latence et evenements parfois perdus).
- **Sway, Hyprland et autres compositeurs** : portal natif (compatibilite selon le backend portal du compositeur).

!!! note "Onboarding sous Wayland"
    Le tutoriel de premiere utilisation est remplace par un court message qui depend du mode actif. En mode portal natif, le message recommande Voice Mode pour la fiabilite, en mode XWayland il rappelle que les raccourcis ne fonctionnent que lorsque Murmure a le focus.

### Utilisation de Murmure en mode XWayland

Lorsque **Integration Wayland** est sur **XWayland** (par defaut sur GNOME) :

- Les raccourcis globaux **ne se declenchent que lorsque la fenetre Murmure a le focus**. Pour declencher la transcription depuis une autre application, **utilisez le Voice Mode**, c'est le seul moyen de demarrer un enregistrement sans focus.
- Verifiez dans **Parametres > Avance > Copier la transcription dans le presse-papier** que l'option est activee (par defaut sur Wayland). La transcription reste dans le presse-papier pour pouvoir etre collee partout avec `Ctrl+V`.

### Compatibilite par bureau

| Bureau | Mode par defaut | Notes |
| ------ | ------ | ------ |
| KDE Plasma 5.27+ / 6.x (Wayland) | Portal natif | Recommande. Les raccourcis globaux fonctionnent de facon fiable. |
| GNOME 48+ (Wayland) | XWayland | Le portal natif est disponible via le toggle mais instable. Voice Mode recommande pour les mains libres. |
| Sway, Hyprland et autres (Wayland) | Portal natif | Depend du backend portal du compositeur. Basculez sur XWayland si les raccourcis ne s'enregistrent pas. |
| X11 (tout bureau) | rdev | Entierement supporte, aucun changement. |

## Problemes connus sous Linux

- **Raccourcis GNOME Wayland** : Latence variable et inconsistances attendues. Voir [Depannage raccourcis sous Linux Wayland](../troubleshooting/shortcuts.fr.md#sur-linux-wayland) pour les options disponibles.
- **Modification d'un raccourci (mode portal natif, GNOME uniquement)** : la capture d'un nouveau raccourci depuis les Parametres ne detecte pas les touches pressees. Basculer sur XWayland dans Parametres > Avance > Integration Wayland pour pouvoir modifier un raccourci.
- **Fermeture de la fenetre (mode portal natif, GNOME uniquement)** : le bouton de fermeture (X) peut ne pas repondre. Faire un clic droit sur l'icone dans la barre des taches ou le dock et choisir "Fermer".
- **xUbuntu** : Avertissement "fast text entry is not possible on X11" - cosmetique, ignorable
- **Diacritiques en mode Direct** : Certaines configurations Linux n'affichent pas correctement les caracteres accentues en mode "Direct (saisie texte)"

## Emplacement des parametres

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
