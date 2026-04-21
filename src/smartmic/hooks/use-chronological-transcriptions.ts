import { useEffect, useRef } from 'react';
import type { TranscriptionEntry } from '../smartmic.types';

interface UseChronologicalTranscriptionsResult {
    chronological: TranscriptionEntry[];
    hasTranscriptions: boolean;
    bottomRef: React.RefObject<HTMLDivElement | null>;
}

export const useChronologicalTranscriptions = (
    transcriptions: TranscriptionEntry[]
): UseChronologicalTranscriptionsResult => {
    const bottomRef = useRef<HTMLDivElement>(null);
    const chronological = [...transcriptions].reverse();
    const hasTranscriptions = chronological.length > 0;

    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [transcriptions]);

    return { chronological, hasTranscriptions, bottomRef };
};
