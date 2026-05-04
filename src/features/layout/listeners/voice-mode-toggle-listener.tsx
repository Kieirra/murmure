import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

export const VoiceModeToggleListener = () => {
    useEffect(() => {
        const unlisten = listen('voice-mode-toggle-requested', async () => {
            try {
                const everEnabled = await invoke<boolean>('get_voice_mode_ever_enabled');
                if (!everEnabled) return;

                const currentEnabled = await invoke<boolean>('get_wake_word_enabled');
                const newEnabled = !currentEnabled;

                await invoke('set_wake_word_enabled', { enabled: newEnabled });
                await invoke('flash_mode_overlay', {
                    text: newEnabled ? 'VOICE' : 'MUTED',
                });
            } catch (err) {
                console.error('Voice mode toggle failed:', err);
            }
        });
        return () => {
            unlisten.then((u) => u()).catch(() => {});
        };
    }, []);

    return null;
};
