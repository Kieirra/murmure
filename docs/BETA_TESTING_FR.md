# Guide du Bêta Testeur - Murmure v1.8.0

Merci de participer au programme de bêta testing de Murmure ! Votre contribution est essentielle pour améliorer la qualité de l'application avant sa sortie officielle.

---

## Comment s'inscrire au programme Bêta Testing ?

Envoyez un message sur LinkedIn à [Luc Marongiu](https://www.linkedin.com/in/luc-m-4b309aa8/) avec votre système d'exploitation (Windows, macOS ou Linux).

Vous recevrez ensuite le lien de téléchargement de la version bêta.

---

## Plan de test

Testez ce que vous pouvez, pas de pression :

### Installation et démarrage

- [ ] Télécharger et installer la version bêta 1.8.0
- [ ] Vérifier que l'application démarre correctement
- [ ] Compléter l'onboarding initial

### Mode vocal (#171, #178)

- [ ] Activer le mode vocal dans les paramètres
- [ ] Prononcer le mot d'activation pour déclencher un enregistrement
- [ ] Tester l'auto-envoi Enter après transcription vocale (#156)
- [ ] Démarrer un enregistrement au clavier, puis utiliser les mots vocaux pour valider/annuler
- [ ] Vérifier que le mode vocal se désactive/réactive correctement

### LLM Connect : Serveur distant

- [ ] Configurer une connexion à un serveur distant (API OpenAI-compatible)
- [ ] Tester une transcription avec post-traitement LLM distant
- [ ] Créer plusieurs modes LLM avec des fournisseurs différents (local Ollama + distant)
- [ ] Réordonner les modes LLM par glisser-déposer (#104)
- [ ] Vérifier que le bon fournisseur est utilisé pour chaque mode
- [ ] Tester avec un modèle de raisonnement (Qwen 3.5, Ministral) et vérifier que la vitesse de réponse est acceptable

### Import/Export des paramètres

- [ ] Ouvrir Paramètres > Import/Export
- [ ] Exporter tous les paramètres
- [ ] Exporter uniquement certains paramètres (export partiel)
- [ ] Modifier un paramètre, puis importer le fichier exporté
- [ ] Vérifier que les paramètres sont restaurés correctement
- [ ] (Linux/macOS) Tester l'import CLI : `murmure import <fichier>` (#223)
- [ ] (Windows) Tester l'import CLI : `murmure.exe import <fichier>` (#223)

### Raccourcis

- [ ] Assigner un bouton de souris comme raccourci (#158)
- [ ] Vérifier que le bouton de souris déclenche l'action correctement
- [ ] Tester le raccourci d'annulation dans l'overlay (#161)
- [ ] Assigner une touche F13-F24 comme raccourci (#189)
- [ ] Assigner une touche OEM (ex: -, =, [, ;) comme raccourci

### Règles de formatage

- [ ] Créer une règle avec une expression régulière (#105)
- [ ] Vérifier que la regex s'applique correctement à la transcription
- [ ] Vérifier les nouveaux libellés de règles (phrases lisibles) (#101)
- [ ] Survoler l'icône "?" du champ texte de remplacement et vérifier l'aide
- [ ] Tester la correction courte : dicter un mot seul, vérifier la minuscule et l'absence de ponctuation
- [ ] Réordonner les règles par glisser-déposer (#170)
- [ ] Vérifier que l'ordre d'application respecte le nouvel ordre

### Interface et système

- [ ] Désactiver le démarrage automatique, le réactiver, puis redémarrer et vérifier que l'app démarre minimisée dans le tray (#201)
- [ ] (macOS) Configurer l'affichage/masquage dans le Dock (#226)
- [ ] Consulter la page "À propos" et vérifier la nouvelle interface (#198)
- [ ] Vérifier la cohérence des couleurs en dark mode
- [ ] Cliquer sur le lien "Notes de version" dans la sidebar
- [ ] Débrancher un microphone sélectionné, vérifier que le choix est conservé

---

## Rapport de Bêta Testing

Après vos tests, envoyez un rapport avec :

### Infos

- **Pseudo / Nom** :
- **OS** : Windows / macOS / Linux (version)

### Tests effectués

- [ ] Installation et démarrage
- [ ] Mode vocal
- [ ] LLM Connect : Serveur distant
- [ ] Import/Export des paramètres
- [ ] Raccourcis
- [ ] Règles de formatage
- [ ] Interface et système

### Bugs trouvés

Pour chaque bug :

- **Description** : Que s'est-il passé ?
- **Comment reproduire** : Étapes pour reproduire le bug

---

Merci pour votre contribution !
