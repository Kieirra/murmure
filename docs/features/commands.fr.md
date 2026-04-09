# Commandes

Les commandes permettent de modifier du texte selectionne avec des instructions vocales. Au lieu de transcrire vers un nouvel emplacement, Murmure lit votre texte selectionne et applique une commande vocale.

## Fonctionnement

1. **Selectionnez** du texte dans n'importe quelle application
2. Appuyez sur le **raccourci Commande** (a configurer dans Parametres > Raccourcis)
3. **Dites votre commande** (ex: "traduis en anglais", "corrige la grammaire", "raccourcis")
4. Murmure lit le texte selectionne, l'envoie avec votre commande au LLM, et remplace la selection par le resultat

## Pre-requis

Les commandes necessitent que [LLM Connect](llm-connect.md) soit configure.

## Cas d'usage

- **Traduction** : Selectionnez un paragraphe, dites "traduis en anglais"
- **Correction** : Selectionnez du texte, dites "corrige la grammaire"
- **Reformulation** : Selectionnez du texte, dites "rends ca plus formel"
- **Resume** : Selectionnez du texte, dites "resume en une phrase"
- **Code** : Selectionnez du code, dites "ajoute la gestion d'erreurs"

## Configuration

Le raccourci commande est separe du raccourci d'enregistrement. Definissez-le dans **Parametres** > **Raccourcis** > **Commande**.

Vous pouvez aussi declencher les commandes via le [Mode vocal](voice-mode.md) en definissant un mot-cle pour l'action commande.
