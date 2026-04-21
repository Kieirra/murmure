# Problemes de raccourcis

## Le raccourci ne fonctionne pas

### Sur macOS

**Verifiez les permissions** : Reglages Systeme > Confidentialite et securite > **Accessibilite** et **Surveillance des entrees**.

**Apres mise a jour depuis 1.6.0** : Reinitialisation complete des permissions necessaire. Voir le [guide d'installation macOS](../getting-started/macos.md#mise-a-jour-depuis-160).

**Conflit raccourci systeme** : `Ctrl+Espace` est utilise par macOS pour le changement de source d'entree. Changez votre raccourci (ex: `Ctrl+Option+M`, `F2`).

**"Echec de sauvegarde du raccourci"** : Une autre application utilise deja ce raccourci.

### Sur Linux (Wayland)

Sous Wayland, les raccourcis globaux passent par le **portail GlobalShortcuts** fourni par `xdg-desktop-portal`. La plupart des bureaux Linux modernes (GNOME 48+, KDE Plasma 6.x, Hyprland) embarquent un backend de portail par defaut, Murmure fonctionne donc sans configuration supplementaire.

Si Murmure indique qu'il n'a pas pu enregistrer un raccourci, votre bureau n'embarque probablement pas de backend de portail (cas des distributions anciennes ou minimales). Vous pouvez soit basculer sur une session X11, soit tenter d'installer un backend `xdg-desktop-portal` via votre gestionnaire de paquets (sans garantie que cela fonctionne sur votre distribution).

**Le raccourci d'annulation d'enregistrement n'est pas disponible sur Wayland** — le portail capturerait la touche au niveau systeme. Utilisez `Ctrl+Z` pour annuler un collage, ou le raccourci *Coller la derniere transcription*.

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
