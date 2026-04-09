# Beta Testing

Merci de participer au programme de beta testing de Murmure ! Votre contribution est essentielle pour ameliorer la qualite de l'application avant sa sortie officielle.

## Comment obtenir la beta

Les builds beta sont partagees avant chaque release. Consultez la [page des releases GitHub](https://github.com/Kieirra/murmure/releases) pour les versions pre-release.

## Plan de test

Testez ce que vous pouvez, sans pression :

### Installation et demarrage

- [ ] Telecharger et installer la version beta
- [ ] Verifier que l'application demarre correctement
- [ ] Completer l'accueil initial

### Transcription de base

- [ ] Enregistrer et transcrire en push-to-talk
- [ ] Enregistrer et transcrire en toggle-to-talk
- [ ] Verifier que le bon microphone est utilise
- [ ] Tester avec une phrase courte (1-2 mots)
- [ ] Tester avec une longue dictee (2+ minutes)

### Mode vocal

- [ ] Activer le mode vocal dans les parametres
- [ ] Dire le mot-cle pour declencher un enregistrement
- [ ] Tester l'envoi automatique (Entree) apres transcription vocale
- [ ] Demarrer un enregistrement au clavier, puis utiliser les mots vocaux pour valider/annuler
- [ ] Verifier que le mode vocal s'active/desactive correctement

### LLM Connect

- [ ] Configurer une connexion a un serveur Ollama local
- [ ] Configurer une connexion a un serveur distant (API compatible OpenAI)
- [ ] Tester une transcription avec post-traitement LLM
- [ ] Creer plusieurs modes LLM avec differents fournisseurs
- [ ] Reordonner les modes LLM par glisser-deposer

### Smart Speech Mic

- [ ] Activer Smart Mic et scanner le QR code avec votre telephone
- [ ] Verifier que l'audio arrive du telephone a Murmure
- [ ] Tester une transcription avec le microphone du telephone

### Import/Export des parametres

- [ ] Exporter tous les parametres
- [ ] Exporter uniquement certains parametres
- [ ] Modifier un parametre, puis importer le fichier precedemment exporte
- [ ] Verifier que les parametres sont restaures correctement
- [ ] Tester l'import CLI : `murmure import <fichier>`

### Raccourcis

- [ ] Assigner un bouton de souris comme raccourci
- [ ] Assigner une touche F13-F24
- [ ] Assigner une touche OEM (ex: -, =, [, ;)
- [ ] Tester le raccourci d'annulation d'enregistrement

### Regles de formatage

- [ ] Creer une regle avec une expression reguliere
- [ ] Verifier que la regex est correctement appliquee
- [ ] Reordonner les regles par glisser-deposer
- [ ] Tester la correction de texte court : dicter un seul mot, verifier minuscule et pas de ponctuation

### Dictionnaire

- [ ] Ajouter un mot et verifier qu'il est corrige en transcription
- [ ] Importer/exporter le dictionnaire
- [ ] Supprimer toutes les entrees

### Interface et systeme

- [ ] Desactiver le demarrage auto, le reactiver, redemarrer et verifier que l'app demarre reduite
- [ ] Verifier la coherence du mode sombre
- [ ] Cliquer sur "Notes de version" dans la barre laterale
- [ ] Debrancher un microphone selectionne, verifier que le choix est preserve

## Signaler un bug

Apres vos tests, [ouvrez une issue GitHub](https://github.com/Kieirra/murmure/issues/new) avec :

- **OS** : Windows / macOS (Intel/Silicon) / Linux (distribution)
- **Version** : La version beta testee
- **Description** : Qu'est-ce qui s'est passe ?
- **Etapes de reproduction** : Comment declencher le bug
- **Logs** : Activez le mode debug dans Parametres > Systeme, reproduisez le bug, puis joignez le fichier log

Merci pour votre contribution !
