import { useEffect, useRef } from 'react';

interface UseChronologicalTranscriptionsResult {
    chronological: string[];
    hasTranscriptions: boolean;
    bottomRef: React.RefObject<HTMLDivElement | null>;
}

export const useChronologicalTranscriptions = (
    transcriptions: string[]
): UseChronologicalTranscriptionsResult => {
    const bottomRef = useRef<HTMLDivElement>(null);
    const chronological = [...transcriptions].reverse();
    const hasTranscriptions = chronological.length > 0;

    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [transcriptions]);

    return { chronological, hasTranscriptions, bottomRef };
};
