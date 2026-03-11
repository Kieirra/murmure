# Guide du Bêta Testeur - Murmure v1.7.0

Merci de participer au programme de bêta testing de Murmure ! Votre contribution est essentielle pour améliorer la qualité de l'application avant sa sortie officielle.

---

## Comment s'inscrire au programme Bêta Testing ?

Envoyez un message sur LinkedIn à [Luc Marongiu](https://www.linkedin.com/in/luc-m-4b309aa8/) avec votre système d'exploitation (Windows, macOS ou Linux).

Vous recevrez ensuite le lien de téléchargement de la version bêta.

---

## Fonctionnalités de la version 1.7.0 à tester

### Paramètres système

| Fonctionnalité                         | Description                                                                                       | Issue |
| -------------------------------------- | ------------------------------------------------------------------------------------------------- | ----- |
| **Sélection du microphone**            | Choisir le périphérique d'entrée audio                                                            | #81   |
| **Niveaux de verbosité des logs**      | Configurer le niveau de détail des logs (trace, debug, info, warn, error)                         | #90   |
| **Bouton d'ouverture du dossier logs** | Accéder rapidement aux fichiers de log                                                            | #130  |
| **Mode d'insertion de texte**          | Trois modes : Standard (Ctrl+V), Terminal (Ctrl+Shift+V), Direct (frappe caractère par caractère) | #121  |

### Dictionnaire personnalisé

| Fonctionnalité               | Description                                                    | Issue |
| ---------------------------- | -------------------------------------------------------------- | ----- |
| **Import/Export CSV**        | Exporter et importer des dictionnaires au format CSV           | #72   |
| **Présets médicaux**         | Dictionnaires pré-configurés pour vocabulaire spécialisé       | #72   |
| **Préservation de la casse** | Support du respect de la casse pour les entrées personnalisées | #109  |

### Commandes texte

| Fonctionnalité               | Description                                                                                     | Issue      |
| ---------------------------- | ----------------------------------------------------------------------------------------------- | ---------- |
| **Commandes personnalisées** | Sélectionner du texte transcrit et appliquer des commandes (corriger grammaire, traduire, etc.) | #107, #122 |

### Intégration LLM

| Fonctionnalité                    | Description                                         | Issue |
| --------------------------------- | --------------------------------------------------- | ----- |
| **Prompts multiples sauvegardés** | Créer et gérer plusieurs configurations de prompts  | #110  |
| **Raccourcis changement de mode** | Basculer entre les modes LLM via raccourcis clavier | #110  |

### Formatage du texte

| Fonctionnalité                       | Description                                                                                | Issue |
| ------------------------------------ | ------------------------------------------------------------------------------------------ | ----- |
| **Seuil de conversion des chiffres** | Configurer à partir de quel nombre les mots sont convertis en chiffres (ex: "trois" → "3") | #106  |

### Améliorations techniques

| Fonctionnalité                 | Description                                               | Issue |
| ------------------------------ | --------------------------------------------------------- | ----- |
| **Raccourcis Windows**         | Correction de la gestion des raccourcis sous Windows      | #128  |
| **Refactoring des raccourcis** | Optimisation de la logique des raccourcis clavier         | #123  |
| **Mises à jour de sécurité**   | Dépendances mises à jour pour corriger les vulnérabilités | #117  |
| **Installeur NSIS Windows**    | Installation sans privilèges administrateur               | #96   |

---

## Plan de test

Testez ce que vous pouvez, pas de pression :

### Installation et démarrage

- [ ] Télécharger et installer la version bêta 1.7.0
- [ ] Vérifier que l'application démarre correctement
- [ ] Vérifier le téléchargement automatique du modèle IA (si premier lancement)
- [ ] Compléter l'onboarding initial

### Sélection du microphone (#81)

- [ ] Ouvrir Paramètres > Système > Microphone
- [ ] Vérifier que la liste des microphones disponibles s'affiche
- [ ] Sélectionner un microphone différent
- [ ] Tester l'enregistrement avec le nouveau microphone
- [ ] Vérifier que le choix est conservé après redémarrage

### Niveaux de verbosité des logs (#90)

- [ ] Ouvrir Paramètres > Système > Logs
- [ ] Changer le niveau de log (trace, debug, info, warn, error)
- [ ] Vérifier l'avertissement pour les niveaux sensibles (debug/trace)
- [ ] Cliquer sur le bouton "Ouvrir le dossier logs"
- [ ] Vérifier que les logs correspondent au niveau sélectionné

### Mode d'insertion de texte (#121)

- [ ] Ouvrir Paramètres > Système > Mode d'insertion
- [ ] Tester le mode "Standard (Ctrl+V)" dans un éditeur de texte
- [ ] Tester le mode "Terminal (Ctrl+Shift+V)" dans un terminal
- [ ] Tester le mode "Direct (frappe)" dans une application

### Import/Export du dictionnaire (#72)

- [ ] Ouvrir Paramètres > Dictionnaire personnalisé
- [ ] Ajouter quelques mots au dictionnaire
- [ ] Exporter le dictionnaire au format CSV
- [ ] Vérifier le contenu du fichier CSV exporté
- [ ] Supprimer les mots du dictionnaire
- [ ] Importer le fichier CSV précédemment exporté
- [ ] Vérifier que les mots sont restaurés

### Préservation de la casse (#109)

- [ ] Ajouter un mot avec une casse spécifique (ex: "iPhone", "macOS")
- [ ] Faire une transcription contenant ce mot
- [ ] Vérifier que la casse est respectée dans le résultat

### Commandes personnalisées (#107, #122)

- [ ] Faire une transcription
- [ ] Sélectionner une partie du texte transcrit
- [ ] Appliquer une commande personnalisée (traduire en anglais)
- [ ] Vérifier le résultat de la commande

### Prompts LLM multiples (#110)

- [ ] Ouvrir la configuration LLM
- [ ] Créer un premier prompt personnalisé
- [ ] Créer un deuxième prompt différent
- [ ] Utiliser les raccourcis clavier pour basculer entre les modes
- [ ] Vérifier que le bon prompt est appliqué lors de la transcription
- [ ] Vérifier la persistence des prompts après redémarrage

### Seuil de conversion des chiffres (#106)

- [ ] Ouvrir Paramètres > Règles de formatage
- [ ] Seuil à 0 : "un chien et deux chats" → "1 chien et 2 chats"
- [ ] Seuil à 3 : "un chien et deux chats" → "un chien et deux chats"

---

## Rapport de Bêta Testing

Après vos tests, envoyez un rapport avec :

### Infos

- **Pseudo** :
- **OS** : Windows / macOS / Linux (version)

### Bugs trouvés

Pour chaque bug :

- **Description** : Que s'est-il passé ?
- **Comment reproduire** : Étapes pour reproduire le bug

### Ergonomie

Si vous avez remarqué des améliorations UX importantes à faire (pas des bugs, mais des points bloquants ou frustrants pour l'utilisateur) :

- ...

---

Merci pour votre contribution !
