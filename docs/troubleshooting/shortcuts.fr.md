# Problemes de raccourcis

## Le raccourci ne fonctionne pas

### Sur macOS

**Verifiez les permissions** : Reglages Systeme > Confidentialite et securite > **Accessibilite** et **Surveillance des entrees**.

**Apres mise a jour depuis 1.6.0** : Reinitialisation complete des permissions necessaire. Voir le [guide d'installation macOS](../getting-started/macos.md#mise-a-jour-depuis-160).

**Conflit raccourci systeme** : `Ctrl+Espace` est utilise par macOS pour le changement de source d'entree. Changez votre raccourci (ex: `Ctrl+Option+M`, `F2`).

**"Echec de sauvegarde du raccourci"** : Une autre application utilise deja ce raccourci.

### Sur Linux (Wayland)

Murmure expose un reglage **Integration Wayland** dans **Parametres > Avance** avec deux modes : portal natif (`xdg-desktop-portal` GlobalShortcuts) ou XWayland (rdev). Le mode est choisi automatiquement selon le bureau et peut etre modifie manuellement. Redemarrez Murmure apres tout changement.

**KDE Plasma 5.27+/6.x** (par defaut : portal natif) : les raccourcis fonctionnent de facon fiable. Si un raccourci ne se declenche pas, verifiez qu'aucune autre application ne l'a deja revendique.

**GNOME 48+** (par defaut : XWayland) : le portal GNOME route les raccourcis via Mutter RemoteDesktop, avec une latence variable (dizaines a centaines de millisecondes) et des evenements parfois perdus. Nous mettons XWayland par defaut sur GNOME pour la fiabilite. En mode XWayland, **les raccourcis globaux ne se declenchent que lorsque la fenetre Murmure a le focus**. Pour enregistrer sans focus, utilisez le **Voice Mode**, et verifiez que **Parametres > Avance > Copier la transcription dans le presse-papier** reste active pour pouvoir coller avec `Ctrl+V`.

**Sway, Hyprland et autres compositeurs** (par defaut : portal natif) : le comportement depend du backend portal disponible sur votre systeme. Si les raccourcis ne s'enregistrent pas, basculez en mode XWayland dans les Parametres.

### Sur Windows

1. Verifiez qu'aucune autre application n'utilise le meme raccourci
2. Verifiez que votre antivirus (surtout Kaspersky) ne bloque pas l'ecouteur de raccourcis
3. Essayez de lancer Murmure en administrateur (test temporaire uniquement)

## Le raccourci bascule rapidement (Linux)

Sous Linux, maintenir un raccourci en mode Push-to-talk peut basculer l'enregistrement on/off tres rapidement.

**Cause** : X11 envoie des evenements de repetition automatique tant que la touche est maintenue. Les portals Wayland peuvent aussi emettre des rafales d'evenements pour une seule pression.

**Solution** : Un mecanisme de cooldown interne gere ce cas depuis la version 1.9.0. Si le probleme persiste, verifiez que vous utilisez la derniere version.

## Touches F13-F24 non reconnues

Le support des touches de fonction etendues (F13-F24), touches du pave numerique et touches OEM a ete ajoute en version **1.8.0**.

## Boutons de souris

Les raccourcis par bouton de souris sont supportes depuis la v1.8.0.

## Raccourcis recommandes par OS

| OS                  | Recommande                                 | A eviter                  |
| ------------------- | ------------------------------------------ | ------------------------- |
| **Windows**         | `Ctrl+Espace`, `Ctrl+Alt+M`, `F2`          | AltGr (= Ctrl+Alt)        |
| **macOS**           | `Ctrl+Option+M`, `F2`, `F3`, bouton souris | Espace, chiffres, lettres |
| **Linux (X11)**     | `Ctrl+Espace`, `F2`, `Ctrl+Alt+M`          | -                         |
| **Linux (Wayland)** | `Ctrl+Shift+Espace`, `F2`, bouton souris   | -                         |
