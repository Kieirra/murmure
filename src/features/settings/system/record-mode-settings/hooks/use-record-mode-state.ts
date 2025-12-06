import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export type RecordMode = 'push_to_talk' | 'toggle_to_talk';

export const useRecordModeState = () => {
    const [recordMode, setRecordMode] = useState<RecordMode>('push_to_talk');

    useEffect(() => {
        invoke<RecordMode>('get_record_mode').then((mode) => {
            if (['push_to_talk', 'toggle_to_talk'].includes(mode)) {
                setRecordMode(mode);
            }
        });
    }, []);

    return {
        recordMode,
        setRecordMode: (mode: RecordMode) => {
            setRecordMode(mode);
            invoke('set_record_mode', { mode: mode });
        },
    };
};
