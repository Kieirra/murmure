# Problemes de raccourcis

## Le raccourci ne fonctionne pas

### Sur macOS

**Verifiez les permissions** : Reglages Systeme > Confidentialite et securite > **Accessibilite** et **Surveillance des entrees**.

**Apres mise a jour depuis 1.6.0** : Reinitialisation complete des permissions necessaire. Voir le [guide d'installation macOS](../getting-started/macos.md#mise-a-jour-depuis-160).

**Conflit raccourci systeme** : `Ctrl+Espace` est utilise par macOS pour le changement de source d'entree. Changez votre raccourci (ex: `Ctrl+Option+M`, `F2`).

**"Echec de sauvegarde du raccourci"** : Une autre application utilise deja ce raccourci.

### Sur Linux (Wayland)

Sur **KDE Plasma Wayland**, les raccourcis globaux passent par le backend `xdg-desktop-portal-kde` — aucune configuration requise, comportement identique a X11 (raccourcis fire depuis n'importe quelle app).

Sur les **autres compositeurs Wayland** (GNOME, Sway, Hyprland, etc.), Murmure bascule automatiquement sur XWayland et capture les raccourcis via rdev. Dans ce mode, les raccourcis ne declenchent que lorsque la fenetre Murmure a le focus clavier — pressez le raccourci apres avoir clique sur Murmure, puis basculez vers votre application cible pour le collage. C'est une limitation de XWayland (les apps non privilegiees ne peuvent pas capturer les touches au niveau systeme), pas de Murmure.

**Consequence** : Murmure est nettement moins fluide sur un bureau base sur GNOME (ou Sway, Hyprland, etc.) que sur un bureau base sur KDE. Pour l'usage dictee-vers-application-tierce sans basculer le focus, KDE Plasma reste recommande jusqu'a ce que les autres compositeurs stabilisent leur impl du portail `GlobalShortcuts`.

**Le raccourci d'annulation d'enregistrement n'est pas disponible sur KDE Wayland** — le portail capturerait la touche au niveau systeme. Utilisez `Ctrl+Z` pour annuler un collage, ou le raccourci *Coller la derniere transcription*.

**Uniquement sur les sessions Wayland non-KDE** (GNOME, Sway, Hyprland, etc.), l'UI peut se figer progressivement apres un usage prolonge. Un bouton **Rafraichir la fenetre** apparait dans le bas de la barre laterale dans ce mode — cliquez dessus pour restaurer le rendu. L'enregistrement et le collage continuent de fonctionner pendant le freeze. X11 et KDE Wayland ne sont pas affectes.

### Sur Windows

1. Verifiez qu'aucune autre application n'utilise le meme raccourci
2. Verifiez que votre antivirus (surtout Kaspersky) ne bloque pas l'ecouteur de raccourcis
3. Essayez de lancer Murmure en administrateur (test temporaire uniquement)

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
| **Linux (Wayland)** | `Ctrl+Espace`, `F2`, `Ctrl+Alt+M`          | -                         |
