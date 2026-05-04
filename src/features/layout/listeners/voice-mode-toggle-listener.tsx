import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

export const VoiceModeToggleListener = () => {
    useEffect(() => {
        const unlisten = listen('voice-mode-toggle-requested', async () => {
            try {
                const current = await invoke<boolean>('get_wake_word_enabled');
                const next = !current;
                await invoke('set_wake_word_enabled', { enabled: next });
                await invoke('flash_text_in_overlay', { text: next ? 'VOICE' : 'MUTED' });
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
