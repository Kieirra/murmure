import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export const useHistoryLimitState = () => {
    const [historyLimit, setHistoryLimit] = useState<number>(5);

    useEffect(() => {
        invoke<number>('get_history_limit').then((limit) => {
            setHistoryLimit(limit);
        });
    }, []);

    const handleSetHistoryLimit = async (limit: number) => {
        try {
            setHistoryLimit(limit);
            await invoke('set_history_limit', { limit });
        } catch {
            // Revert on error, though we don't know the exact previous state easily without ref, 
            // but setting it to 5 is a safe fallback or just re-fetching
            invoke<number>('get_history_limit').then(setHistoryLimit);
        }
    };

    return {
        historyLimit,
        setHistoryLimit: handleSetHistoryLimit,
    };
};
