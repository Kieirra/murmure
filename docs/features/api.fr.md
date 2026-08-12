# API locale

!!! info "Experimental"
    L'API HTTP locale est experimentale. Sa conception peut changer dans les futures versions.

L'API locale permet a d'autres applications d'envoyer des fichiers audio a Murmure pour transcription sans utiliser l'interface graphique.

## Demarrage rapide

1. Ouvrez Murmure
2. Allez dans **Parametres** > **Systeme**
3. Trouvez **API locale (Experimental)** et activez-la
4. L'API demarre sur `http://localhost:4800`
5. (Optionnel) Changez le port si necessaire

L'API tourne tant que Murmure est ouvert.

## Endpoint

**POST** `http://localhost:4800/api/transcribe`

Envoyez un formulaire multipart avec un fichier WAV :

```bash
curl -X POST http://127.0.0.1:4800/api/transcribe \
  -F "audio=@enregistrement.wav" \
  | jq '.text'
```

### Reponse

**Succes (200) :**

```json
{
    "text": "Bonjour a tous, voici la transcription complete..."
}
```

**Erreur (4xx/5xx) :**

```json
{
    "error": "Message d'erreur decrivant le probleme"
}
```

## Exemples de code

=== "Python"

    ```python
    import requests

    with open('audio.wav', 'rb') as f:
        response = requests.post(
            'http://localhost:4800/api/transcribe',
            files={'audio': f}
        )
        print(response.json()['text'])
    ```

=== "JavaScript"

    ```javascript
    const fs = require('fs');
    const FormData = require('form-data');
    const axios = require('axios');

    const form = new FormData();
    form.append('audio', fs.createReadStream('enregistrement.wav'));

    const response = await axios.post(
      'http://localhost:4800/api/transcribe',
      form,
      { headers: form.getHeaders() }
    );
    console.log(response.data.text);
    ```

=== "Bash"

    ```bash
    curl -X POST http://localhost:4800/api/transcribe \
      -F "audio=@enregistrement.wav" \
      | jq '.text'
    ```

## Limitations

| Contrainte            | Valeur                                                                                                              |
| --------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Format audio          | WAV uniquement                                                                                                      |
| Taille max            | 100 Mo                                                                                                              |
| Duree audio           | Aucune limite, seule la taille de 100 Mo s'applique                                                                 |
| Annulation            | Fermez la connexion. L'arret intervient a la fin du segment audio en cours                                          |
| Frequence optimale    | 16kHz mono (les autres sont reechantillonnes)                                                                       |
| Streaming temps reel  | Non supporte                                                                                                        |
| Requetes concurrentes | Sequentielles uniquement (file d'attente). Une requete annulee libere sa place des qu'elle s'est reellement arretee |
| Acces reseau          | localhost / 127.0.0.1 uniquement                                                                                    |
| CORS                  | Desactive                                                                                                           |

## Notes

- Le dictionnaire personnalise est automatiquement applique
- La langue est detectee automatiquement
- La premiere requete est plus lente (chargement du modele)
- Le port est configurable entre 1024 et 65535
- Un audio long est decoupe en segments avant transcription, exactement comme le font le raccourci clavier et la CLI. Il n'y a aucune limite de duree.
- La reponse est synchrone. Un fichier long garde la connexion ouverte plusieurs minutes, desactivez donc le timeout de votre client HTTP ou reglez-le bien au-dela de la duree attendue : un timeout qui expire annule la transcription.
- Le nettoyage des tics de langage et les regles de formatage sont appliques au resultat, l'API renvoie donc le meme texte que le raccourci clavier pour un meme audio.
