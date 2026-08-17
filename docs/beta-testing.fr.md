# Beta Testing

Merci de participer au programme beta de Murmure ! Vos retours sont précieux pour fiabiliser l'application avant sa sortie officielle.

## Comment obtenir la beta

Les builds beta sont publiées avant chaque release. Rendez-vous sur la [page des releases GitHub](https://github.com/Kieirra/murmure/releases) et téléchargez la dernière version marquée en pre-release.

## Nouveautés de la 1.11.0

### LLM Connect

- **Transform** : sélectionnez du texte dans n'importe quelle application, appuyez sur `Ctrl+Alt+Shift+1` à `Ctrl+Alt+Shift+4`, et le prompt enregistré du mode s'applique à votre sélection, sans aucune dictée
- Serveurs distants : vous pouvez maintenant saisir un nom de modèle à la main, et un test de connexion échoué ne bloque plus le fournisseur, ce qui permet d'utiliser des serveurs sans endpoint `/models` (comme l'API Claude)
- Le paramètre temperature n'est plus envoyé aux serveurs distants, ce qui corrige l'erreur 400 Bad Request renvoyée par les modèles OpenAI GPT-5

### Dictionnaire

- Refonte légère, avec un indicateur du nombre de mots : vert jusqu'à 50 mots, jaune de 51 à 100, rouge au-delà de 100
- Les entrées de deux mots sont désormais acceptées, donc les expressions avec un espace ou un tiret peuvent être ajoutées
- Tous les caractères du vocabulaire de Parakeet sont autorisés, plus seulement les lettres
- Exportez votre dictionnaire en `.txt`, et importez-en un avec un format documenté

### Raccourcis

- Les touches Pause et Arrêt défil peuvent servir de raccourci sous Windows et Linux
- La touche fn, marquée d'une icône de globe sur les Mac récents, peut maintenant être associée à un raccourci sur macOS
- Les touches sont lues depuis le backend natif, donc `F13` ne s'affiche plus comme « Unidentified » et les libellés de lettres correspondent à votre vraie disposition clavier sous X11

### Audio

- Le volume système peut baisser pendant l'enregistrement, pour mieux vous entendre
- La détection du niveau du micro s'adapte à votre gain et au bruit ambiant
- Chaque morceau audio est complété par du silence avant la transcription, ce qui corrige les résultats silencieusement tronqués

### Linux

- Nouveau paquet pacman pour les distributions basées sur Arch, dont CachyOS
- Wayland : les caractères accentués sont tapés nativement en mode d'insertion direct
- Murmure apparaît désormais dans la catégorie Utilitaires des menus d'applications

### API et CLI

- L'API locale accepte l'audio de n'importe quelle durée, tant que la requête reste sous 100 Mo, et arrête la transcription dès que le client se déconnecte
- Nouveau drapeau `--hidden` pour démarrer Murmure sans afficher la fenêtre

### Divers

- Les logs ne grossissent plus indéfiniment : le fichier est réinitialisé au-delà de 1 Mo pendant que l'application tourne, et un nouveau niveau Off désactive complètement la journalisation
- Une image copiée reste dans votre presse-papiers quand vous dictez, au lieu d'être remplacée
- L'écoute du mot d'activation ne redémarre plus en boucle quand le micro est indisponible

## Plan de test

Faites ce que vous pouvez, même une seule case nous aide. Commencez par les quatre essentiels, ils prennent environ cinq minutes.

### Les essentiels

- [ ] Dictez une phrase comme d'habitude, et vérifiez que le texte arrive correctement
- [ ] Sélectionnez du texte dans une application, appuyez sur `Ctrl+Alt+Shift+1`, et vérifiez que le prompt s'applique à votre sélection (configurez le mode 1 dans LLM Connect si ce n'est pas déjà fait)
- [ ] Ajoutez une entrée de deux mots à votre dictionnaire, par exemple un prénom et un nom, puis dictez-la
- [ ] Activez la baisse du volume dans Réglages > Système, lancez de la musique, puis dictez

### Si vous avez plus de temps

- [ ] Changez un de vos raccourcis dans les réglages, puis utilisez-le
- [ ] Copiez une image, puis dictez, et vérifiez que l'image est toujours dans votre presse-papiers
- [ ] Dictez un texte long, plus d'une minute, et vérifiez qu'il ne manque rien à la fin
- [ ] Ouvrez le dictionnaire et vérifiez l'indicateur du nombre de mots (vert jusqu'à 50 mots, jaune de 51 à 100, rouge au-delà de 100)

### Uniquement la ligne qui correspond à votre configuration

- [ ] macOS : associez la touche fn (celle avec l'icône de globe) à un raccourci, puis utilisez-la
- [ ] Linux Wayland : dictez une phrase avec des caractères accentués en mode d'insertion direct
- [ ] Arch ou CachyOS : installez le paquet `.pkg.tar.zst` et lancez l'application

## Signaler un bug

Pas besoin d'ouvrir une issue GitHub, répondez simplement dans la conversation d'annonce de la beta. Dites-nous ce qui a cassé et sur quel OS, c'est déjà suffisant.

Si vous le pouvez, ajoutez les étapes pour le reproduire et le fichier de log (activez le mode debug dans Réglages > Système, puis reproduisez le bug).

Merci pour votre contribution !
