import type { TranslationEntry } from '../smartmic.types';
import { formatTimestamp } from '../helpers/format-timestamp';
import { t } from '../i18n';

interface ChatBubbleProps {
    entry: TranslationEntry;
    onLeft: boolean;
}

export const ChatBubble = ({ entry, onLeft }: ChatBubbleProps) => {
    const detectedLabel = entry.detectedLang === null ? '??' : entry.detectedLang.toUpperCase();
    const targetLabel = entry.targetLang.toUpperCase();
    const hasTranslation = entry.translatedText.length > 0;
    const bubbleBg = onLeft ? 'bg-[#1a2e3a]' : 'bg-[#2e1a1a]';
    const sideAlign = onLeft ? 'text-left' : 'text-right';

    return (
        <div
            className={`flex ${onLeft ? 'justify-start' : 'justify-end'} mb-2 animate-in slide-in-from-bottom-2 fade-in-0 duration-200`}
        >
            <div className="flex flex-col max-w-[85%] gap-0.5">
                <div className={`rounded-2xl px-3 py-2 ${bubbleBg} text-[#e5e5e5]`}>
                    <div className="flex items-start gap-2">
                        <span className="text-[10px] font-semibold uppercase text-[#888] pt-0.5 w-5 shrink-0">
                            {detectedLabel}
                        </span>
                        <span className="text-sm font-bold">{entry.text}</span>
                    </div>
                    {hasTranslation && (
                        <div className="flex items-start gap-2 mt-1">
                            <span className="text-[10px] font-semibold uppercase text-[#888] pt-0.5 w-5 shrink-0">
                                {targetLabel}
                            </span>
                            <span className="text-sm">{entry.translatedText}</span>
                        </div>
                    )}
                </div>
                {!hasTranslation && (
                    <span className={`text-[#666] text-[10px] ${sideAlign}`}>
                        {t('translation.unavailable')}
                    </span>
                )}
                <span className={`text-[10px] text-[#666] tabular-nums ${sideAlign}`}>
                    {formatTimestamp(entry.timestamp)}
                </span>
            </div>
        </div>
    );
};
