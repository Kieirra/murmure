import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { AppSettings } from '@/features/settings/settings.types';

export const useWaylandNoticeState = () => {
    const [dismissed, setDismissed] = useState(true);

    useEffect(() => {
        invoke<AppSettings>('get_all_settings')
            .then((settings) => setDismissed(settings.wayland_notice_dismissed))
            .catch((error) => {
                console.error('Failed to load Wayland notice dismissed flag:', error);
            });
    }, []);

    const dismiss = async () => {
        setDismissed(true);
        try {
            await invoke('dismiss_wayland_notice');
        } catch (error) {
            console.error('Failed to persist Wayland notice dismissed flag:', error);
            setDismissed(false);
        }
    };

    return { dismissed, dismiss };
};
