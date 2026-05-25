# Problemes de raccourcis

## Le raccourci ne fonctionne pas

### Sur macOS

**Verifiez les permissions** : Reglages Systeme > Confidentialite et securite > **Accessibilite** et **Surveillance des entrees**.

**Apres mise a jour depuis 1.6.0** : Reinitialisation complete des permissions necessaire. Voir le [guide d'installation macOS](../getting-started/macos.md#mise-a-jour-depuis-160).

**Conflit raccourci systeme** : `Ctrl+Espace` est utilise par macOS pour le changement de source d'entree. Changez votre raccourci (ex: `Ctrl+Option+M`, `F2`).

**"Echec de sauvegarde du raccourci"** : Une autre application utilise deja ce raccourci.

### Sur Linux (Wayland)

Murmure n'enregistre aucun raccourci global par lui-même sous Wayland. Vous configurez des raccourcis personnalisés au niveau OS qui appellent directement le binaire `murmure`. Voir [Configurer les raccourcis sous Linux](../configure-shortcuts-on-linux.fr.md) pour le guide par compositeur (GNOME, KDE, Hyprland, Sway).

Si un raccourci ne se déclenche pas :

- Vérifiez que `murmure` est dans le PATH (`which murmure`).
- Assurez-vous que Murmure est déjà lancé en arrière-plan. Les commandes CLI communiquent avec l'instance en cours d'exécution.
- Vérifiez qu'aucune autre application n'a revendiqué la même combinaison dans les paramètres clavier de votre OS.

### Sur Windows

1. Verifiez qu'aucune autre application n'utilise le meme raccourci
2. Verifiez que votre antivirus (surtout Kaspersky) ne bloque pas l'ecouteur de raccourcis
3. Essayez de lancer Murmure en administrateur (test temporaire uniquement)

## Le raccourci bascule rapidement (Linux)

(X11 uniquement, le push-to-talk est desactive sous Wayland.)

Sous Linux X11, maintenir un raccourci en mode Push-to-talk peut basculer l'enregistrement on/off tres rapidement.

**Cause** : X11 envoie des evenements de repetition automatique tant que la touche est maintenue.

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
| **Linux (Wayland)** | Bindez la combinaison de votre choix au niveau OS via `murmure --transcription`. Voir Configurer les raccourcis sous Linux. | -                         |
