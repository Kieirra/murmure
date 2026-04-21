import { useCallback, useEffect, useRef, useState } from 'react';
import { useChronologicalTranscriptions } from '../hooks/use-chronological-transcriptions';
import { formatTimestamp } from '../helpers/format-timestamp';
import { t } from '../i18n';
import type { TranscriptionEntry } from '../smartmic.types';

interface TranscriptionModeProps {
    transcriptions: TranscriptionEntry[];
    onClearHistory: () => void;
}

export const TranscriptionMode = ({ transcriptions, onClearHistory }: TranscriptionModeProps) => {
    const { chronological, hasTranscriptions, bottomRef } = useChronologicalTranscriptions(transcriptions);
    const [copiedIndex, setCopiedIndex] = useState<number | null>(null);
    const [copiedAll, setCopiedAll] = useState(false);
    const [menuOpen, setMenuOpen] = useState(false);
    const menuRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (!menuOpen) return;
        const handleClickOutside = (event: MouseEvent | TouchEvent) => {
            if (menuRef.current !== null && !menuRef.current.contains(event.target as Node)) {
                setMenuOpen(false);
            }
        };
        document.addEventListener('mousedown', handleClickOutside);
        document.addEventListener('touchstart', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
            document.removeEventListener('touchstart', handleClickOutside);
        };
    }, [menuOpen]);

    const copyText = useCallback((text: string, index: number | null) => {
        if (navigator.clipboard?.writeText === undefined) return;
        navigator.clipboard.writeText(text).then(() => {
            if (index === null) {
                setCopiedAll(true);
                setTimeout(() => setCopiedAll(false), 2000);
            } else {
                setCopiedIndex(index);
                setTimeout(() => setCopiedIndex(null), 2000);
            }
        }).catch(() => {
            // Clipboard not available
        });
    }, []);

    const handleCopyAll = useCallback(() => {
        if (chronological.length === 0) return;
        copyText(chronological.map((entry) => entry.text).join('\n'), null);
    }, [chronological, copyText]);

    const handleClear = useCallback(() => {
        if (globalThis.confirm(t('transcription.clearConfirm'))) {
            onClearHistory();
        }
        setMenuOpen(false);
    }, [onClearHistory]);

    return (
        <div className="flex-1 flex flex-col min-h-0 relative">
            <div className="absolute top-1 right-1 z-20" ref={menuRef}>
                <button
                    type="button"
                    aria-label={t('status.menu.options')}
                    aria-haspopup="menu"
                    aria-expanded={menuOpen}
                    className="h-8 w-8 flex items-center justify-center text-[#888] active:text-[#e5e5e5] rounded-md"
                    onClick={() => setMenuOpen((prev) => !prev)}
                    disabled={!hasTranscriptions}
                >
                    &#8942;
                </button>
                {menuOpen && (
                    <div
                        role="menu"
                        className="absolute top-full right-0 mt-1 bg-[#111] border border-[#333] rounded-md min-w-[160px] py-1 shadow-lg"
                    >
                        <button
                            type="button"
                            role="menuitem"
                            className="block w-full text-left px-3 py-2 text-sm text-[#e5e5e5] active:bg-[#222]"
                            onClick={handleClear}
                        >
                            {t('transcription.clear')}
                        </button>
                    </div>
                )}
            </div>
            <div className="flex-1 overflow-y-auto p-4 pt-10">
                {hasTranscriptions ? (
                    chronological.map((entry, i) => (
                        <div key={`${entry.timestamp}-${i}`}>
                            <button
                                type="button"
                                className={`block w-full text-left text-sm p-2 rounded-md transition-colors ${
                                    copiedIndex === i
                                        ? 'bg-sky-400/20 text-sky-400'
                                        : 'text-[#e5e5e5] active:bg-[#222]'
                                }`}
                                onClick={() => copyText(entry.text, i)}
                            >
                                {copiedIndex === i ? (
                                    t('transcription.copied')
                                ) : (
                                    <>
                                        <span className="text-[#666] text-[10px] mr-2 tabular-nums">
                                            {formatTimestamp(entry.timestamp)}
                                        </span>
                                        {entry.text}
                                    </>
                                )}
                            </button>
                            {i < chronological.length - 1 && (
                                <div className="border-b border-[#222] my-2" />
                            )}
                        </div>
                    ))
                ) : (
                    <div className="h-full flex items-center justify-center">
                        <p className="text-sm text-[#888]">{t('transcription.empty')}</p>
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
                {copiedAll ? t('transcription.copied') : t('transcription.copyAll')}
            </button>
        </div>
    );
};
