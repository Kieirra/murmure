import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

import fr from './locales/fr.json';

i18n.use(LanguageDetector)
    .use(initReactI18next)
    .init({
        resources: {
            fr: { translation: fr },
        },
        ns: ['translation'],
        defaultNS: 'translation',
        keySeparator: false,
        nsSeparator: false,
        fallbackLng: false,
        interpolation: {
            escapeValue: false,
        },
        detection: {
            order: ['localStorage', 'navigator'],
            caches: ['localStorage'],
        },
    });

// Initialize language from Tauri settings on startup
if (typeof window !== 'undefined') {
    import('@tauri-apps/api/core').then(({ invoke }) => {
        invoke<string>('get_current_language')
            .then((lang) => {
                if (lang != null && lang.length > 0 && lang !== i18n.language) {
                    i18n.changeLanguage(lang);
                }
            })
            .catch(() => {
                // If command fails, use default detection
            });
    });
}

export default i18n;
