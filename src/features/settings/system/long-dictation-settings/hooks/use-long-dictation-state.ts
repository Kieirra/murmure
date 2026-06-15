import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useLongDictationState = () => {
    const [longDictationEnabled, setLongDictationEnabled] = useState(false);
    const [longDictationSilenceMs, setLongDictationSilenceMs] = useState(800);
    const { t } = useTranslation();
    const showSaveError = () => toast.error(t('Failed to save overlay settings'));

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            if (typeof settings.long_dictation_enabled === 'boolean')
                setLongDictationEnabled(settings.long_dictation_enabled);
            if (typeof settings.long_dictation_silence_ms === 'number')
                setLongDictationSilenceMs(settings.long_dictation_silence_ms);
        });
    }, []);

    return {
        longDictationEnabled,
        setLongDictationEnabled: (enabled: boolean) => {
            setLongDictationEnabled(enabled);
            invoke('set_long_dictation_enabled', { enabled }).catch(showSaveError);
        },
        longDictationSilenceMs,
        setLongDictationSilenceMs: (ms: number) => {
            setLongDictationSilenceMs(ms);
            invoke('set_long_dictation_silence_ms', { ms }).catch(showSaveError);
        },
    };
};
