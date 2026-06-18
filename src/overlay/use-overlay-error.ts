import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

const DISMISS_MS = 4000;

export const useOverlayError = () => {
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const unlistenLlmError = listen<string>('llm-error', (event) => {
            setError(event.payload);
        });
        const unlistenRecordingError = listen<string>('recording-error', () => {
            setError('Mic error');
        });
        const unlistenChunkError = listen<string>('transcription-chunk-error', () => {
            setError('Transcription partial');
        });
        return () => {
            unlistenLlmError.then((u) => u());
            unlistenRecordingError.then((u) => u());
            unlistenChunkError.then((u) => u());
        };
    }, []);

    useEffect(() => {
        if (error == null) return;
        const timer = setTimeout(() => setError(null), DISMISS_MS);
        return () => clearTimeout(timer);
    }, [error]);

    return error;
};
