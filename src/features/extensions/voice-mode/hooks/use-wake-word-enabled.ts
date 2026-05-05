import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export const useWakeWordEnabled = () => {
    const [enabled, setEnabled] = useState<boolean | null>(null);

    useEffect(() => {
        invoke<boolean>('get_wake_word_enabled')
            .then(setEnabled)
            .catch((err) => {
                console.error('Failed to load wake word enabled:', err);
                setEnabled(false);
            });
    }, []);

    useEffect(() => {
        const unlisten = listen<boolean>('wake-word-enabled-changed', (event) => {
            setEnabled(event.payload);
        });
        return () => {
            unlisten.then((u) => u()).catch(() => {});
        };
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
