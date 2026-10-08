import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useAutoInsertState = () => {
    const [autoInsert, setAutoInsert] = useState<boolean>(true);
    const { t } = useTranslation();

    const loadAutoInsertState = async () => {
        try {
            const settings = await invoke<AppSettings>('get_all_settings');
            setAutoInsert(settings.auto_insert);
        } catch (error) {
            console.error('Failed to load automatic insert state:', error);
        }
    };

    useEffect(() => {
        loadAutoInsertState();
    }, []);

    const handleSetAutoInsert = async (enabled: boolean) => {
        try {
            setAutoInsert(enabled);
            await invoke('set_auto_insert', { enabled });
        } catch (error) {
            console.error('Failed to set automatic insert:', error);
            toast.error(t('Failed to save automatic insert setting'));
            setAutoInsert(!enabled);
        }
    };

    return {
        autoInsert,
        setAutoInsert: handleSetAutoInsert,
    };
};
