# Beta Testing

Merci de participer au programme beta de Murmure ! Vos retours sont précieux pour fiabiliser l'application avant sa sortie officielle.

## Comment obtenir la beta

Les builds beta sont publiées avant chaque release. Rendez-vous sur la [page des releases GitHub](https://github.com/Kieirra/murmure/releases) et téléchargez la dernière version marquée en pre-release.

## Nouveautés de la 2.0.0

### Nouveau modèle de transcription

- Murmure utilise maintenant Parakeet ultra, une version de Parakeet améliorée par Moondream, qui fait moins d'erreurs
- Le modèle est l'export int8 de @thiswillbeyourgithub, qui utilise environ 380 Mo de RAM de moins que le précédent

### Résultat après la dictée

- L'overlay peut rester affiché après une dictée, avec le texte et un bouton pour le copier. Choisissez quand dans Paramètres > Système > **Afficher le résultat après la dictée** (Désactivé, Commande, Commande et LLM, Toutes les dictées), et choisissez combien de temps il reste affiché
- Passez la souris sur le résultat pour le garder à l'écran

### Insertion automatique

- Nouvel interrupteur **Insertion automatique** dans Paramètres > Système, séparé de la méthode d'insertion. Quand il est désactivé, Murmure ne tape jamais le texte de lui-même. Le texte reste dans l'historique, et dans le presse-papiers si **Copier dans le presse-papiers** est activé
- Le raccourci **Coller la dernière transcription** colle toujours, même quand l'insertion automatique est désactivée
- Si vous utilisiez la méthode d'insertion « Aucune (collage manuel) », elle est convertie pour vous. L'insertion automatique est désactivée et vos autres réglages sont conservés

### Mode Prompt et Mode Commande

- LLM Connect s'appelle maintenant Mode Prompt
- Le Mode Commande devient une extension à part, avec son propre modèle. Activez-le dans Extensions > Mode Commande
- Sans sélection, une commande répond maintenant à votre question au lieu de la répéter

### API locale

- L'API locale n'est plus expérimentale. Elle a sa propre page dans Extensions > API locale
- De nouveaux endpoints appliquent un prompt du Mode Prompt à la transcription et renvoient le texte final
- Chaque endpoint a un bouton **Essayer** pour le tester avec un fichier WAV, sans écrire de code

### Divers

- La suppression des sons d'hésitation (« euh », « um ») est maintenant optionnelle et désactivée par défaut, car elle supprimait aussi de vrais mots comme la préposition allemande « um ». Réactivez-la dans Personnaliser > Règles de formatage
- Le menu de la barre système est traduit dans votre langue
- Nouveau drapeau `--quit` en ligne de commande pour fermer Murmure
- Les transcriptions ne contiennent plus de jeton `<unk>` parasite
- macOS : cliquer sur l'icône du Dock rouvre la fenêtre
- Linux : le volume d'un son lancé pendant une dictée, comme un nouvel onglet de navigateur, est bien rétabli à la fin
- Les mises à jour téléchargées en `.deb` gardent la bonne extension de fichier
- Plusieurs améliorations de sécurité

## Plan de test

Faites ce que vous pouvez, même une seule case nous aide. Commencez par les quatre essentiels, ils prennent environ cinq minutes.

### Les essentiels

- [ ] Dictez quelques phrases comme d'habitude, et vérifiez que le texte arrive correctement et sans erreur
- [ ] Dans Paramètres > Système, réglez **Afficher le résultat après la dictée** sur **Toutes les dictées**, dictez, et vérifiez que le résultat s'affiche avec un bouton pour copier, puis disparaît après la durée choisie
- [ ] Désactivez **Insertion automatique**, dictez, et vérifiez que rien n'est tapé mais que le résultat s'affiche quand même. Utilisez ensuite le raccourci **Coller la dernière transcription** et vérifiez que le texte est collé
- [ ] Si vous avez mis à jour depuis une ancienne version, vérifiez que vos réglages, raccourcis et votre dictionnaire sont toujours là

### Si vous avez plus de temps

- [ ] Si vous utilisez le Mode Prompt ou le Mode Commande, activez le Mode Commande, appuyez sur le raccourci Commande sans rien sélectionner et posez une question. Sélectionnez ensuite une phrase et dites « traduis en anglais »
- [ ] Ouvrez Extensions > API locale, activez-la, et utilisez **Essayer** avec un court fichier WAV
- [ ] Dictez un texte long, plus d'une minute, et vérifiez qu'il ne manque rien à la fin
- [ ] Ouvrez le menu de la barre système et vérifiez qu'il est dans votre langue

### Uniquement la ligne qui correspond à votre configuration

- [ ] macOS : fermez la fenêtre de Murmure, cliquez sur l'icône du Dock, et vérifiez que la fenêtre revient
- [ ] Linux : activez la baisse du volume dans Paramètres > Système, lancez une vidéo dans un nouvel onglet pendant que vous dictez, et vérifiez que son volume revient à la fin
- [ ] Linux : lancez `murmure --quit` dans un terminal et vérifiez que Murmure se ferme

## Signaler un bug

Pas besoin d'ouvrir une issue GitHub, répondez simplement dans la conversation d'annonce de la beta. Dites-nous ce qui a cassé et sur quel OS, c'est déjà suffisant.

Si vous le pouvez, ajoutez les étapes pour le reproduire et le fichier de log (activez le mode debug dans Réglages > Système, puis reproduisez le bug).

Merci pour votre contribution !
