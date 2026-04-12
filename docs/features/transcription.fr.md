# Transcription

La fonctionnalite principale de Murmure est la reconnaissance vocale locale propulsee par le modele Parakeet TDT 0.6B v3 de NVIDIA.

## Fonctionnement

1. Vous appuyez sur le raccourci d'enregistrement
2. L'audio est capture depuis votre microphone en WAV 16kHz mono
3. Le modele Parakeet transcrit l'audio localement
4. Le post-traitement est applique (dictionnaire, regles de formatage, LLM si active)
5. Le texte est insere dans l'application active

Tout est traite sur votre CPU - pas de GPU requis, pas d'internet necessaire.

## Detection de la langue

Parakeet detecte automatiquement la langue a partir de l'audio. Il n'est actuellement pas possible de forcer une langue specifique.

**Langues supportees** : bulgare, croate, tcheque, danois, neerlandais, anglais, estonien, finnois, francais, allemand, grec, hongrois, italien, letton, lituanien, maltais, polonais, portugais, roumain, slovaque, slovene, espagnol, suedois, russe, ukrainien.

!!! note "La precision varie selon les langues"
Le francais, l'anglais, l'allemand et le suedois fonctionnent tres bien. Le grec et certaines langues plus rares ont une precision inferieure.

## Limites d'enregistrement

- **Duree maximale** : 5 minutes par enregistrement
- **Duree minimale** : Les enregistrements tres courts (< 1 seconde) peuvent echouer

Apres 5 minutes, l'enregistrement s'arrete automatiquement et la transcription commence.

## Conseils pour une meilleure transcription

### La qualite du microphone est determinante

Le facteur le plus important est votre microphone. Un microphone de mauvaise qualite avec du bruit de fond entrainera :

- Une detection de langue incorrecte (ex: le francais transcrit en anglais)
- Des mots manquants ou deformes

**Testez votre microphone** : Enregistrez-vous avec une autre application (Audacity, l'enregistreur de votre OS) et reecoutez. Si c'est bruite ou etouffe, Murmure aura du mal aussi.

### Bonnes pratiques

- Parlez a un rythme naturel
- Gardez une distance constante avec le microphone
- Minimisez le bruit de fond
- Utilisez un microphone dedie plutot que celui integre a votre ordinateur
- Pour les longues dictees, marquez une pause entre les phrases

### Pourquoi Murmure transcrit-il dans la mauvaise langue ?

C'est le probleme le plus signale. Parakeet detecte la langue a partir des caracteristiques audio. Quand la qualite est mauvaise, il tend a choisir l'anglais par defaut.

**Solutions :**

1. Verifiez la qualite de votre microphone
2. Assurez-vous que le bon microphone est selectionne dans Parametres > Systeme
3. Reduisez le bruit de fond
4. Augmentez le volume du microphone dans les parametres son de votre OS

Voir [Depannage transcription](../troubleshooting/transcription.md) pour un guide detaille.

## Overlay d'enregistrement

Pendant l'enregistrement, Murmure affiche un petit overlay. Configurez-le dans Parametres > Systeme :

- **Toujours** : L'overlay est toujours visible
- **Enregistrement** : L'overlay n'apparait que pendant l'enregistrement
- **Jamais** : Pas d'overlay

## Historique

Murmure conserve vos 5 dernieres transcriptions dans la barre laterale. Cliquez sur une entree pour la copier. L'historique affiche aussi :

- Mots par minute
- Total de mots transcrits
- Economies de donnees estimees vs les services cloud
