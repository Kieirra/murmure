# Installation macOS

## Telechargement

=== "Apple Silicon (M1/M2/M3/M4)"

    1. Telechargez **Murmure_aarch64_darwin.dmg** depuis le [site officiel](https://murmure.al1x-ai.com/) (ou [GitHub Releases](https://github.com/Kieirra/murmure/releases))
    2. Ouvrez le DMG et glissez Murmure dans votre dossier Applications
    3. Ouvrez Murmure depuis Applications

=== "Intel"

    1. Telechargez **Murmure_x86_64_darwin.dmg** depuis le [site officiel](https://murmure.al1x-ai.com/) (ou [GitHub Releases](https://github.com/Kieirra/murmure/releases))
    2. Ouvrez le DMG et glissez Murmure dans votre dossier Applications
    3. Ouvrez Murmure depuis Applications

## Permissions requises

Murmure a besoin de trois permissions macOS :

1. **Microphone** - Pour capturer votre voix
2. **Accessibilite** - Pour simuler les frappes clavier et coller le texte
3. **Surveillance des entrees** - Pour detecter les raccourcis clavier et empecher qu'ils soient envoyes a l'application active

Au premier lancement, macOS vous demandera automatiquement d'accorder ces permissions. Apres les avoir accordees, **redemarrez Murmure**.

!!! tip "Quelque chose ne fonctionne pas ?"
Si les raccourcis ou la transcription ne fonctionnent pas, vous avez peut-etre refuse une permission par erreur. Allez dans **Reglages Systeme** > **Confidentialite et securite** et verifiez que Murmure est liste et active sous :

    - Microphone
    - Accessibilite
    - Surveillance des entrees

## Mise a jour depuis 1.6.0

!!! warning "Important : reinitialisation des permissions requise"
Si vous mettez a jour depuis la version 1.6.0, la signature du code a change. macOS les traite comme des applications differentes, vous devez reinitialiser completement les permissions.

Suivez ces etapes **dans cet ordre exact** :

1. **Supprimez** Murmure de Reglages Systeme > Confidentialite et securite > **Accessibilite** (pas juste decocher - supprimez-le de la liste)
2. **Supprimez** Murmure de Reglages Systeme > Confidentialite et securite > **Surveillance des entrees**
3. **Installez** la nouvelle version
4. **Lancez** Murmure
5. **Accordez** la permission Accessibilite
6. **Accordez** la permission Surveillance des entrees
7. **Redemarrez** Murmure

C'est une procedure unique. Les futures mises a jour ne le requerront pas.

## Raccourcis recommandes

Le raccourci par defaut `Ctrl+Espace` entre en conflit avec le changement de source d'entree macOS. Nous recommandons :

- `Ctrl+Option+M`
- `F2`, `F3`, ou une autre touche de fonction
- Un bouton de souris

!!! warning "Evitez les raccourcis avec Espace ou des chiffres"
Sur macOS, les raccourcis contenant `Espace` ou des chiffres peuvent inserer ces caracteres dans l'application active. Utilisez des combinaisons de modificateurs ou des touches de fonction.

## Notes specifiques macOS

- **Nom du developpeur dans les notifications** : macOS peut afficher le nom personnel du developpeur au lieu de "Murmure". C'est une limitation macOS pour les certificats individuels.
- **Visibilite dans le Dock** : Vous pouvez masquer Murmure du Dock dans Parametres > Systeme > "Afficher dans le Dock".

## Emplacement des parametres

```
~/Library/Application Support/com.al1x-ai.murmure/settings.json
```
