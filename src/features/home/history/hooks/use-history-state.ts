import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';

export interface HistoryEntry {
    id: number;
    timestamp: number;
    text: string;
}

export const useHistoryState = () => {
    const [history, setHistory] = useState<HistoryEntry[]>([]);

    useEffect(() => {
        const loadHistory = async () => {
            try {
                const entries = await invoke<HistoryEntry[]>('get_recent_transcriptions');
                setHistory(entries);
            } catch (e) {
                console.error('Failed to load history:', e);
            }
        };

        loadHistory();

        const unlistenPromise = listen('history-updated', () => {
            loadHistory();
        });

        return () => {
            unlistenPromise.then((unlisten) => unlisten());
        };
    }, []);

    const deleteItem = async (id: number) => {
        try {
            await invoke('delete_history_item', { id });
        } catch (e) {
            console.error('Failed to delete history item:', e);
        }
    };

    return {
        history,
        deleteItem,
    };
};
