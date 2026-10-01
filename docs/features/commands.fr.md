# Commandes

Les commandes permettent de donner une instruction vocale au LLM. Si du texte est selectionne, Murmure lui applique votre instruction et remplace la selection. Si rien n'est selectionne, Murmure repond a votre demande. Selon vos reglages, la reponse s'ecrit la ou se trouve le curseur, ou s'affiche dans l'overlay.

!!! note "Commande vs Transformer"
    Commande demande de dicter l'instruction a chaque fois. Si vous repetez toujours la meme instruction, enregistrez-la comme prompt dans un prompt du [mode prompt](llm-connect.fr.md) et utilisez **Transformer** a la place : selectionnez du texte, appuyez sur le raccourci Transformer du prompt, et le prompt enregistre s'applique sans rien dicter.

## Fonctionnement

1. **Selectionnez** du texte dans n'importe quelle application
2. Appuyez sur le **raccourci Commande** (a configurer dans Parametres > Raccourcis)
3. **Dites votre commande** (ex: "traduis en anglais", "corrige la grammaire", "raccourcis")
4. Murmure lit le texte selectionne, l'envoie avec votre commande au LLM, et remplace la selection par le resultat

## Sans selection

Si rien n'est selectionne, votre demande dictee est envoyee au LLM comme une question ou une demande de texte. La reponse est en texte brut et dans la langue de votre demande. Selon vos reglages, elle s'ecrit la ou se trouve le curseur, ou s'affiche dans l'overlay. Les reponses sont courtes par defaut, une a trois phrases. Elles sont plus longues seulement si vous demandez un texte, comme un mail ou une liste.

Par exemple, sans rien selectionner, dites "que signifie idempotent ?". Murmure vous donne une courte definition.

## Activation

Ouvrez **Extensions** > **Mode Commande** et cliquez sur **Activer le mode commande**. La premiere fois, Murmure vous demande de connecter un modele (Ollama sur votre ordinateur, ou votre propre serveur). Cette configuration est partagee avec le mode prompt, vous ne la faites qu'une fois.

Si le mode commande est desactive, le raccourci Commande ne lance pas d'enregistrement. L'overlay affiche "Mode Commande désactivé". Le desactiver conserve votre modele et vos serveurs.

## Modele

Les commandes ont leur propre modele, distinct des prompts du mode prompt. Choisissez le fournisseur (local ou distant) et le modele dans **Extensions** > **Mode Commande**. Changer de prompt actif dans le mode prompt ne change pas le modele utilise par les commandes.

Si aucun modele n'est choisi, le raccourci Commande ne lance pas d'enregistrement. L'overlay affiche "Commande sans modèle" et Murmure vous demande de choisir un modele dans Mode Commande.

## Pre-requis

Les commandes utilisent la meme configuration de modele que le [mode prompt](llm-connect.fr.md), avec Ollama ou un serveur distant. Le mode prompt lui-meme n'a pas besoin d'etre active.

## Cas d'usage

- **Traduction** : Selectionnez un paragraphe, dites "traduis en anglais"
- **Correction** : Selectionnez du texte, dites "corrige la grammaire"
- **Reformulation** : Selectionnez du texte, dites "rends ca plus formel"
- **Resume** : Selectionnez du texte, dites "resume en une phrase"
- **Code** : Selectionnez du code, dites "ajoute la gestion d'erreurs"

## Configuration

Le raccourci commande est separe du raccourci d'enregistrement. Definissez-le dans **Parametres** > **Raccourcis** > **Commande, prompt libre**.

Vous pouvez aussi declencher les commandes via le [Mode vocal](voice-mode.md) en definissant un mot-cle pour l'action commande.
