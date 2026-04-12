# Dictionnaire

![Dictionnaire](../assets/dictionary.png)

Le dictionnaire aide Murmure a reconnaitre des mots qu'il pourrait autrement manquer ou mal orthographier - noms propres, termes techniques, noms de marques, etc.

## Fonctionnement

Murmure utilise un algorithme de correspondance phonetique (Beider-Morse) pour comparer la transcription de Parakeet avec vos entrees de dictionnaire. Si un mot prononce ressemble a une entree, il est remplace.

**Exemple** : Vous ajoutez "Kieirra" au dictionnaire. Quand Parakeet transcrit "Kierra" ou "Kyera", le dictionnaire corrige en "Kieirra".

## Ajouter des mots

1. Allez dans **Parametres** > **Dictionnaire** (ou la section Personnaliser)
2. Tapez le mot et cliquez sur Ajouter
3. Le mot est immediatement actif

## Bonnes pratiques

!!! warning "Moins c'est mieux"
Le dictionnaire fonctionne mieux avec une liste courte et ciblee. Ajouter trop d'entrees (surtout des centaines de mots similaires) **degrade la qualite de transcription**.

**A faire :**

- Ajouter les noms propres que Parakeet rate systematiquement
- Ajouter les termes techniques specifiques a votre domaine
- Ajouter les acronymes qui doivent etre en majuscules

**A eviter :**

- Ajouter des listes entieres de medicaments ou des glossaires complets
- Ajouter des mots courants que Parakeet gere deja bien
- Ajouter des mots avec des chiffres ou caracteres speciaux (non supporte - utilisez les [Regles de formatage](formatting-rules.md))

### Pourquoi un gros dictionnaire degrade la qualite ?

L'algorithme phonetique fait des correspondances trop agressives quand il y a beaucoup d'entrees similaires. Par exemple, 200 termes medicaux peuvent transformer "a" en "eau" ou ajouter des prefixes aleatoires.

## Limitations du dictionnaire

- **Caracteres alphabetiques uniquement** - Pas de chiffres, tirets ou caracteres speciaux
- **Mots uniques uniquement** - Les expressions multi-mots ne sont pas supportees
- **Correspondance phonetique uniquement** - L'algorithme matche par son, pas par contexte

Pour les remplacements complexes, utilisez les [Regles de formatage](formatting-rules.md) avec regex.

## Import / Export

- **Exporter** : Telecharge votre dictionnaire
- **Importer** : Charge des mots depuis un fichier exporte
- **Tout supprimer** : Supprime toutes les entrees

Des presets medicaux sont disponibles.
