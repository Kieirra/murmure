# Depannage

Solutions aux problemes les plus courants, basees sur les retours utilisateurs.

## Problemes les plus frequents

1. **[Transcription dans la mauvaise langue](transcription.md)** - Murmure transcrit en anglais au lieu du francais
2. **[Le texte n'apparait pas](text-insertion.md)** - La transcription fonctionne mais le texte n'est pas insere
3. **[Les raccourcis ne fonctionnent pas](shortcuts.md)** - Le raccourci d'enregistrement n'a aucun effet
4. **[Erreurs LLM Connect](llm-connect.md)** - Erreurs Ollama 500, reponses lentes

## Corrections rapides

| Probleme | Solution |
|---|---|
| Mauvaise langue | Verifier la qualite du micro, reduire le bruit |
| Texte pas colle | Passer en mode Direct (Parametres > Systeme) |
| Conflit raccourci (macOS) | Changer pour Ctrl+Option+M |
| Raccourci inactif (Linux) | Passer en session X11 |
| Erreur MSVCP140.dll (Windows) | Installer [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe) |
| Erreur Ollama 500 | Utiliser un modele plus petit (qwen3.5:4b) |
| Parametres corrompus | Supprimer settings.json et redemarrer |

## Comment obtenir les logs

1. Allez dans **Parametres** > **Systeme**
2. Mettez le **Niveau de log** sur **Debug**
3. Cliquez sur l'icone dossier a cote du niveau de log
4. Reproduisez le probleme
5. Joignez le fichier log a votre [issue GitHub](https://github.com/Kieirra/murmure/issues/new)

## Emplacement des fichiers de parametres

| OS | Chemin |
|---|---|
| Windows | `%APPDATA%\com.al1x-ai.murmure\settings.json` |
| macOS | `~/Library/Application Support/com.al1x-ai.murmure/settings.json` |
| Linux | `~/.local/share/com.al1x-ai.murmure/settings.json` |
