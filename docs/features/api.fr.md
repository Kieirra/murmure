# API locale

L'API locale permet à d'autres applications d'envoyer des fichiers audio à Murmure pour transcription sans utiliser l'interface graphique. Elle peut aussi appliquer un prompt du Mode Prompt à la transcription et renvoyer le texte final.

## Démarrage rapide

1. Ouvrez Murmure
2. Allez dans **Extensions** > **API locale**
3. Cliquez sur **Activer l'API locale**
4. L'API démarre sur `http://127.0.0.1:4800`
5. (Optionnel) Changez le port dans le bloc **Serveur**

L'API tourne tant que Murmure est ouvert. La page affiche l'état du serveur. Si le port est déjà utilisé par une autre application, choisissez un autre port et le serveur redémarre tout seul.

La même page documente chaque endpoint avec sa commande `curl`, ses champs et un exemple de réponse. Un bouton **Essayer** envoie une vraie requête à l'API depuis la page, ce qui permet de tester un endpoint avec un fichier WAV sans écrire de code. Le résultat d'un test n'est pas enregistré et disparaît quand vous quittez la page.

## Transcription

**POST** `http://127.0.0.1:4800/api/transcribe`

Envoyez un formulaire multipart avec un fichier WAV :

```bash
curl -X POST http://127.0.0.1:4800/api/transcribe \
  -F "audio=@enregistrement.wav" \
  | jq '.text'
```

### Réponse de la transcription

**Succès (200) :**

```json
{
    "text": "Bonjour à tous, voici la transcription complète..."
}
```

**Erreur (4xx/5xx) :**

```json
{
    "error": "Message d'erreur décrivant le problème"
}
```

## Mode Prompt

Deux endpoints transcrivent l'audio, envoient le texte à un modèle de langage et renvoient le texte final. Un troisième liste vos prompts enregistrés.

| Endpoint                       | Ce qu'il fait                                                                            |
| ------------------------------ | ---------------------------------------------------------------------------------------- |
| `POST /api/prompt-mode/custom` | Applique un prompt que vous envoyez dans la requête, avec le fournisseur et le modèle de votre choix |
| `POST /api/prompt-mode`        | Applique l'un de vos prompts enregistrés du Mode Prompt, choisi par son nom              |
| `GET /api/prompt-mode/prompts` | Renvoie les noms de vos prompts enregistrés                                              |

Avant de les utiliser, retenez les points suivants :

- Le [Mode Prompt](llm-connect.md) doit être activé dans **Extensions** > **Mode Prompt**. S'il est désactivé, les deux endpoints POST renvoient une erreur `409`.
- L'API utilise la connexion déjà configurée dans Murmure. L'URL d'Ollama, l'URL du serveur distant et la clé d'API ne sont jamais envoyées dans une requête.
- Un appel API n'a aucun effet dans Murmure. Il n'y a ni overlay, ni son, ni notification, ni carte de résultat. Rien n'est ajouté à l'historique ni aux statistiques, rien n'est collé ni copié, et le prompt actif et vos réglages ne changent pas.
- Les requêtes sont traitées une par une, comme `/api/transcribe`. L'appel au modèle de langage fait partie de la requête, la réponse arrive donc après les deux étapes.
- Le serveur Ollama ou le serveur distant peut être lent au premier appel. Les timeouts sont de 120 secondes pour Ollama (local) et de 60 secondes pour un serveur distant.

### Prompt personnalisé

**POST** `http://127.0.0.1:4800/api/prompt-mode/custom`

Tous les champs sont obligatoires, envoyés en `multipart/form-data` :

| Champ         | Description                                                                 |
| ------------- | --------------------------------------------------------------------------- |
| `audio`       | Fichier WAV                                                                 |
| `instruction` | Votre prompt, 4000 caractères maximum                                       |
| `provider`    | `local` (Ollama) ou `remote` (votre serveur compatible OpenAI)              |
| `model`       | Nom du modèle chez ce fournisseur                                           |

```bash
curl -X POST http://127.0.0.1:4800/api/prompt-mode/custom \
  -F "audio=@enregistrement.wav" \
  -F "instruction=Résume en trois points" \
  -F "provider=local" \
  -F "model=qwen3:8b"
```

L'instruction est envoyée comme prompt système et la transcription comme prompt utilisateur. Les variables de prompt comme `{{TRANSCRIPT}}` ne sont pas remplacées. Murmure ne vérifie pas que le modèle existe avant l'appel. Un modèle inconnu renvoie une erreur `502`.

!!! warning "Fournisseur remote"
    Avec le prompt personnalisé, n'importe quel programme de votre ordinateur peut utiliser le modèle de langage configuré dans Murmure tant que l'API est activée. Cela inclut un serveur distant et la clé d'API enregistrée pour lui. L'API est désactivée par défaut et n'écoute que sur `127.0.0.1`. Comme d'habitude, Murmure refuse d'envoyer la clé d'API à un serveur public en HTTP simple.

### Prompt enregistré

**POST** `http://127.0.0.1:4800/api/prompt-mode`

Tous les champs sont obligatoires, envoyés en `multipart/form-data` :

| Champ    | Description                                                                   |
| -------- | ----------------------------------------------------------------------------- |
| `audio`  | Fichier WAV                                                                   |
| `prompt` | Nom exact d'un prompt enregistré. La casse compte, les espaces autour du nom sont ignorés |

```bash
curl -X POST http://127.0.0.1:4800/api/prompt-mode \
  -F "audio=@enregistrement.wav" \
  -F "prompt=Email"
```

Le texte du prompt, le fournisseur et le modèle enregistrés dans Murmure sont utilisés, comme avec le raccourci du Mode Prompt. Le prompt actif n'est ni utilisé ni modifié. Si deux prompts portent le même nom, le premier est utilisé.

### Lister les prompts

**GET** `http://127.0.0.1:4800/api/prompt-mode/prompts`

```bash
curl http://127.0.0.1:4800/api/prompt-mode/prompts
```

```json
{
    "prompts": ["General", "Email"]
}
```

Seuls les noms sont renvoyés. Cet endpoint fonctionne aussi quand le Mode Prompt est désactivé.

### Réponse du Mode Prompt

**Succès (200) :**

```json
{
    "text": "- Premier point\n- Deuxième point\n- Troisième point",
    "transcription": "Bonjour à tous, voici la transcription complète..."
}
```

- `text` est la sortie du modèle de langage, avec vos règles de formatage appliquées
- `transcription` est le texte que `/api/transcribe` renvoie pour le même audio

Si l'audio est silencieux, les deux champs sont vides et le modèle de langage n'est pas appelé.

### Erreurs du Mode Prompt

Les erreurs ont cette forme :

```json
{
    "error": "Message décrivant le problème",
    "code": "prompt_mode_disabled"
}
```

| Statut | `code`                    | Quand                                                                                          |
| ------ | ------------------------- | ---------------------------------------------------------------------------------------------- |
| 400    | `invalid_request`         | La requête n'est pas un formulaire multipart valide, un champ est manquant ou vide, ou `provider` n'est ni `local` ni `remote` |
| 400    | `instruction_too_long`    | L'instruction dépasse 4000 caractères                                                          |
| 404    | `prompt_not_found`        | Aucun prompt enregistré ne porte ce nom                                                        |
| 409    | `prompt_mode_disabled`    | Le Mode Prompt est désactivé                                                                   |
| 422    | `prompt_not_configured`   | Le prompt enregistré n'a pas de texte ou pas de modèle                                         |
| 422    | `provider_not_configured` | `provider` vaut `remote` mais aucun serveur distant n'est configuré dans le Mode Prompt        |
| 500    | `transcription_failed`    | La transcription a échoué, pour les mêmes raisons que `/api/transcribe`                        |
| 502    | `llm_failed`              | L'appel au modèle de langage a échoué, par exemple Ollama est arrêté ou le modèle n'existe pas |

Les contrôles `400`, `404`, `409` et `422` sont faits avant la transcription, une requête refusée ne coûte donc aucun temps de transcription.

Deux erreurs portent un champ supplémentaire :

- `404 prompt_not_found` ajoute `available`, la liste des noms de prompts enregistrés
- `502 llm_failed` ajoute `transcription`, pour ne pas perdre la transcription quand seul le modèle de langage a échoué

```json
{
    "error": "Prompt \"Mail\" not found.",
    "code": "prompt_not_found",
    "available": ["General", "Email"]
}
```

Les messages d'erreur (`error`) sont en anglais.

## Exemples de code

=== "Python"

    ```python
    import requests

    with open('audio.wav', 'rb') as f:
        response = requests.post(
            'http://127.0.0.1:4800/api/transcribe',
            files={'audio': f}
        )
        print(response.json()['text'])
    ```

    Avec le Mode Prompt :

    ```python
    import requests

    with open('audio.wav', 'rb') as f:
        response = requests.post(
            'http://127.0.0.1:4800/api/prompt-mode/custom',
            files={'audio': f},
            data={
                'instruction': 'Résume en trois points',
                'provider': 'local',
                'model': 'qwen3:8b',
            },
        )

    result = response.json()
    if response.ok:
        print(result['text'])
    else:
        print(result['code'], result['error'])
    ```

=== "JavaScript"

    ```javascript
    const fs = require('fs');
    const FormData = require('form-data');
    const axios = require('axios');

    const form = new FormData();
    form.append('audio', fs.createReadStream('enregistrement.wav'));

    const response = await axios.post(
      'http://127.0.0.1:4800/api/transcribe',
      form,
      { headers: form.getHeaders() }
    );
    console.log(response.data.text);
    ```

=== "Bash"

    ```bash
    curl -X POST http://127.0.0.1:4800/api/transcribe \
      -F "audio=@enregistrement.wav" \
      | jq '.text'
    ```

## Limitations

| Contrainte            | Valeur                                                                                                              |
| --------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Format audio          | WAV uniquement                                                                                                      |
| Taille max            | 100 Mo                                                                                                              |
| Durée audio           | Aucune limite, seule la taille de 100 Mo s'applique                                                                 |
| Annulation            | Fermez la connexion. L'arrêt intervient à la fin du segment audio en cours                                          |
| Fréquence optimale    | 16kHz mono (les autres sont rééchantillonnés)                                                                       |
| Streaming temps réel  | Non supporté                                                                                                        |
| Requêtes concurrentes | Séquentielles uniquement (file d'attente). Une requête annulée libère sa place dès qu'elle s'est réellement arrêtée |
| Accès réseau          | 127.0.0.1 uniquement                                                                                                |
| En-tête Host          | Les requêtes dont l'en-tête `Host` n'est ni `localhost` ni une IP de bouclage (comme `127.0.0.1`) sont refusées avec une erreur `403` |
| CORS                  | Désactivé. Les requêtes qui envoient un `Origin` autre que localhost sont refusées. curl (sans Origin) est autorisé. Les pages web ne peuvent pas lire les réponses. Seule la fenêtre de Murmure reçoit des en-têtes CORS, pour le bouton **Essayer**. |

## Notes

- Le dictionnaire personnalisé est automatiquement appliqué
- La langue est détectée automatiquement
- La première requête est plus lente (chargement du modèle)
- Le port est configurable entre 1024 et 65535
- Si vous utilisiez déjà l'API avant son passage dans Extensions, elle reste activée avec le même port
- Un audio long est découpé en segments avant transcription, exactement comme le font le raccourci clavier et la CLI. Il n'y a aucune limite de durée.
- La réponse est synchrone. Un fichier long garde la connexion ouverte plusieurs minutes, désactivez donc le timeout de votre client HTTP ou réglez-le bien au-delà de la durée attendue : un timeout qui expire annule la transcription.
- Le nettoyage des tics de langage et les règles de formatage sont appliqués au résultat, l'API renvoie donc le même texte que le raccourci clavier pour un même audio.
