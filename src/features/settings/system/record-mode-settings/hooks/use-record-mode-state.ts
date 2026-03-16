import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export type RecordMode = 'push_to_talk' | 'toggle_to_talk';

export const useRecordModeState = () => {
    const [recordMode, setRecordMode] = useState<RecordMode>('push_to_talk');
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            const mode = settings.record_mode;
            if (mode === 'push_to_talk' || mode === 'toggle_to_talk') {
                setRecordMode(mode as RecordMode);
            }
        });
    }, []);

    return {
        recordMode,
        setRecordMode: (mode: RecordMode) => {
            setRecordMode(mode);
            invoke('set_record_mode', { mode: mode }).catch(() => {
                toast.error(t('Failed to save record mode'));
            });
        },
    };
};
