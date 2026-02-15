import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';

export const useAutoSendEnterState = () => {
    const [autoSendEnter, setAutoSendEnter] = useState<boolean>(false);

    const loadAutoSendEnterState = async () => {
        try {
            const enabled = await invoke<boolean>('get_auto_send_enter');
            setAutoSendEnter(enabled);
        } catch (error) {
            console.error('Failed to load auto send enter state:', error);
        }
    };

    useEffect(() => {
        loadAutoSendEnterState();
    }, []);

    const handleSetAutoSendEnter = async (enabled: boolean) => {
        try {
            setAutoSendEnter(enabled);
            await invoke('set_auto_send_enter', { enabled });
        } catch (error) {
            console.error('Failed to set auto send enter:', error);
            // Revert the state on error
            setAutoSendEnter(!enabled);
        }
    };

    return {
        autoSendEnter,
        setAutoSendEnter: handleSetAutoSendEnter,
    };
};
