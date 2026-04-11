import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useRef, useState } from 'react';

interface HighlightRange {
    start: number;
    end: number;
}

interface StreamingTranscript {
    text: string;
    highlights: HighlightRange[];
}

export const useStreamingState = () => {
    const [text, setText] = useState('');
    const [highlights, setHighlights] = useState<HighlightRange[]>([]);

    const reset = useCallback(() => {
        setText('');
        setHighlights([]);
    }, []);

    const resetRef = useRef(reset);
    resetRef.current = reset;

    useEffect(() => {
        const unlistenTranscript = listen<StreamingTranscript>('streaming-transcript', (event) => {
            setText(event.payload.text);
            setHighlights(event.payload.highlights);
        });

        const unlistenShow = listen('show-overlay', () => {
            resetRef.current();
        });

        return () => {
            unlistenTranscript.then((unlisten) => unlisten());
            unlistenShow.then((unlisten) => unlisten());
        };
    }, []);

    return {
        text,
        highlights,
        hasStreamingText: text.length > 0,
        reset,
    };
};
