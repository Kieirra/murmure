import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export const useHistoryPersistenceState = () => {
    const [persistHistory, setPersistHistory] = useState<boolean>(true);

    useEffect(() => {
        invoke<boolean>('get_persist_history').then((enabled) => {
            setPersistHistory(enabled);
        });
    }, []);

    const handleSetPersistHistory = async (enabled: boolean) => {
        try {
            setPersistHistory(enabled);
            await invoke('set_persist_history', { enabled });
        } catch {
            setPersistHistory(!enabled);
        }
    };

    return {
        persistHistory,
        setPersistHistory: handleSetPersistHistory,
    };
};
