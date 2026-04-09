# Problemes de transcription

## Murmure transcrit dans la mauvaise langue

Le probleme le plus signale. Vous parlez francais mais Murmure transcrit en anglais.

### Pourquoi

Parakeet detecte la langue automatiquement. Quand la qualite audio est mauvaise - bruit de fond, volume faible, mauvais micro - le modele tend a choisir l'anglais par defaut.

### Diagnostic etape par etape

**Etape 1 : Testez votre microphone**

Enregistrez-vous avec une autre application (Audacity, l'enregistreur de votre OS). Si l'audio est bruite, etouffe ou faible, c'est le probleme.

**Etape 2 : Verifiez le bon microphone**

Allez dans **Parametres** > **Systeme** > **Microphone**. Si "Automatique", essayez de selectionner manuellement votre microphone.

**Etape 3 : Augmentez le volume**

Ouvrez les parametres son de votre OS et augmentez le volume d'entree.

**Etape 4 : Reduisez le bruit de fond**

Fermez les fenetres, coupez les autres sources audio, eloignez-vous des ventilateurs.

### Microphones connus pour causer des problemes

- Microphones integres d'ordinateurs portables (surtout en environnement bruyant)
- Microphones sans fil DJI Mini
- Microphones USB bon marche sans reduction de bruit

## La transcription est coupee

### En mode push-to-talk

Assurez-vous de maintenir le raccourci pendant toute la duree souhaitee.

### En mode activation vocale

Le mode vocal s'arrete quand le volume descend sous le seuil de silence. Si le volume de votre micro est trop bas, il coupe trop tot.

**Solution** : Augmentez le volume du micro ou ajustez le delai de silence (defaut : 1,5 seconde).

### Duree maximale

Les enregistrements sont limites a **5 minutes**.

## Erreur de transcription : "ORT Error"

| Variante | Cause | Solution |
|---|---|---|
| "Non-zero status code returned while running Pad node" | Audio trop court (0 echantillons) | Verifier le comportement du raccourci |
| Erreur ORT au demarrage | Fichiers modele corrompus | Reinstaller Murmure |
| Erreur ORT sous Linux | Problemes de permissions Wayland | Passer a X11 |

## Qualite insuffisante pour certaines langues

La precision de Parakeet varie. Le francais, l'anglais, l'allemand et le suedois ont la meilleure precision. Le grec et certaines langues plus petites ont une precision sensiblement inferieure.
