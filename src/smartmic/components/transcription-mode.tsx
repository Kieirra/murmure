import { useCallback, useState } from 'react';
import { useChronologicalTranscriptions } from '../hooks/use-chronological-transcriptions';

interface TranscriptionModeProps {
    transcriptions: string[];
}

export const TranscriptionMode = ({ transcriptions }: TranscriptionModeProps) => {
    const { chronological, hasTranscriptions, bottomRef } = useChronologicalTranscriptions(transcriptions);
    const [copiedIndex, setCopiedIndex] = useState<number | null>(null);
    const [copiedAll, setCopiedAll] = useState(false);

    const copyText = useCallback((text: string, index: number | null) => {
        if (navigator.clipboard?.writeText) {
            navigator.clipboard.writeText(text).then(() => {
                if (index !== null) {
                    setCopiedIndex(index);
                    setTimeout(() => setCopiedIndex(null), 2000);
                } else {
                    setCopiedAll(true);
                    setTimeout(() => setCopiedAll(false), 2000);
                }
            }).catch(() => {
                // Clipboard not available in this context
            });
        }
    }, []);

    const handleCopyAll = useCallback(() => {
        if (chronological.length === 0) return;
        copyText(chronological.join('\n'), null);
    }, [chronological, copyText]);

    return (
        <>
            <div className="flex-1 overflow-y-auto p-4">
                {hasTranscriptions ? (
                    chronological.map((text, i) => (
                        <div key={`${i}-${text.slice(0, 20)}`}>
                            <button
                                type="button"
                                className={`block w-full text-left text-sm p-2 rounded-md transition-colors ${
                                    copiedIndex === i
                                        ? 'bg-sky-400/20 text-sky-400'
                                        : 'text-[#e5e5e5] active:bg-[#222]'
                                }`}
                                onClick={() => copyText(text, i)}
                            >
                                {copiedIndex === i ? 'Copied!' : text}
                            </button>
                            {i < chronological.length - 1 && (
                                <div className="border-b border-[#222] my-2" />
                            )}
                        </div>
                    ))
                ) : (
                    <div className="h-full flex items-center justify-center">
                        <p className="text-sm text-[#888]">Press REC to start dictating</p>
                    </div>
                )}
                <div ref={bottomRef} />
            </div>
            <button
                type="button"
                className={`shrink-0 h-12 flex items-center justify-center border-t border-[#222] w-full text-sm font-medium ${
                    hasTranscriptions ? 'text-sky-400' : 'text-[#555]'
                }`}
                disabled={!hasTranscriptions}
                onClick={handleCopyAll}
            >
                {copiedAll ? 'Copied!' : 'Copy all'}
            </button>
        </>
    );
};
