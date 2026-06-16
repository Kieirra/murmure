import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings, LONG_DICTATION_ENABLED_EVENT } from '@/features/settings/settings.types';

export const useLongDictationState = () => {
    const [longDictationEnabled, setLongDictationEnabled] = useState(false);
    const [longDictationSilenceMs, setLongDictationSilenceMs] = useState(500);
    const { t } = useTranslation();
    const showSaveError = () => toast.error(t('Failed to save long dictation settings'));

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
            globalThis.dispatchEvent(new CustomEvent(LONG_DICTATION_ENABLED_EVENT, { detail: enabled }));
        },
        longDictationSilenceMs,
        setLongDictationSilenceMs: (ms: number) => {
            setLongDictationSilenceMs(ms);
            invoke('set_long_dictation_silence_ms', { ms }).catch(showSaveError);
        },
    };
};
