import { useEffect, useRef } from 'react';
import { LANGUAGES } from '../constants/languages';
import { useI18n } from '../i18n/use-i18n';

interface LanguagePickerSheetProps {
    open: boolean;
    currentCode: string;
    onSelect: (code: string) => void;
    onClose: () => void;
}

export const LanguagePickerSheet = ({ open, currentCode, onSelect, onClose }: LanguagePickerSheetProps) => {
    const { t } = useI18n();
    const firstItemRef = useRef<HTMLButtonElement>(null);

    useEffect(() => {
        if (open) {
            // Basic focus trap: move focus to first option for keyboard users.
            firstItemRef.current?.focus();
        }
    }, [open]);

    useEffect(() => {
        if (!open) return;
        const handler = (e: KeyboardEvent) => {
            if (e.key === 'Escape') onClose();
        };
        document.addEventListener('keydown', handler);
        return () => document.removeEventListener('keydown', handler);
    }, [open, onClose]);

    if (!open) return null;

    return (
        <div
            role="dialog"
            aria-modal="true"
            aria-label={t('translation.chooseLang')}
            className="fixed inset-0 z-40 flex items-end justify-center bg-black/70"
            onClick={onClose}
            onKeyDown={(e) => {
                if (e.key === 'Escape') onClose();
            }}
        >
            <div
                className="w-full max-w-md bg-[#111] border-t border-[#333] rounded-t-2xl max-h-[60vh] flex flex-col animate-in slide-in-from-bottom-2 duration-200"
                onClick={(e) => e.stopPropagation()}
            >
                <div className="flex items-center justify-between px-4 h-12 border-b border-[#222] shrink-0">
                    <span className="text-sm text-[#e5e5e5] font-semibold">
                        {t('translation.chooseLang')}
                    </span>
                    <button
                        type="button"
                        className="h-8 w-8 flex items-center justify-center text-[#888] active:text-[#e5e5e5]"
                        onClick={onClose}
                        aria-label={t('translation.closeLangPicker')}
                    >
                        &#10005;
                    </button>
                </div>
                <div className="flex-1 overflow-y-auto">
                    {LANGUAGES.map((lang, i) => {
                        const isCurrent = lang.code === currentCode;
                        return (
                            <button
                                key={lang.code}
                                ref={i === 0 ? firstItemRef : null}
                                type="button"
                                className={`w-full text-left px-4 h-11 flex items-center justify-between text-sm ${
                                    isCurrent ? 'text-sky-400 bg-[#0f172a]' : 'text-[#e5e5e5] active:bg-[#1a1a1a]'
                                }`}
                                onClick={() => onSelect(lang.code)}
                            >
                                <span>{lang.name}</span>
                                {isCurrent && <span aria-hidden="true">&#10003;</span>}
                            </button>
                        );
                    })}
                </div>
            </div>
        </div>
    );
};
