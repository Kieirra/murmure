import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export const useSilenceTimeout = () => {
    const [silenceTimeoutMs, setSilenceTimeoutMs] = useState(1500);

    useEffect(() => {
        invoke<number>('get_silence_timeout_ms')
            .then(setSilenceTimeoutMs)
            .catch((err) => console.error('Failed to load silence timeout:', err));
    }, []);

    const updateSilenceTimeout = async (value: number) => {
        try {
            await invoke('set_silence_timeout_ms', { value });
            setSilenceTimeoutMs(value);
        } catch (err) {
            console.error('Failed to set silence timeout:', err);
        }
    };

    return { silenceTimeoutMs, setSilenceTimeoutMs: updateSilenceTimeout };
};
