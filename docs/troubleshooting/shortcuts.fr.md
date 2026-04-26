# Problemes de raccourcis

## Le raccourci ne fonctionne pas

### Sur macOS

**Verifiez les permissions** : Reglages Systeme > Confidentialite et securite > **Accessibilite** et **Surveillance des entrees**.

**Apres mise a jour depuis 1.6.0** : Reinitialisation complete des permissions necessaire. Voir le [guide d'installation macOS](../getting-started/macos.md#mise-a-jour-depuis-160).

**Conflit raccourci systeme** : `Ctrl+Espace` est utilise par macOS pour le changement de source d'entree. Changez votre raccourci (ex: `Ctrl+Option+M`, `F2`).

**"Echec de sauvegarde du raccourci"** : Une autre application utilise deja ce raccourci.

### Sur Linux (Wayland)

Murmure utilise le portal GlobalShortcuts de `xdg-desktop-portal` sur Wayland. Les raccourcis globaux sont enregistres nativement sans necessiter de session X11.

**KDE Plasma 5.27+/6.x** : les raccourcis fonctionnent de facon fiable. Si un raccourci ne se declenche pas, verifiez qu'aucune autre application ne l'a deja revendique.

**GNOME 48+** : le portal route les raccourcis via Mutter RemoteDesktop, ce qui introduit une latence variable (dizaines a centaines de millisecondes) et des evenements parfois perdus. Un indicateur de partage d'ecran persistant apparait egalement dans la barre superieure pendant que Murmure fonctionne. Il s'agit d'une limitation connue de l'implementation du portal GNOME, pas d'un bug Murmure.

Si le comportement GNOME est trop inconsistant, vous pouvez basculer en mode XWayland : allez dans **Parametres > Systeme**, desactivez **"Use Wayland portal for global shortcuts"** et redemarrez Murmure. En mode XWayland, les raccourcis ne fonctionnent que lorsque la fenetre Murmure est au premier plan.

**Sway, Hyprland et autres compositeurs** : le comportement depend du backend portal disponible sur votre systeme. Si les raccourcis ne fonctionnent pas du tout, essayez le fallback XWayland decrit ci-dessus.

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
