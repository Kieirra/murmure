import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export const useAutoEnter = () => {
    const [autoEnter, setAutoEnter] = useState(false);

    useEffect(() => {
        invoke<boolean>('get_auto_enter_after_wake_word')
            .then(setAutoEnter)
            .catch((err) => console.error('Failed to load auto enter setting:', err));
    }, []);

    const updateAutoEnter = async (value: boolean) => {
        try {
            await invoke('set_auto_enter_after_wake_word', {
                enabled: value,
            });
            setAutoEnter(value);
        } catch (err) {
            console.error('Failed to set auto enter:', err);
        }
    };

    return { autoEnter, setAutoEnter: updateAutoEnter };
};
