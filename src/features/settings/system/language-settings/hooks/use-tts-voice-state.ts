import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useTtsVoiceState = () => {
    const [ttsVoice, setTtsVoiceState] = useState<string>('af_heart');
    const { t } = useTranslation();

    useEffect(() => {
        const loadTtsVoice = async () => {
            try {
                const savedVoice = await invoke<string>('get_tts_voice');
                if (savedVoice) {
                    setTtsVoiceState(savedVoice);
                }
            } catch (error) {
                console.error('Failed to load tts voice:', error);
            }
        };
        loadTtsVoice();
    }, []);

    const setTtsVoice = async (voice: string) => {
        try {
            await invoke('set_tts_voice', { voice });
            setTtsVoiceState(voice);
        } catch (error) {
            console.error('Failed to save tts voice:', error);
            toast.error(t('Failed to save tts voice'));
        }
    };

    return {
        ttsVoice,
        setTtsVoice,
    };
};
