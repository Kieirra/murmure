import { listen } from '@tauri-apps/api/event';
import { useState, useEffect, useRef, useCallback } from 'react';

export const useLevelState = () => {
    const [level, setLevel] = useState(0);
    const [isRecording, setIsRecording] = useState(false);
    const lastNonZeroTime = useRef<number>(0);

    useEffect(() => {
        const unlistenPromise = listen<number>('mic-level', (e) => {
            const value = Math.max(0, Math.min(1, Number(e.payload ?? 0)));
            setLevel(value);

            // Track recording state based on mic-level events
            if (value > 0) {
                lastNonZeroTime.current = Date.now();
                setIsRecording(true);
            } else {
                // When we receive 0, recording has stopped
                setIsRecording(false);
            }
        });
        return () => {
            unlistenPromise.then((un) => un());
        };
    }, []);

    // Reset function that can be called externally
    const reset = useCallback(() => {
        setLevel(0);
        setIsRecording(false);
    }, []);

    return { level, isRecording, reset };
};
