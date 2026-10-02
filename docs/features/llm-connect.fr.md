# Mode Prompt

![Mode Prompt](../assets/llm-connect.png)

Le mode prompt (anciennement LLM Connect) permet de post-traiter votre transcription avec un modele de langage local ou distant avant l'insertion. Utile pour la traduction, la correction grammaticale, le formatage medical, la generation de code, etc.

## Pre-requis

Vous avez besoin de :

- **Ollama** (local) - Gratuit, tourne sur votre machine
- **Toute API compatible OpenAI** (distant) - LM Studio, vLLM, text-generation-webui, etc.

## Configuration avec Ollama (local)

### 1. Installer Ollama

Telechargez depuis [ollama.com](https://ollama.com) et installez, puis assurez-vous qu'Ollama est en cours d'execution.

### 2. Ouvrir l'onboarding du Mode Prompt dans Murmure

1. Ouvrez Murmure > **Extensions** > **Mode Prompt** et cliquez sur **Activer le mode prompt**
2. Suivez l'assistant d'onboarding en 3 etapes. Choisissez d'abord **Local** (Ollama sur votre ordinateur)
3. Installez Ollama : Murmure verifie la connexion a Ollama
4. Choisissez le modele : Murmure affiche une liste de modeles recommandes avec les exigences materiel. Cliquez sur un modele pour le telecharger, Murmure orchestre le telechargement directement depuis son interface, avec une barre de progression
5. Une fois le modele choisi, cliquez sur **Finish Setup** pour terminer la configuration

**Recommandations par materiel :**

| VRAM recommandee | Modele recommande    | Notes                                       |
| ---------------- | -------------------- | ------------------------------------------- |
| 4 Go             | `qwen3.5:4b`         | Leger, corrections basiques                 |
| 7 Go             | `ministral-3:latest` | Bon raisonnement (Ministral 3 8B)           |
| 8 Go             | `qwen3.5:latest`     | Meilleur suivi des instructions (Qwen 3.5 9B) |

!!! warning "Sans GPU = lent"
    Sans GPU, l'inference LLM est tres lente. Pour une experience fluide, il faut soit un GPU avec suffisamment de VRAM, soit un CPU rapide avec assez de RAM.

### Verifier qu'Ollama fonctionne

```bash
ollama list    # Modeles installes
ollama ps      # Modele charge + utilisation GPU
```

Si `ollama ps` affiche **0% GPU**, l'inference sera sur CPU uniquement.

## Configuration avec serveur distant

Murmure supporte toute API compatible OpenAI : Ollama distant, LM Studio, vLLM, text-generation-webui, etc.

1. Ouvrez Murmure > **Extensions** > **Mode Prompt** et cliquez sur **Activer le mode prompt**
2. Suivez l'assistant d'onboarding en 3 etapes. Choisissez d'abord **Remote** (serveur distant)
3. Configurez le serveur distant en entrant l'URL du serveur :
    - Ollama distant : `http://your-server:11434`
    - LM Studio : `http://your-server:1234/v1`
    - Tout endpoint compatible OpenAI
4. Choisissez le modele : selectionnez-le dans la liste (Murmure recupere les modeles disponibles sur le serveur), ou saisissez le nom exact du modele dans le champ si votre serveur ne fournit pas de liste, par exemple `claude-haiku-4-5`
5. Cliquez sur **Finish Setup** pour terminer la configuration

!!! note "Ollama distant"
    Si vous hebergez Ollama sur une autre machine, assurez-vous que `OLLAMA_HOST=0.0.0.0` est defini sur le serveur pour accepter les connexions distantes.

Vous pouvez mixer fournisseurs locaux et distants entre vos prompts - par exemple, Prompt 1 avec Ollama local et Prompt 2 avec un serveur distant.

![Configuration avancee du Mode Prompt](../assets/llm-connect-advanced.png)

## Templates de prompts

Le mode prompt supporte plusieurs prompts sauvegardes, jusqu'a 4. Chaque prompt peut avoir son propre fournisseur, modele, prompt systeme et prompt utilisateur (avec `{{text}}` comme placeholder).

### Presets integres

- **Traduction** - Traduire la transcription
- **Medical** - Formatage pour dictee medicale (terminologie DCI)
- **Developpement** - Formatage pour dictee liee au code
- **Dictee vocale** - Nettoyer le texte parle pour l'ecrit

## Deux facons d'utiliser un prompt

L'onglet de chaque prompt affiche une petite barre au-dessus de l'editeur de prompt, avec une entree par geste et une icone d'aide qui detaille ses etapes.

| Geste | Entree | Instruction |
| --- | --- | --- |
| **Dicter** | votre voix | le texte enregistre du prompt |
| **Transformer** | la selection | le texte enregistre du prompt, applique instantanement |

### Dicter

Chacun des 4 prompts dispose de son propre raccourci pour Dicter (`Ctrl+Shift+1` a `Ctrl+Shift+4` par defaut). Appuyer sur l'un de ces raccourcis lance immediatement l'enregistrement, et le prompt est applique a votre dictee en une seule action.

### Transformer

Chaque prompt dispose aussi de son propre raccourci, independant, pour Transformer (`Ctrl+Alt+Shift+1` a `Ctrl+Alt+Shift+4` par defaut). Selectionnez du texte dans n'importe quelle application, appuyez sur le raccourci, et le prompt enregistre s'applique directement a votre selection, sans rien dicter. Un son et une animation de vagues jouent pendant que le modele traite votre selection.

Si rien n'est selectionne, Murmure affiche un toast demandant de selectionner du texte, sans appeler le modele. Si l'appel au modele echoue, votre selection reste intacte.

Si un prompt est vide, Dicter et Transformer affichent un toast : "Le prompt N n'est pas configure. Ouvrez Mode Prompt pour le configurer."

## Commande

Commande ne fait pas partie des prompts. Elle a sa propre page dans **Extensions** > **Mode Commande** et son propre modele, donc changer de prompt ne change jamais le modele utilise par vos commandes. Voir [Commandes](commands.fr.md).

Le mode prompt et le mode commande s'activent separement, chacun depuis sa page. La configuration du modele est partagee, une fois faite depuis une page il suffit d'activer l'autre. Desactiver une extension conserve vos prompts, modeles et serveurs.

## Raccourcis

Les raccourcis Dicter et Transformer sont independants et configurables par prompt dans **Parametres > Raccourcis**, nommes **Dicter avec {nom du prompt}** et **Transformer avec {nom du prompt}**. Chacun peut etre rebinde sur n'importe quelle combinaison, y compris un bouton de souris ou une touche `F13` a `F20`.

Sous Linux Wayland, ou le compositeur possede les raccourcis clavier, utilisez la CLI a la place : `murmure --llm-mode <N>` pour Dicter et `murmure --llm-transform <N>` pour Transformer. Voir [CLI](cli.fr.md).

## Problemes connus

- Certains modeles ajoutent des guillemets ou des balises `<think>`. La solution la plus efficace est de creer une [Regle de formatage](formatting-rules.md) personnalisee avec regex pour les supprimer automatiquement (ex: `<think>[\s\S]*?</think>` remplace par rien). Vous pouvez aussi ajouter "Donne uniquement le resultat, sans guillemets, sans reflexion" a votre prompt, ou utiliser les modeles recommandes (Qwen, Ministral).
- **macOS** : Les raccourcis Dicter par defaut (`Ctrl+Shift+1..4`) peuvent inserer des caracteres parasites. Si c'est le cas, rebindez-les sur des combinaisons sans chiffres dans Settings > Shortcuts.

Voir [Depannage du Mode Prompt](../troubleshooting/llm-connect.md).
