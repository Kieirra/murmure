import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useHistoryPersistenceState = () => {
    const [persistHistory, setPersistHistory] = useState<boolean>(false);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            setPersistHistory(settings.persist_history);
        });
    }, []);

    const handleSetPersistHistory = async (enabled: boolean) => {
        try {
            setPersistHistory(enabled);
            await invoke('set_persist_history', { enabled });
        } catch {
            toast.error(t('Failed to save history setting'));
            setPersistHistory(!enabled);
        }
    };

    return {
        persistHistory,
        setPersistHistory: handleSetPersistHistory,
    };
};
