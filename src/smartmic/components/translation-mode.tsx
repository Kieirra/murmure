import { useEffect, useMemo, useRef, useState } from 'react';
import type { TranslationEntry } from '../types';
import { usePersistedLangPair } from '../hooks/use-persisted-lang-pair';
import { useI18n } from '../i18n/use-i18n';
import { LanguagePickerSheet } from './language-picker-sheet';
import { ChatBubble } from './chat-bubble/chat-bubble';
import { isOnLeft, nameForCode } from './translation-mode.helpers';

interface TranslationModeProps {
    isRecording: boolean;
    isTranslating: boolean;
    micLevel: number;
    translationEntries: TranslationEntry[];
    onToggleRec: (langA: string, langB: string) => void;
}

export const TranslationMode = ({
    isRecording,
    isTranslating,
    micLevel,
    translationEntries,
    onToggleRec,
}: TranslationModeProps) => {
    const { t } = useI18n();
    const [pair, setPair] = usePersistedLangPair({ a: 'en', b: 'fr' });
    const [pickerTarget, setPickerTarget] = useState<'a' | 'b' | null>(null);
    const bottomRef = useRef<HTMLDivElement>(null);
    const recBtnRef = useRef<HTMLDivElement>(null);

    const recLabel = useMemo(
        () => `${pair.a.toUpperCase()} \u21C4 ${pair.b.toUpperCase()} \u00B7 ${t('translation.recHint')}`,
        [pair.a, pair.b, t]
    );

    // Auto-scroll to bottom on new entry or when translating indicator appears.
    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [translationEntries.length, isTranslating]);

    // REC button touch handler.
    useEffect(() => {
        const el = recBtnRef.current;
        if (el === null) return;
        const handleTouch = (e: TouchEvent) => {
            e.preventDefault();
            onToggleRec(pair.a, pair.b);
        };
        el.addEventListener('touchstart', handleTouch, { passive: false });
        return () => {
            el.removeEventListener('touchstart', handleTouch);
        };
    }, [onToggleRec, pair.a, pair.b]);

    const handleLangSelect = (code: string) => {
        if (pickerTarget === null) return;
        // Prevent both slots from sharing the same language: swap if collision.
        if (pickerTarget === 'a') {
            setPair(code === pair.b ? { a: code, b: pair.a } : { a: code, b: pair.b });
        } else {
            setPair(code === pair.a ? { a: pair.b, b: code } : { a: pair.a, b: code });
        }
        setPickerTarget(null);
    };

    const hasEntries = translationEntries.length > 0 || isTranslating;

    return (
        <div className="flex-1 flex flex-col min-h-0">
            {/* Language selectors header */}
            <div className="shrink-0 h-12 flex items-center gap-2 px-3 border-b border-[#222]">
                <span className="text-xs text-[#888]">{t('translation.languages')}</span>
                <button
                    type="button"
                    disabled={isRecording}
                    onClick={() => setPickerTarget('a')}
                    className={`h-10 px-3 min-w-[44px] rounded-lg border border-[#333] bg-[#111] text-sm text-[#e5e5e5] ${
                        isRecording ? 'opacity-40' : 'active:bg-[#222]'
                    }`}
                >
                    {nameForCode(pair.a)}
                </button>
                <span className="text-xs text-[#555]" aria-hidden="true">
                    &#8644;
                </span>
                <button
                    type="button"
                    disabled={isRecording}
                    onClick={() => setPickerTarget('b')}
                    className={`h-10 px-3 min-w-[44px] rounded-lg border border-[#333] bg-[#111] text-sm text-[#e5e5e5] ${
                        isRecording ? 'opacity-40' : 'active:bg-[#222]'
                    }`}
                >
                    {nameForCode(pair.b)}
                </button>
            </div>

            {/* Bubbles list */}
            <div className="flex-1 overflow-y-auto px-3 py-2" aria-live="polite">
                {hasEntries ? (
                    <>
                        {translationEntries.map((entry, i) => (
                            <ChatBubble
                                key={`${entry.timestamp}-${i}`}
                                entry={entry}
                                onLeft={isOnLeft(entry, pair.b)}
                            />
                        ))}
                        {isTranslating && (
                            <div className="flex justify-start mb-2 animate-in fade-in-0 duration-200">
                                <div className="rounded-2xl px-3 py-2 bg-[#1a2e3a] text-[#e5e5e5] flex items-center gap-2">
                                    <span className="text-sm">{t('translation.translating')}</span>
                                    <span className="animate-pulse text-sm">...</span>
                                </div>
                            </div>
                        )}
                        <div ref={bottomRef} />
                    </>
                ) : (
                    <div className="h-full flex items-center justify-center">
                        <p className="text-sm text-[#888] text-center px-4">{t('translation.empty')}</p>
                    </div>
                )}
            </div>

            {/* REC bar */}
            <div className="shrink-0 h-20 flex items-center px-2">
                <div
                    ref={recBtnRef}
                    className={`mx-2 h-16 w-full rounded-xl flex items-center justify-center gap-2 relative cursor-pointer select-none transition-all duration-150 ${
                        isRecording
                            ? 'bg-[#7f1d1d] border-2 border-[#dc2626] text-[#fca5a5]'
                            : 'bg-[#1a1a2e] border-2 border-[#444] text-[#e5e5e5]'
                    }`}
                    style={{ touchAction: 'none' }}
                >
                    <span className="text-base font-semibold">
                        {isRecording ? '\u25A0' : '\u25CF'} REC {recLabel}
                    </span>
                    <div
                        className="absolute bottom-0 left-0 h-[3px] bg-[#dc2626] rounded-b-[10px] transition-[width] duration-100"
                        style={{ width: isRecording ? `${Math.round(micLevel * 100)}%` : '0%' }}
                    />
                </div>
            </div>

            <LanguagePickerSheet
                open={pickerTarget !== null}
                currentCode={pickerTarget === 'a' ? pair.a : pair.b}
                onSelect={handleLangSelect}
                onClose={() => setPickerTarget(null)}
            />
        </div>
    );
};
