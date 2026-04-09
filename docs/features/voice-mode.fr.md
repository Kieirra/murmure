# Mode vocal

!!! info "Fonctionnalite beta"
    Le mode vocal est en beta depuis la v1.8.0. Il peut utiliser des ressources CPU significatives.

![Mode vocal](../assets/voice-mode.png)

Le mode vocal permet d'activer Murmure avec votre voix au lieu d'un raccourci clavier. Dites un mot-cle et Murmure commence a enregistrer - completement mains libres.

## Fonctionnement

1. Murmure surveille le niveau sonore ambiant via la detection d'activite vocale (VAD)
2. Quand une activite audio suffisante est detectee (quelqu'un parle), une transcription locale est declenchee
3. Si la transcription correspond a votre mot-cle, l'enregistrement demarre
4. L'enregistrement s'arrete apres un delai de silence configurable
5. La transcription est traitee et inseree (le mot-cle est retire du resultat)

## Configuration

1. Allez dans **Extensions** > **Mode vocal**
2. Activez le mode vocal
3. Definissez vos mots-cles pour chaque action

## Actions par mot-cle

| Action | Description | Exemple de mot-cle |
|---|---|---|
| **Enregistrer** | Demarrer la transcription standard | "Murmure" |
| **Enregistrer LLM Mode 1-4** | Demarrer avec un mode LLM specifique | "Murmure traduis" |
| **Enregistrer Commande** | Demarrer le mode commande | "Murmure commande" |
| **Annuler** | Annuler l'enregistrement en cours | "Annuler" |
| **Valider** | Terminer l'enregistrement et envoyer | "Termine" |

## Conseils

- **Choisissez des mots-cles distinctifs** - Evitez les mots courants
- **Les mots-cles multi-mots sont plus fiables** - "OK Murmure" est mieux que juste "Murmure"
- **Prononcez clairement** - La correspondance floue tolere de legeres variations

## Envoi automatique

Quand active, Murmure appuie automatiquement sur `Entree` apres l'insertion. Utile pour les messageries ou les invites de commande.

## Delai de silence

Controle combien de temps Murmure attend apres que vous arretez de parler. Par defaut : 1,5 seconde. Augmentez-le si vos enregistrements sont coupes trop tot.

## Avertissement CPU

Le mode vocal traite l'audio en continu en arriere-plan. Dans un environnement bruyant, l'utilisation CPU sera plus elevee.

**Recommandation** : Desactivez le mode vocal quand vous n'en avez pas besoin.
