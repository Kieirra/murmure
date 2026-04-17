import { useEffect, useRef } from 'react';
import type { TranslationEntry, TranslationSide } from '../types';
import { LANGUAGES } from '../constants/languages';
import { usePersistedLang } from '../hooks/use-persisted-lang';

interface TranslationModeProps {
    isRecording: boolean;
    recordingSide: TranslationSide | null;
    micLevel: number;
    translationEntries: TranslationEntry[];
    onToggleRec: (side: TranslationSide, sourceLang: string, targetLang: string) => void;
}

interface TranslationHalfProps {
    side: TranslationSide;
    entries: TranslationEntry[];
    lang: string;
    onLangChange: (code: string) => void;
    isRecording: boolean;
    recordingSide: TranslationSide | null;
    micLevel: number;
    otherLang: string;
    onToggleRec: (side: TranslationSide, sourceLang: string, targetLang: string) => void;
}

const TranslationHalf = ({
    side,
    entries,
    lang,
    onLangChange,
    isRecording,
    recordingSide,
    micLevel,
    otherLang,
    onToggleRec,
}: TranslationHalfProps) => {
    const bottomRef = useRef<HTMLDivElement>(null);
    const recBtnRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [entries.length]);

    useEffect(() => {
        const el = recBtnRef.current;
        if (!el) return;

        const handleTouch = (e: TouchEvent) => {
            e.preventDefault();
            onToggleRec(side, lang, otherLang);
        };

        el.addEventListener('touchstart', handleTouch, { passive: false });
        return () => {
            el.removeEventListener('touchstart', handleTouch);
        };
    }, [onToggleRec, side, lang, otherLang]);

    const isThisSideRecording = isRecording && recordingSide === side;
    const isOtherSideRecording = isRecording && recordingSide !== side;

    return (
        <div className="flex-1 flex flex-col min-h-0">
            <div className="flex-1 overflow-y-auto min-h-0 px-3 py-2">
                {entries.length > 0 ? (
                    entries.map((entry, i) => (
                        <p key={`${i}-${entry.text.slice(0, 20)}`} className="text-sm text-[#e5e5e5] mb-1.5">
                            {entry.text}
                        </p>
                    ))
                ) : (
                    <div className="h-full flex items-center justify-center">
                        <p className="text-xs text-[#888]">Waiting for translation...</p>
                    </div>
                )}
                <div ref={bottomRef} />
            </div>

            <div className="shrink-0 h-7 flex items-center justify-center">
                <select
                    className="bg-transparent text-sky-400 text-xs text-center border-none outline-none appearance-none cursor-pointer"
                    value={lang}
                    disabled={isRecording}
                    onChange={(e) => onLangChange(e.target.value)}
                >
                    {LANGUAGES.map((l) => (
                        <option key={l.code} value={l.code} className="bg-[#0a0a0a] text-[#e5e5e5]">
                            {l.name}
                        </option>
                    ))}
                </select>
            </div>

            <div
                ref={recBtnRef}
                className={`shrink-0 h-16 flex items-center justify-center gap-2 relative cursor-pointer transition-all duration-150 ${
                    isThisSideRecording
                        ? 'bg-[#7f1d1d]'
                        : 'bg-[#1a1a2e]'
                } ${isOtherSideRecording ? 'opacity-40 pointer-events-none' : ''}`}
                style={{ touchAction: 'none' }}
            >
                <span className={`text-base font-bold ${isThisSideRecording ? 'text-[#fca5a5]' : 'text-[#e5e5e5]'}`}>
                    &#9679; REC
                </span>
                <div
                    className="absolute bottom-0 left-0 h-[3px] bg-[#dc2626] transition-[width] duration-100"
                    style={{ width: isThisSideRecording ? `${Math.round(micLevel * 100)}%` : '0%' }}
                />
            </div>
        </div>
    );
};

export const TranslationMode = ({
    isRecording,
    recordingSide,
    micLevel,
    translationEntries,
    onToggleRec,
}: TranslationModeProps) => {
    const [bottomLang, setBottomLang] = usePersistedLang('smartmic_translation_bottom_lang', 'fr');
    const [topLang, setTopLang] = usePersistedLang('smartmic_translation_top_lang', 'en');

    const topEntries = translationEntries.filter((e) => e.fromSide === 'bottom');
    const bottomEntries = translationEntries.filter((e) => e.fromSide === 'top');

    return (
        <div className="flex-1 flex flex-col min-h-0">
            <div className="flex-1 flex flex-col rotate-180 border-b-2 border-[#444] min-h-0">
                <TranslationHalf
                    side="top"
                    entries={topEntries}
                    lang={topLang}
                    onLangChange={setTopLang}
                    isRecording={isRecording}
                    recordingSide={recordingSide}
                    micLevel={micLevel}
                    otherLang={bottomLang}
                    onToggleRec={onToggleRec}
                />
            </div>
            <div className="flex-1 flex flex-col min-h-0">
                <TranslationHalf
                    side="bottom"
                    entries={bottomEntries}
                    lang={bottomLang}
                    onLangChange={setBottomLang}
                    isRecording={isRecording}
                    recordingSide={recordingSide}
                    micLevel={micLevel}
                    otherLang={topLang}
                    onToggleRec={onToggleRec}
                />
            </div>
        </div>
    );
};
