import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

interface Settings {
    show_in_dock?: boolean;
}

export const useDockSettingsState = () => {
    const [showInDock, setShowInDock] = useState(true);

    useEffect(() => {
        // We use get_all_settings to get the initial state
        invoke<Settings>('get_all_settings').then((settings) => {
            if (settings && typeof settings.show_in_dock === 'boolean') {
                setShowInDock(settings.show_in_dock);
            }
        });
    }, []);

    const setDockVisibility = async (show: boolean) => {
        try {
            await invoke('set_show_in_dock', { show });
            setShowInDock(show);
        } catch (error) {
            console.error('Failed to set dock visibility:', error);
        }
    };

    return {
        showInDock,
        setDockVisibility,
    };
};
