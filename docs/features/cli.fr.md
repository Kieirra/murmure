# CLI

Murmure propose une interface en ligne de commande pour deux usages : contrôler l'instance en cours via des commandes one-shot (utiles pour des Custom Shortcuts OS), et importer des fichiers de configuration pour le déploiement ou le partage.

## Commandes de contrôle

Ces commandes communiquent avec l'instance Murmure en cours d'exécution. Murmure doit déjà être lancé pour qu'elles prennent effet.

| Commande | Description |
| -------- | ----------- |
| `murmure --transcription` | Toggle la transcription standard ON/OFF |
| `murmure --transcription-llm` | Toggle la transcription en mode LLM |
| `murmure --transcription-command` | Toggle la transcription en mode Command |
| `murmure --paste-last` | Colle la dernière transcription |
| `murmure --cancel` | Annule l'enregistrement en cours et revient en idle |
| `murmure --voice-mode` | Toggle le Voice Mode ON/OFF |
| `murmure --llm-mode 1` | Bascule vers le mode LLM 1 |
| `murmure --llm-mode 2` | Bascule vers le mode LLM 2 |
| `murmure --llm-mode 3` | Bascule vers le mode LLM 3 |
| `murmure --llm-mode 4` | Bascule vers le mode LLM 4 |

Ces commandes sont principalement utilisées sous Linux Wayland avec le mode **CLI** de gestion des raccourcis. Voir [Configurer les raccourcis sous Linux](../configure-shortcuts-on-linux.fr.md) pour les instructions par environnement de bureau.

## Commande import

Depuis la version 1.8.0, Murmure peut importer des fichiers de configuration. Utile pour les administrateurs déployant Murmure sur plusieurs postes ou pour partager des paramètres.

### Utilisation

```bash
murmure import <FICHIER> [OPTIONS]
```

### Commandes

| Commande                                      | Description                             |
| --------------------------------------------- | --------------------------------------- |
| `murmure --help`                              | Afficher l'aide                         |
| `murmure --version`                           | Afficher la version                     |
| `murmure import <FICHIER>`                    | Importer un fichier .murmure            |
| `murmure import <FICHIER> --strategy replace` | Remplacer tous les paramètres (défaut)  |
| `murmure import <FICHIER> --strategy merge`   | Fusionner avec les paramètres existants |

### Stratégies d'import

- **replace** (defaut) - Ecrase tous les parametres existants
- **merge** - Conserve les parametres existants et ajoute les nouveaux

### Chemins par OS

=== "Linux"

    ```bash
    murmure import config.murmure
    murmure import config.murmure --strategy merge
    ```

=== "macOS"

    ```bash
    /Applications/murmure.app/Contents/MacOS/murmure import config.murmure
    ```

=== "Windows"

    ```powershell
    murmure.exe import config.murmure
    ```

![Import / Export](../assets/settings-import-export.png)

### Le format .murmure

Le fichier `.murmure` est un JSON avec la structure suivante :

```json
{
  "version": 1,
  "settings": { ... },
  "shortcuts": { ... },
  "formatting_rules": { ... },
  "llm_connect": { ... },
  "dictionary": { ... }
}
```

Chaque section est optionnelle.

### Cas d'usage

#### Déploiement IT en masse

Combinaison avec l'installation silencieuse MSI :

```powershell
msiexec /package Murmure_x64.msi /quiet
murmure.exe import config-entreprise.murmure
```

#### Partage de paramètres

Exportez vos parametres, partagez le fichier `.murmure`, et le destinataire l'importe :

```bash
murmure import parametres-collegue.murmure --strategy merge
```

### Notes

- Les operations CLI sont rapides - detection precoce sans initialisation complete
- Si Murmure est deja lance, l'import declenche un rechargement a chaud
- Le format et la version sont valides avant application
