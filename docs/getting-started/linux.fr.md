# Installation Linux

!!! important "Pre-requis"
    - **Les sessions X11** sont entierement supportees.
    - **Les sessions Wayland** sont supportees. Les raccourcis globaux sont configures au niveau OS via des raccourcis personnalises qui appellent le binaire `murmure`. Vous devez les configurer manuellement avant d'utiliser Murmure. Le Push-to-talk n'est pas disponible sous Wayland (uniquement le mode toggle). Voir [Configurer les raccourcis sous Linux](../configure-shortcuts-on-linux.fr.md).

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

=== "Paquet RPM (Fedora)"

    1. Telechargez `Murmure_amd64.rpm` depuis [GitHub Releases](https://github.com/Kieirra/murmure/releases)
    2. Installez :
    ```bash
    sudo rpm -i Murmure_amd64.rpm
    ```
    Ou avec dnf :
    ```bash
    sudo dnf install ./Murmure_amd64.rpm
    ```

    !!! note "Fedora 44 KDE Wayland"
        En cas de crash au demarrage (`Could not create default EGL display: EGL_BAD_PARAMETER`), consultez la section Problemes connus ci-dessous.

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

Murmure tourne nativement sous Wayland. Les raccourcis globaux sont configurés au niveau OS : vous créez des raccourcis personnalisés dans les paramètres clavier de votre bureau (ou la config de votre compositeur) qui appellent directement le binaire `murmure`. Cette approche fonctionne sur tous les compositeurs et survit aux redémarrages sans configuration supplémentaire.

Seul le mode toggle est disponible sous Wayland car les raccourcis personnalisés OS se déclenchent à l'appui de la touche, pas au relâchement.

!!! note "Onboarding sous Wayland"
    Au premier lancement sous Wayland, un message indique que le support Wayland est expérimental et vous oriente vers Paramètres > Raccourcis où les commandes disponibles sont listées.

### Configurer les raccourcis

Voir [Configurer les raccourcis sous Linux](../configure-shortcuts-on-linux.fr.md) pour les instructions pas-à-pas pour GNOME, KDE, Hyprland et Sway.

### Forcer XWayland

Murmure ne force plus XWayland automatiquement. Si vous en avez besoin, définissez la variable d'environnement `GDK_BACKEND` avant le lancement :

```bash
GDK_BACKEND=x11 murmure
```

En mode XWayland, les raccourcis globaux ne se déclenchent que lorsque la fenêtre Murmure a le focus.

## Problèmes connus sous Linux

- **Configuration des raccourcis Wayland** : sur tous les compositeurs Wayland, les raccourcis doivent être configurés au niveau OS. Voir [Configurer les raccourcis sous Linux](../configure-shortcuts-on-linux.fr.md) pour les instructions par bureau.
- **Fermeture de la fenêtre sous Wayland** : le bouton de fermeture (X) peut parfois ne pas répondre sous Wayland, quel que soit le compositeur. Faire un clic droit sur l'icône Murmure dans la barre des tâches ou le dock et choisir "Fermer".
- **xUbuntu** : Avertissement "fast text entry is not possible on X11" - cosmétique, ignorable
- **Diacritiques en mode Direct** : Certaines configurations Linux n'affichent pas correctement les caractères accentués en mode "Direct (saisie texte)"
- **Crash au démarrage sur Fedora 44 KDE Wayland** (`Could not create default EGL display: EGL_BAD_PARAMETER`) : lancer Murmure avec `WEBKIT_DISABLE_COMPOSITING_MODE=1 murmure`. Cette option désactive l'accélération GPU de WebKit, donc l'interface peut sembler lente et la fenêtre peut se figer lors de son déplacement. Bug upstream WebKit/Mesa en attente de correction.

## Emplacement des parametres

```
~/.local/share/com.al1x-ai.murmure/settings.json
```
