import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export type RecordingMode = 'standard' | 'llm' | 'command';

export const useRecordingMode = () => {
    const [recordingMode, setRecordingMode] = useState<RecordingMode>('standard');

    useEffect(() => {
        invoke<string>('get_recording_mode').then((mode) => {
            if (mode === 'llm' || mode === 'command' || mode === 'standard') setRecordingMode(mode);
        });
    }, []);

    return recordingMode;
};
