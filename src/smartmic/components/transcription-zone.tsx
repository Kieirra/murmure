import { useCallback, useState } from 'react';
import { useI18n } from '../i18n/use-i18n';
import type { TranscriptionEntry } from '../types';
import { COPY_FEEDBACK_MS, REMOTE_VISIBLE_COUNT } from './transcription-zone.helpers';

interface TranscriptionZoneProps {
    transcriptions: TranscriptionEntry[];
}

export const TranscriptionZone = ({ transcriptions }: TranscriptionZoneProps) => {
    const { t } = useI18n();
    const [copiedIndex, setCopiedIndex] = useState<number | null>(null);

    const copyText = useCallback((text: string, index: number) => {
        if (navigator.clipboard?.writeText === undefined) return;
        navigator.clipboard.writeText(text).then(() => {
            setCopiedIndex(index);
            setTimeout(() => setCopiedIndex(null), COPY_FEEDBACK_MS);
        }).catch(() => {
            // Clipboard not available
        });
    }, []);

    const entries = transcriptions.slice(0, REMOTE_VISIBLE_COUNT);

    return (
        <div className="flex-1 overflow-y-auto border-b border-[#222]">
            {entries.length === 0 ? (
                <div className="h-full flex items-center justify-center px-3">
                    <span className="text-[#555] italic text-sm">{t('remote.empty')}</span>
                </div>
            ) : (
                <div className="p-3 flex flex-col gap-2">
                    {entries.map((entry, i) => (
                        <button
                            key={`${entry.timestamp}-${i}`}
                            type="button"
                            onClick={() => copyText(entry.text, i)}
                            className={`text-left text-sm p-2 rounded-md transition-colors ${
                                copiedIndex === i
                                    ? 'bg-sky-400/20 text-sky-400'
                                    : 'text-[#ccc] active:bg-[#222]'
                            }`}
                        >
                            {copiedIndex === i ? t('remote.copied') : entry.text}
                        </button>
                    ))}
                </div>
            )}
        </div>
    );
};
