# Beta Testing

Merci de participer au programme beta de Murmure ! Vos retours sont precieux pour fiabiliser l'application avant sa sortie officielle.

## Comment obtenir la beta

Les builds beta sont publiees avant chaque release. Rendez-vous sur la [page des releases GitHub](https://github.com/Kieirra/murmure/releases) et telechargez la derniere version marquee en pre-release.

## Nouveautes de la 1.10.0

- Decoupage audio : les longues dictees ne sont plus limitees a 5 minutes et les morceaux sont transcrits en arriere-plan pendant que vous continuez a parler, ce qui accelere nettement les longues dictees
- Nouveau modele Parakeet personnalise, plus precis et avec moins de bascules involontaires vers l'anglais
- Reechantillonnage audio de meilleure qualite, qui ameliore la precision, surtout sur les micros bas de gamme
- Dictionnaire : algorithme ameliore pour une meilleure precision, tri alphabetique et refonte de l'interface
- Refonte de la page d'accueil
- CLI : transcrivez de l'audio directement depuis le terminal avec la commande `murmure`
- Overlay : bouton de fermeture pour annuler une transcription en cours
- Mode d'insertion de texte : nouvelle option None pour desactiver l'insertion automatique
- Icones de tray monochromes (au repos et en enregistrement) sous Linux et macOS
- Raccourcis : la touche Suppr retire le raccourci selectionne, et les doublons sont desormais empeches
- Option de debug pour conserver les cinq derniers enregistrements audio dans le dossier temporaire, avec un bouton pour l'ouvrir
- Corrections : enregistrements gresillants/robotiques sur certaines configs Linux, peripheriques Bluetooth maintenus actifs, redemarrage de l'enregistrement apres un raccourci Ctrl/Shift seul, decoupage du Smart Mic avec une limite de 20 minutes
- Corrections de securite signalees par les audits de dependances

## Plan de test

Testez ce que vous pouvez, sans pression. Chaque case cochee nous aide.

### Transcription

- [ ] Enregistrer et transcrire en push-to-talk
- [ ] Enregistrer et transcrire en toggle-to-talk
- [ ] Tester sur une phrase courte (5 a 6 mots)
- [ ] Tester une transcription avec post-traitement LLM
- [ ] (Optionnel) Tester une longue dictee de plus de 5 minutes et verifier qu'elle n'est pas coupee

### Dictionnaire

- [ ] Ajouter des mots personnalises et verifier que le nouvel algorithme du dictionnaire les reconnait mieux pendant la transcription

### Overlay

- [ ] Verifier que l'overlay s'affiche pendant l'enregistrement
- [ ] Utiliser le bouton de fermeture pour annuler une transcription en cours

### CLI

- [ ] Transcrire un fichier audio depuis le terminal avec la commande `murmure`

### Raccourcis

- [ ] Retirer un raccourci avec la touche Suppr
- [ ] Tenter d'ajouter un raccourci en doublon et verifier qu'il est empeche

### Smart Speech Mic

- [ ] Tester le Smart Speech Mic

### Parametres

- [ ] Regler le mode d'insertion de texte sur None et verifier que rien n'est insere automatiquement
- [ ] Activer l'option de debug conservant les cinq derniers enregistrements, puis ouvrir le dossier temporaire

### Autres

- [ ] Verifier les icones de tray monochromes (au repos et en enregistrement) sous Linux et macOS
- [ ] (Linux) Verifier que les enregistrements ne sont plus gresillants ni robotiques
- [ ] Verifier que les peripheriques audio Bluetooth sont liberes au repos

## Signaler un bug

Pas besoin d'ouvrir une issue GitHub, repondez directement dans la conversation d'annonce de la beta en precisant :

- **OS** : Windows, macOS (Intel ou Silicon) ou Linux (avec la distribution)
- **Version** : la version beta utilisee
- **Description** : ce qui s'est passe
- **Etapes de reproduction** : comment declencher le bug
- **Logs** : activez le mode debug dans Parametres > Systeme, reproduisez le bug, puis joignez le fichier log

Merci pour votre contribution !
