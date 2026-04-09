# Smart Speech Mic

![Smart Speech Mic](../assets/smart-mic.png)

Smart Speech Mic transforme n'importe quel smartphone en microphone sans fil pour Murmure. Aucune installation requise sur le telephone - scannez simplement un QR code.

## Fonctionnement

1. Murmure demarre un serveur HTTPS local sur votre ordinateur
2. Un QR code s'affiche dans l'application
3. Vous scannez le QR code avec votre telephone
4. Votre telephone ouvre une page web qui diffuse l'audio vers Murmure via WebSocket
5. Murmure utilise l'audio du telephone comme entree microphone

## Configuration

1. Allez dans **Extensions** > **Smart Speech Mic**
2. Activez Smart Speech Mic
3. Un QR code apparait
4. Scannez-le avec votre telephone (les deux appareils doivent etre sur le meme reseau)
5. Autorisez l'acces au microphone dans le navigateur du telephone
6. Enregistrez dans Murmure comme d'habitude

## Pre-requis

- Les deux appareils doivent etre sur le **meme reseau local** (Wi-Fi)
- Votre telephone a besoin d'un navigateur moderne (Chrome, Safari, Firefox)
- La permission microphone doit etre accordee dans le navigateur

## Securite

- Communication chiffree via TLS (HTTPS + WSS)
- Certificats auto-signes generes localement
- Appairage par tokens stockes dans le trousseau natif du systeme
- Aucune donnee ne quitte votre reseau local

## Gestion des appareils appaires

Vous pouvez voir et supprimer les appareils appaires dans les parametres Smart Speech Mic. Supprimer un appareil revoque son token.

## Configuration

- **Port** : Le port du serveur peut etre change si le port par defaut entre en conflit
- **Activer/Desactiver** : Activez ou desactivez le serveur Smart Mic selon vos besoins
