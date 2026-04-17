export type Lang = 'en' | 'fr';

export const STRINGS = {
    en: {
        'tabs.remote': 'Remote',
        'tabs.transcription': 'Transcription',
        'tabs.translation': 'Translation',

        'remote.empty': 'Transcription will appear here',
        'remote.copied': 'Copied!',
        'remote.trackpad.tap': 'Tap . Click',
        'remote.trackpad.hold': 'Hold . Right click',
        'remote.trackpad.scroll': '2 fingers . Scroll',
        'remote.rec.cancel': 'Cancel',
        'remote.rec.stop': 'Stop & Send',
        'remote.rec.enter': 'Enter',

        'transcription.empty': 'Press REC to start dictating',
        'transcription.copyAll': 'Copy all',
        'transcription.copied': 'Copied!',
        'transcription.clear': 'Clear history',
        'transcription.clearConfirm': 'Clear all transcriptions?',

        'translation.empty': 'Tap REC to start a bilingual conversation',
        'translation.languages': 'Languages:',
        'translation.translating': 'Translating...',
        'translation.unavailable': 'Translation unavailable',
        'translation.recHint': 'Tap to talk',
        'translation.chooseLang': 'Choose language',
        'translation.closeLangPicker': 'Close',

        'status.menu.reload': 'Reload',
        'status.menu.options': 'Options',

        'errors.micDenied': 'Microphone access denied. Please allow it in your browser settings.',
        'errors.micGeneric': 'Unable to access microphone.',
        'errors.micError': 'Unable to access microphone: {err}',
        'errors.forceDisconnect': 'Another device took over.',
        'errors.title': 'Error',
        'errors.disconnected': 'Disconnected',

        'status.connected': 'Connected',
        'status.connecting': 'Connecting...',

        'conflict.title': 'Device already connected',
        'conflict.message': 'The device "{name}" is already connected.',
        'conflict.cancel': 'Cancel',
        'conflict.takeOver': 'Take control',
    },
    fr: {
        'tabs.remote': 'Remote',
        'tabs.transcription': 'Transcription',
        'tabs.translation': 'Traduction',

        'remote.empty': 'La transcription apparaitra ici',
        'remote.copied': 'Copie !',
        'remote.trackpad.tap': 'Tap . Clic',
        'remote.trackpad.hold': 'Appui long . Clic droit',
        'remote.trackpad.scroll': '2 doigts . Defilement',
        'remote.rec.cancel': 'Annuler',
        'remote.rec.stop': 'Stop et envoyer',
        'remote.rec.enter': 'Entree',

        'transcription.empty': 'Appuyez sur REC pour dicter',
        'transcription.copyAll': 'Tout copier',
        'transcription.copied': 'Copie !',
        'transcription.clear': "Effacer l'historique",
        'transcription.clearConfirm': 'Effacer toutes les transcriptions ?',

        'translation.empty': 'Appuyez sur REC pour demarrer une conversation bilingue',
        'translation.languages': 'Langues :',
        'translation.translating': 'Traduction...',
        'translation.unavailable': 'Traduction indisponible',
        'translation.recHint': 'Appuyez pour parler',
        'translation.chooseLang': 'Choisir la langue',
        'translation.closeLangPicker': 'Fermer',

        'status.menu.reload': 'Recharger',
        'status.menu.options': 'Options',

        'errors.micDenied': "Acces au micro refuse. Veuillez l'autoriser dans les parametres du navigateur.",
        'errors.micGeneric': "Impossible d'acceder au micro.",
        'errors.micError': "Impossible d'acceder au micro : {err}",
        'errors.forceDisconnect': 'Un autre appareil a pris le controle.',
        'errors.title': 'Erreur',
        'errors.disconnected': 'Deconnecte',

        'status.connected': 'Connecte',
        'status.connecting': 'Connexion...',

        'conflict.title': 'Appareil deja connecte',
        'conflict.message': 'L\'appareil "{name}" est deja connecte.',
        'conflict.cancel': 'Annuler',
        'conflict.takeOver': 'Prendre le controle',
    },
} as const;

export type StringKey = keyof (typeof STRINGS)['en'];
