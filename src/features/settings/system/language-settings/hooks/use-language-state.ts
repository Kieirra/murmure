import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import { useTranslation, i18n } from '@/i18n';

const SUPPORTED_LANGUAGES = [
    { code: 'en', label: 'English' },
    { code: 'fr', label: 'Français' },
];

export const useLanguageState = () => {
    const [currentLang, setCurrentLang] = useState<string>('default');
    const { t } = useTranslation();

    const applyLanguage = (lang: string) => {
        if (lang === 'default') {
            try {
                window.localStorage.removeItem('i18nextLng');
            } catch {}
            const normalize = (code: string) => code.split('-')[0];
            const browserLang =
                (navigator &&
                    (navigator.language ||
                        (navigator.languages && navigator.languages[0]))) ||
                '';
            const detected = browserLang ? normalize(browserLang) : '';
            console.log('detected', detected);
            let target = 'en';
            if (SUPPORTED_LANGUAGES.some((lang) => lang.code === detected)) {
                target = detected;
                console.log('target', target);
            }
            if (target !== i18n.language) {
                i18n.changeLanguage(target);
            }
            setCurrentLang('default');
        } else {
            i18n.changeLanguage(lang);
            setCurrentLang(lang);
        }
    };

    useEffect(() => {
        const loadLanguage = async () => {
            try {
                const savedLang = await invoke<string>('get_current_language');
                applyLanguage(savedLang || 'default');
            } catch (error) {
                console.error('Failed to load language:', error);
                toast.error(t('Failed to load language'), {
                    duration: 30000,
                    closeButton: true,
                });
            }
        };
        loadLanguage();
    }, []);

    const setLanguage = async (lang: string) => {
        try {
            await invoke('set_current_language', { lang });
            applyLanguage(lang);
        } catch (error) {
            console.error('Failed to save language:', error);
            toast.error(t('Failed to save language'), {
                duration: 2000,
                closeButton: true,
            });
        }
    };

    return {
        currentLang,
        setLanguage,
    };
};
