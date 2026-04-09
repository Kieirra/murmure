# Problemes de raccourcis

## Le raccourci ne fonctionne pas

### Sur macOS

**Verifiez les permissions** : Reglages Systeme > Confidentialite et securite > **Accessibilite** et **Surveillance des entrees**.

**Apres mise a jour depuis 1.6.0** : Reinitialisation complete des permissions necessaire. Voir le [guide d'installation macOS](../getting-started/macos.md#mise-a-jour-depuis-160).

**Conflit raccourci systeme** : `Ctrl+Espace` est utilise par macOS pour le changement de source d'entree. Changez votre raccourci (ex: `Ctrl+Option+M`, `F2`).

**"Echec de sauvegarde du raccourci"** : Une autre application utilise deja ce raccourci.

### Sur Linux (Wayland)

Les raccourcis globaux ne fonctionnent pas sous Wayland. Le raccourci ne marche que quand Murmure est au premier plan.

**Solution** : Passez a une session X11. Sur l'ecran de connexion, selectionnez "GNOME sur Xorg" ou "Plasma (X11)".

Le support Wayland pour GNOME 48+ est prevu pour la version 1.9.0.

### Sur Windows

1. Verifiez qu'aucune autre application n'utilise le meme raccourci
2. Verifiez que votre antivirus (surtout Kaspersky) ne bloque pas l'ecouteur de raccourcis
3. Essayez de lancer Murmure en administrateur (test temporaire uniquement)

## Le raccourci bascule rapidement (Linux)

Sur Ubuntu 24.04 Wayland, maintenir le raccourci peut basculer l'enregistrement on/off tres vite (~3Hz).

**Solution** : Passez a X11. Correction prevue en 1.9.0.

## Touches F13-F24 non reconnues

Le support des touches de fonction etendues (F13-F24), touches du pave numerique et touches OEM a ete ajoute en version **1.8.0**.

## Boutons de souris

Les raccourcis par bouton de souris sont supportes depuis la v1.8.0.

## Raccourcis recommandes par OS

| OS | Recommande | A eviter |
|---|---|---|
| **Windows** | `Ctrl+Espace`, `Ctrl+Alt+M`, `F2` | AltGr (= Ctrl+Alt) |
| **macOS** | `Ctrl+Option+M`, `F2`, `F3`, bouton souris | Espace, chiffres, lettres |
| **Linux (X11)** | `Ctrl+Espace`, `F2`, `Ctrl+Alt+M` | - |
| **Linux (Wayland)** | N/A (pas encore supporte) | - |
