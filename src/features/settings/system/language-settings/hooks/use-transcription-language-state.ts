import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useTranscriptionLanguageState = () => {
    const [transcriptionLang, setTranscriptionLang] = useState<string>('auto');
    const { t } = useTranslation();

    useEffect(() => {
        const load = async () => {
            try {
                const saved = await invoke<string>('get_transcription_language');
                setTranscriptionLang(saved || 'auto');
            } catch (error) {
                console.error('Failed to load transcription language:', error);
                toast.error(t('Failed to load transcription language'));
            }
        };
        load();
    }, []);

    const setLanguage = async (lang: string) => {
        try {
            await invoke('set_transcription_language', { lang });
            setTranscriptionLang(lang);
        } catch (error) {
            console.error('Failed to save transcription language:', error);
            toast.error(t('Failed to save transcription language'));
        }
    };

    return {
        transcriptionLang,
        setTranscriptionLanguage: setLanguage,
    };
};
