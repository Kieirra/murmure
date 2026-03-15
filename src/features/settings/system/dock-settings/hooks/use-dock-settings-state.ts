import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useDockSettingsState = () => {
    const [showInDock, setShowInDock] = useState(true);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            setShowInDock(settings.show_in_dock);
        });
    }, []);

    const setDockVisibility = async (show: boolean) => {
        try {
            await invoke('set_show_in_dock', { show });
            setShowInDock(show);
        } catch (error) {
            console.error('Failed to set dock visibility:', error);
            toast.error(t('Failed to save dock setting'));
        }
    };

    return {
        showInDock,
        setDockVisibility,
    };
};
