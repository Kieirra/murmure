import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'sonner';

export const useKeyboardModeState = () => {
    const [mode, setMode] = useState<'push-to-talk' | 'toggle'>('push-to-talk');

    const loadMode = async () => {
        try {
            const value = await invoke<string>('get_keyboard_mode');
            if (value === 'push-to-talk' || value === 'toggle') {
                setMode(value);
            }
        } catch (error) {
            console.error('Failed to load keyboard mode:', error);
        }
    };

    useEffect(() => {
        loadMode();
    }, []);

    const saveMode = async (value: 'push-to-talk' | 'toggle') => {
        try {
            await invoke('set_keyboard_mode', {
                mode: value,
            });
            setMode(value);
        } catch {
            toast('Failed to save keyboard mode');
        }
    };

    return {
        keyboardMode: mode,
        setKeyboardMode: saveMode,
    };
};
