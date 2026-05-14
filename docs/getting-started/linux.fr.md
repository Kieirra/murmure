# Installation Linux

!!! important "Pre-requis"
    - **Les sessions X11** sont entierement supportees.
    - **Les sessions Wayland** sont supportees. Deux modes de raccourcis sont disponibles selon votre environnement de bureau :
        - **KDE Plasma 5.27+/6.x, Hyprland, Sway** : les raccourcis globaux fonctionnent via XDG Portal sans configuration manuelle.
        - **GNOME 48+ Wayland** : Murmure passe en mode CLI par defaut. Vous devez configurer un raccourci OS manuellement avant d'utiliser Murmure. Le Push-to-talk n'est pas disponible en mode CLI (uniquement le mode toggle). Voir [Configurer les raccourcis sous Linux](../configure-shortcuts-on-linux.fr.md).
        - **Autres compositeurs** : peuvent fonctionner si le compositeur supporte le backend portal GlobalShortcuts de `xdg-desktop-portal`.

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

Murmure tourne nativement sous Wayland. Les raccourcis globaux sont gérés par l'un des deux modes suivants, configurable dans **Paramètres > Système > Gestion des raccourcis**. Un redémarrage est nécessaire après tout changement.

| Mode | Description |
| ---- | ----------- |
| **XDG Portal** | Murmure enregistre les raccourcis via l'interface GlobalShortcuts de `xdg-desktop-portal`. Fonctionne de façon fiable sur KDE Plasma 6, Hyprland et Sway. Push-to-talk et mode toggle sont disponibles. |
| **CLI** | Murmure n'enregistre aucun raccourci. Vous configurez des Custom Shortcuts OS qui appellent le binaire `murmure`. Par défaut sur GNOME car l'implémentation portal de Mutter est instable. Seul le mode toggle est disponible (les raccourcis OS se déclenchent à l'appui, pas au relâchement). |

### Defaults par bureau

| Bureau | Mode par défaut | Notes |
| ------ | --------------- | ----- |
| KDE Plasma 5.27+ / 6.x (Wayland) | XDG Portal | Recommandé. Les raccourcis globaux fonctionnent de façon fiable. |
| GNOME 48+ (Wayland) | CLI | Le portal Mutter est instable. Configurer des raccourcis personnalisés dans Paramètres > Clavier. |
| Hyprland (Wayland) | XDG Portal | Le portal fonctionne. Le mode CLI est aussi disponible via `bind` dans `hyprland.conf`. |
| Sway (Wayland) | XDG Portal | Le portal fonctionne. Le mode CLI est aussi disponible via `bindsym` dans `sway/config`. |
| X11 (tout bureau) | rdev | Entièrement supporté, aucune configuration Wayland nécessaire. |

!!! note "Onboarding sous Wayland"
    Au premier lancement sous Wayland, un message indique que le support Wayland est expérimental. Si le mode CLI est actif, le message inclut un lien vers Paramètres > Raccourcis où les commandes disponibles sont listées.

### Configurer les raccourcis en mode CLI

Voir [Configurer les raccourcis sous Linux](../configure-shortcuts-on-linux.fr.md) pour les instructions pas-à-pas pour GNOME, KDE, Hyprland et Sway.

### Forcer XWayland

Murmure ne force plus XWayland automatiquement. Si vous en avez besoin, définissez la variable d'environnement `GDK_BACKEND` avant le lancement :

```bash
GDK_BACKEND=x11 murmure
```

En mode XWayland, les raccourcis globaux ne se déclenchent que lorsque la fenêtre Murmure a le focus.

## Problèmes connus sous Linux

- **Raccourcis GNOME Wayland** : Murmure passe par défaut en mode CLI sur GNOME. Configurez un Custom Shortcut dans Paramètres GNOME > Clavier pointant vers `murmure --transcription`. Voir [Configurer les raccourcis sous Linux](../configure-shortcuts-on-linux.fr.md).
- **Fermeture de la fenêtre sous Wayland** : le bouton de fermeture (X) peut parfois ne pas répondre sous Wayland, quel que soit le compositeur ou le mode de raccourci. Faire un clic droit sur l'icône Murmure dans la barre des tâches ou le dock et choisir "Fermer".
- **xUbuntu** : Avertissement "fast text entry is not possible on X11" - cosmétique, ignorable
- **Diacritiques en mode Direct** : Certaines configurations Linux n'affichent pas correctement les caractères accentués en mode "Direct (saisie texte)"
- **Crash au démarrage sur Fedora 44 KDE Wayland** (`Could not create default EGL display: EGL_BAD_PARAMETER`) : lancer Murmure avec `WEBKIT_DISABLE_COMPOSITING_MODE=1 murmure`. Cette option désactive l'accélération GPU de WebKit, donc l'interface peut sembler lente et la fenêtre peut se figer lors de son déplacement. Bug upstream WebKit/Mesa en attente de correction.

## Emplacement des parametres

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
