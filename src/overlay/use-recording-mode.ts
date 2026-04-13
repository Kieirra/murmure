import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export type RecordingMode = 'standard' | 'llm' | 'command';

export const useRecordingMode = () => {
    const [recordingMode, setRecordingMode] = useState<RecordingMode>('standard');

    useEffect(() => {
        invoke<string>('get_recording_mode')
            .then((mode) => {
                if (mode === 'llm' || mode === 'command' || mode === 'standard') setRecordingMode(mode);
            })
            .catch(() => {});
    }, []);

    useEffect(() => {
        const unlisten = listen<string>('recording-mode', (event) => {
            const mode = event.payload;
            if (mode === 'llm' || mode === 'command' || mode === 'standard') setRecordingMode(mode);
        });
        return () => {
            unlisten.then((u) => u());
        };
    }, []);

    return recordingMode;
};
