# Problemes LLM Connect

## Erreur Ollama 500

Une erreur 500 d'Ollama signifie generalement que le modele est trop gros pour votre memoire disponible.

### Diagnostic

```bash
ollama list    # Modeles installes
ollama ps      # Modele charge + utilisation GPU
ollama run qwen3.5:4b  # Tester directement
```

Si `ollama ps` affiche **0% GPU**, l'inference est entierement sur CPU et sera lente.

### Solution : Utiliser un modele plus petit

| RAM/VRAM disponible | Modele recommande |
|---|---|
| 4 Go | `qwen3.5:2b` |
| 8 Go | `qwen3.5:4b` |
| 16+ Go (ou 8+ Go VRAM) | `qwen3.5:8b` |

```bash
ollama pull qwen3.5:4b
```

Puis selectionnez le nouveau modele dans les parametres LLM Connect.

## Ollama non detecte

**Solution** : Renseignez manuellement l'URL dans LLM Connect :

- Ollama local : `http://localhost:11434`
- Ollama distant : `http://<ip-serveur>:11434`

## Le LLM ajoute des guillemets ou des balises

Certains modeles enveloppent leur sortie dans des guillemets (`"..."`) ou ajoutent des balises `<think>...</think>`.

**Solutions :**

1. Utilisez les modeles recommandes : Qwen 3.5, Ministral
2. Ajoutez a votre prompt systeme : "Donne uniquement le resultat. Pas de guillemets, pas de reflexion, pas d'explication."

## Le LLM est tres lent

- **Pas de GPU** : L'inference sur CPU est lente. Essayez un modele plus petit.
- **Modele trop gros** : S'il ne tient pas en VRAM, il tombe sur le CPU. Verifiez avec `ollama ps`.
- **Premiere requete** : Plus lente (chargement du modele). Les suivantes sont plus rapides.

## Problemes de connexion au serveur distant

1. Verifiez que le serveur est accessible : `curl http://<serveur>:<port>/api/tags`
2. Verifiez les pare-feu des deux machines
3. L'URL dans Murmure doit inclure le protocole (`http://` ou `https://`)
4. Pour Ollama, assurez-vous que `OLLAMA_HOST=0.0.0.0` est defini sur le serveur

!!! note "Support proxy"
    Le proxy HTTP pour LLM Connect n'est pas encore supporte. Commentez sur [#286](https://github.com/Kieirra/murmure/issues/286) si vous en avez besoin.
