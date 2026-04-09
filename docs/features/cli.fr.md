# CLI

Depuis la version 1.8.0, Murmure propose une interface en ligne de commande pour importer des fichiers de configuration. Utile pour les administrateurs deploying Murmure sur plusieurs postes ou pour partager des parametres.

## Utilisation

```bash
murmure import <FICHIER> [OPTIONS]
```

### Commandes

| Commande | Description |
|---|---|
| `murmure --help` | Afficher l'aide |
| `murmure --version` | Afficher la version |
| `murmure import <FICHIER>` | Importer un fichier .murmure |
| `murmure import <FICHIER> --strategy replace` | Remplacer tous les parametres (defaut) |
| `murmure import <FICHIER> --strategy merge` | Fusionner avec les parametres existants |

### Strategies d'import

- **replace** (defaut) - Ecrase tous les parametres existants
- **merge** - Conserve les parametres existants et ajoute les nouveaux

## Chemins par OS

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

## Le format .murmure

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

## Cas d'usage

### Deploiement IT en masse

Combinaison avec l'installation silencieuse MSI :

```powershell
msiexec /package Murmure_x64.msi /quiet
murmure.exe import config-entreprise.murmure
```

### Partage de parametres

Exportez vos parametres, partagez le fichier `.murmure`, et le destinataire l'importe :

```bash
murmure import parametres-collegue.murmure --strategy merge
```

## Notes

- Les operations CLI sont rapides - detection precoce sans initialisation complete
- Si Murmure est deja lance, l'import declenche un rechargement a chaud
- Le format et la version sont valides avant application
