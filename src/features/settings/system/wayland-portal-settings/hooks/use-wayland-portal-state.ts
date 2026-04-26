import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useWaylandPortalState = () => {
    const [useWaylandPortal, setUseWaylandPortal] = useState<boolean>(false);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings')
            .then((settings) => setUseWaylandPortal(settings.use_wayland_portal))
            .catch((error) => console.error('Failed to load Wayland portal state:', error));
    }, []);

    const handleSetUseWaylandPortal = async (enabled: boolean) => {
        try {
            setUseWaylandPortal(enabled);
            await invoke('set_use_wayland_portal', { enabled });
        } catch (error) {
            console.error('Failed to set Wayland portal:', error);
            toast.error(t('Failed to save Wayland portal setting'));
            setUseWaylandPortal(!enabled);
        }
    };

    return {
        useWaylandPortal,
        setUseWaylandPortal: handleSetUseWaylandPortal,
    };
};
