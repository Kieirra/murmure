import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export const useOverlayError = () => {
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const unlistenError = listen<string>('llm-error', (event) => {
            setError(event.payload);
        });
        const unlistenRecordingError = listen<string>('recording-error', () => {
            setError('Mic error');
        });
        return () => {
            unlistenError.then((u) => u());
            unlistenRecordingError.then((u) => u());
        };
    }, []);

    useEffect(() => {
        if (error) {
            const timer = setTimeout(() => setError(null), 2000);
            return () => clearTimeout(timer);
        }
    }, [error]);

    return error;
};
