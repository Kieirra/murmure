# Beta Testing

Merci de participer au programme beta de Murmure ! Vos retours sont precieux pour fiabiliser l'application avant sa sortie officielle.

## Comment obtenir la beta

Les builds beta sont publiees avant chaque release. Rendez-vous sur la [page des releases GitHub](https://github.com/Kieirra/murmure/releases) et telechargez la derniere version marquee en pre-release.

## Nouveautes de la 1.9.0

- Smart Speech Mic : utilisez votre smartphone comme micro pour piloter Murmure
- Refonte du design des menus
- Overlay : mode streaming et nouvelles options de personnalisation
- macOS : la touche Echap ne bloque plus celle des autres applications
- Linux : support de Wayland (experience plus aboutie sous KDE que sous GNOME)
- Les raccourcis LLM Connect n'apparaissent plus dans les raccourcis quand la fonctionnalite est desactivee
- Les logs sont desormais affiches dans votre fuseau horaire
- L'icone du tray change pendant l'enregistrement
- Nom personnalise pour vos regles de formatage
- Voice Mode et Smart Mic sont inclus dans l'Import/Export
- Avertissement lorsque des chiffres sont utilises dans le Dictionnaire

## Plan de test

Testez ce que vous pouvez, sans pression. Chaque case cochee nous aide.

### Transcription

- [ ] Enregistrer et transcrire en push-to-talk
- [ ] Enregistrer et transcrire en toggle-to-talk
- [ ] Tester sur une phrase courte (1 a 2 mots)
- [ ] Tester sur une dictee plus longue (2 a 3 phrases)
- [ ] Tester une transcription avec post-traitement LLM

### Overlay

- [ ] Verifier que l'overlay s'affiche pendant l'enregistrement
- [ ] Activer le mode streaming et verifier qu'il fonctionne correctement
- [ ] Personnaliser l'overlay et verifier que les reglages sont bien appliques

### Mode vocal

- [ ] Activer le mode vocal depuis le menu Extensions
- [ ] Declencher un enregistrement en prononcant le mot-cle
- [ ] Tester l'envoi automatique avec "Merci alix" apres une transcription vocale
- [ ] Tester avec le delai de silence regle sur Indefinite
- [ ] Verifier l'activation/desactivation du mode vocal via Ctrl+Shift+0

### Smart Speech Mic

- [ ] Activer Smart Mic et scanner le QR code avec votre telephone
- [ ] Verifier que l'audio du telephone arrive bien dans Murmure
- [ ] Realiser une transcription avec le micro du telephone
- [ ] Tester les actions clic gauche, supprimer et entree depuis le telephone

### Import/Export des parametres

- [ ] Exporter l'ensemble des parametres
- [ ] Exporter seulement une selection de parametres
- [ ] Modifier un parametre, puis reimporter le fichier exporte
- [ ] Verifier que les parametres sont bien restaures
- [ ] Tester l'import en ligne de commande : `murmure import <fichier>`

### Autres

- [ ] (macOS) Verifier qu'Echap ne bloque plus la fermeture des autres apps hors transcription
- [ ] Verifier qu'Echap annule bien l'enregistrement en cours
- [ ] Renommer une regle de formatage personnalisee

## Signaler un bug

Pas besoin d'ouvrir une issue GitHub, repondez directement dans la conversation d'annonce de la beta en precisant :

- **OS** : Windows, macOS (Intel ou Silicon) ou Linux (avec la distribution)
- **Version** : la version beta utilisee
- **Description** : ce qui s'est passe
- **Etapes de reproduction** : comment declencher le bug
- **Logs** : activez le mode debug dans Parametres > Systeme, reproduisez le bug, puis joignez le fichier log

Merci pour votre contribution !
