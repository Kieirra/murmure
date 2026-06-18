import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export enum OverlayErrorKind {
    Fatal = 'fatal',
    Chunk = 'chunk',
}

export interface OverlayError {
    kind: OverlayErrorKind;
    message: string;
}

const FATAL_DISMISS_MS = 3000;
const CHUNK_DISMISS_MS = 4000;

export const useOverlayError = () => {
    const [error, setError] = useState<OverlayError | null>(null);

    useEffect(() => {
        const unlistenLlmError = listen<string>('llm-error', (event) => {
            setError({ kind: OverlayErrorKind.Fatal, message: event.payload });
        });
        const unlistenRecordingError = listen<string>('recording-error', () => {
            setError({ kind: OverlayErrorKind.Fatal, message: 'Mic error' });
        });
        const unlistenChunkError = listen<string>('transcription-chunk-error', () => {
            setError({ kind: OverlayErrorKind.Chunk, message: "One part wasn't transcribed" });
        });
        return () => {
            unlistenLlmError.then((u) => u());
            unlistenRecordingError.then((u) => u());
            unlistenChunkError.then((u) => u());
        };
    }, []);

    useEffect(() => {
        if (error == null) return;
        const dismissMs = error.kind === OverlayErrorKind.Fatal ? FATAL_DISMISS_MS : CHUNK_DISMISS_MS;
        const timer = setTimeout(() => setError(null), dismissMs);
        return () => clearTimeout(timer);
    }, [error]);

    return error;
};
