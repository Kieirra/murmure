import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

interface ChunkCommitted {
    text: string;
    isFinal: boolean;
}

const joinCommitted = (previous: string, addition: string) => {
    const trimmed = addition.trim();
    if (trimmed.length === 0) return previous;
    if (previous.length === 0) return trimmed;
    return `${previous} ${trimmed}`;
};

export const useCommittedText = () => {
    const [text, setText] = useState('');
    const [isDone, setIsDone] = useState(false);

    useEffect(() => {
        const unlistenCommitted = listen<ChunkCommitted>('transcription-chunk-committed', (event) => {
            setText((previous) => joinCommitted(previous, event.payload.text));
            if (event.payload.isFinal) setIsDone(true);
        });
        const unlistenReset = listen('streaming-transcript', () => {
            setText('');
            setIsDone(false);
        });
        return () => {
            unlistenCommitted.then((u) => u());
            unlistenReset.then((u) => u());
        };
    }, []);

    return { text, isDone, hasCommittedText: text.length > 0 };
};
