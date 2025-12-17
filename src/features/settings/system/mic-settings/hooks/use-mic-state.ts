import { useCallback, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface MicDevice {
    id: string;
    label: string;
}

const DEFAULT_MIC_ID = 'default';
const DEFAULT_MIC_LABEL = 'Default';

export function useMicState() {
    const [micList, setMicList] = useState<MicDevice[]>([]);
    const [currentMic, setCurrentMic] = useState<string>(DEFAULT_MIC_ID);

    useEffect(() => {
        const loadMicState = async () => {
            try {
                const devices = await invoke<string[]>('get_mic_list');
                const mapped = devices.map((label) => ({ id: label, label }));
                setMicList([
                    { id: DEFAULT_MIC_ID, label: DEFAULT_MIC_LABEL },
                    ...mapped,
                ]);
            } catch {
                setMicList([{ id: DEFAULT_MIC_ID, label: DEFAULT_MIC_LABEL }]);
            }

            try {
                const id = await invoke<string | null>('get_current_mic_id');
                setCurrentMic(id ?? DEFAULT_MIC_ID);
            } catch {
                setCurrentMic(DEFAULT_MIC_ID);
            }
        };

        loadMicState();
    }, []);

    const setMic = useCallback((id: string) => {
        setCurrentMic(id);
        // If 'default' is selected, send None (null) to backend
        invoke('set_current_mic_id', { micId: id === DEFAULT_MIC_ID ? null : id });
    }, []);

    return { micList, currentMic, setMic };
}
