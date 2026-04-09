# Problemes d'insertion de texte

## Le texte n'apparait pas apres transcription

La transcription fonctionne (visible dans l'historique de Murmure) mais le texte n'apparait pas dans l'application cible.

### Cause

Par defaut, Murmure insere le texte en copiant dans le presse-papier et en simulant `Ctrl+V`. Certaines applications gerent le collage differemment.

### Solution : Changer le mode d'insertion

Allez dans **Parametres** > **Systeme** > **Mode d'insertion du texte** :

| Mode | Raccourci | Ideal pour |
|---|---|---|
| **Standard** | Ctrl+V | La plupart des applications, navigateurs, editeurs |
| **Terminal** | Ctrl+Shift+V | Emulateurs de terminal (GNOME Terminal, Konsole, etc.) |
| **Direct** | Simulation de frappe | LibreOffice, Git Bash, applications ou Ctrl+V ne marche pas |

### Applications necessitant le mode Direct

- **LibreOffice** (Writer, Calc, Impress)
- **Git Bash** sous Windows
- **Certains terminaux Linux**
- **Applications Electron** qui interceptent les evenements presse-papier

!!! note "Limitations du mode Direct sous Linux"
    Sur certaines configurations Linux, le mode Direct peut ne pas afficher correctement les diacritiques (e, a, u). Utilisez le mode Standard ou Terminal dans ce cas.

## Le texte apparait au mauvais endroit

Assurez-vous que l'application cible est au premier plan quand vous arretez l'enregistrement. Murmure colle dans la fenetre active au moment ou la transcription se termine.

## Le presse-papier est ecrase

Le mode Standard utilise le presse-papier. Votre contenu precedent est remplace. Si c'est un probleme, utilisez le mode **Direct** qui simule les frappes sans toucher au presse-papier.

## AltGr declenche l'enregistrement (Windows)

Sur Windows, `AltGr` est interprete comme `Ctrl+Alt`. Si votre raccourci est `Ctrl+Alt+quelquechose`, AltGr peut le declencher accidentellement.

**Solution** : Choisissez un raccourci sans `Ctrl+Alt`, ou utilisez une touche de fonction.

## Des caracteres parasites apparaissent (macOS)

Sur macOS, les raccourcis avec `Espace` ou des chiffres "fuient" ces caracteres dans l'application active.

**Solution** : Utilisez des combinaisons de modificateurs comme `Ctrl+Option+M` ou des touches de fonction (`F2`, `F3`).
