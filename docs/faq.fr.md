# FAQ

## General

### Murmure a-t-il besoin d'internet ?

Non. Toute la transcription se fait localement. Aucune donnee n'est envoyee a un serveur.

### Murmure collecte-t-il des donnees ?

Non. Zero telemetrie, zero analyse, zero tracking. Voir la [Politique de confidentialite](https://github.com/Kieirra/murmure/blob/main/PRIVACY_POLICY.md).

### Murmure a-t-il besoin d'un GPU ?

Non. Murmure tourne sur CPU. Un GPU n'est pas requis pour la transcription. En revanche, si vous utilisez [LLM Connect](features/llm-connect.md), un GPU ameliore significativement la vitesse d'inference.

### Quelles langues Murmure supporte-t-il ?

25 langues europeennes : bulgare, croate, tcheque, danois, neerlandais, anglais, estonien, finnois, francais, allemand, grec, hongrois, italien, letton, lituanien, maltais, polonais, portugais, roumain, slovaque, slovene, espagnol, suedois, russe, ukrainien.

### Peut-on forcer une langue specifique ?

Pas encore. Parakeet detecte automatiquement la langue. Un selecteur de langue est sur la roadmap.

### Existe-t-il une application mobile ?

Non. Murmure necessite l'acces au presse-papier et la simulation clavier, que les OS mobiles n'autorisent pas. Vous pouvez cependant utiliser [Smart Speech Mic](features/smart-speech-mic.md) pour utiliser votre telephone comme micro sans fil.

### Existe-t-il une version web ?

Non, pour les memes raisons que le mobile - le sandboxing du navigateur empeche l'acces au presse-papier et au clavier.

## Transcription

### Pourquoi Murmure transcrit-il en anglais quand je parle francais ?

C'est presque toujours un probleme de qualite du microphone. Voir [Depannage transcription](troubleshooting/transcription.md).

### Quelle est la duree maximale d'enregistrement ?

5 minutes. Au-dela, l'enregistrement s'arrete automatiquement.

### Murmure peut-il transcrire des reunions ?

La transcription de reunions avec identification des locuteurs n'est pas supportee actuellement. Murmure est concu pour la dictee a un seul locuteur.

### Murmure peut-il transcrire des fichiers audio ?

Oui, via l'[API locale](features/api.md). Envoyez un fichier WAV a l'endpoint API.

## Installation

### Ou sont les logs ?

1. **Parametres** > **Systeme**
2. Mettez le **Niveau de log** sur **Debug**
3. Cliquez sur l'**icone dossier** a cote du selecteur de niveau

### Ou est le fichier de parametres ?

| OS      | Chemin                                                            |
| ------- | ----------------------------------------------------------------- |
| Windows | `%APPDATA%\com.al1x-ai.murmure\settings.json`                     |
| macOS   | `~/Library/Application Support/com.al1x-ai.murmure/settings.json` |
| Linux   | `~/.local/share/com.al1x-ai.murmure/settings.json`                |

### Comment reinitialiser tous les parametres ?

Supprimez le fichier `settings.json` et redemarrez Murmure.

### Murmure est-il disponible en Flatpak ?

Pas actuellement. Le sandboxing Flatpak entre en conflit avec les raccourcis clavier globaux.

## Fonctionnalites

### Quelle difference entre Dictionnaire et Regles de formatage ?

Le **Dictionnaire** utilise la correspondance phonetique pour corriger les mots mal reconnus (ideal pour les noms propres).

Les **Regles de formatage** font du chercher/remplacer avec support regex (ideal pour les commandes vocales, remplacements multi-mots, entrees avec chiffres).

### Peut-on utiliser Murmure avec ChatGPT/Claude/Cursor ?

Oui. Murmure fonctionne avec n'importe quelle application - il tape le texte dans la fenetre active.

### Peut-on utiliser un Stream Deck ?

Oui. Assignez une touche F13-F24 a votre bouton Stream Deck, puis definissez cette touche comme raccourci Murmure. Support ajoute en v1.8.0.

### Comment deployer Murmure sur plusieurs postes ?

Utilisez le [CLI](features/cli.md) avec l'installation silencieuse MSI :

```powershell
msiexec /package Murmure_x64.msi /quiet
murmure.exe import config-entreprise.murmure
```

## Depannage rapide

### Murmure affiche "MSVCP140.dll not found"

Installez le [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe).

### Le texte n'apparait pas dans LibreOffice

Changez le mode d'insertion en **Direct** dans Parametres > Systeme.

### Les raccourcis ne marchent pas sur macOS apres MAJ

Reinitialisation des permissions necessaire. Voir le [guide de MAJ macOS](getting-started/macos.md#mise-a-jour-depuis-160).

### Les raccourcis ne marchent pas sur Linux

Vous etes probablement sous Wayland. Murmure necessite une session X11. Voir [Installation Linux](getting-started/linux.md).
