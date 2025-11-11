import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

import enCommon from './locales/en/common.json';
import enHome from './locales/en/home.json';
import enHistory from './locales/en/history.json';
import enSettings from './locales/en/settings.json';
import enAbout from './locales/en/about.json';
import enNavigation from './locales/en/navigation.json';
import enDictionary from './locales/en/dictionary.json';

import frCommon from './locales/fr/common.json';
import frHome from './locales/fr/home.json';
import frHistory from './locales/fr/history.json';
import frSettings from './locales/fr/settings.json';
import frAbout from './locales/fr/about.json';
import frNavigation from './locales/fr/navigation.json';
import frDictionary from './locales/fr/dictionary.json';
import enUpdate from './locales/en/update.json';
import frUpdate from './locales/fr/update.json';

i18n.use(LanguageDetector)
    .use(initReactI18next)
    .init({
        resources: {
            en: {
                common: enCommon,
                home: enHome,
                history: enHistory,
                settings: enSettings,
                about: enAbout,
                navigation: enNavigation,
                dictionary: enDictionary,
                update: enUpdate,
            },
            fr: {
                common: frCommon,
                home: frHome,
                history: frHistory,
                settings: frSettings,
                about: frAbout,
                navigation: frNavigation,
                dictionary: frDictionary,
                update: frUpdate,
            },
        },
        defaultNS: 'common',
        fallbackLng: 'en',
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
