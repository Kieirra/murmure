import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export const useWakeWordEnabled = () => {
    const [enabled, setEnabled] = useState(false);

    useEffect(() => {
        invoke<boolean>('get_wake_word_enabled')
            .then(setEnabled)
            .catch((err) => console.error('Failed to load wake word enabled:', err));
    }, []);

    const updateEnabled = async (value: boolean) => {
        try {
            await invoke('set_wake_word_enabled', { enabled: value });
            setEnabled(value);
        } catch (err) {
            console.error('Failed to set wake word enabled:', err);
        }
    };

    return { enabled, setEnabled: updateEnabled };
};
