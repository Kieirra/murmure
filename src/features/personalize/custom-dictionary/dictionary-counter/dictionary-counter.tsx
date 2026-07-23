import clsx from 'clsx';
import { AlertTriangle } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { DictionaryZone, getDictionaryZone } from './dictionary-counter.helpers';

interface DictionaryCounterProps {
    wordCount: number;
}

export const DictionaryCounter = ({ wordCount }: DictionaryCounterProps) => {
    const { t } = useTranslation();

    if (wordCount === 0) return null;

    const zone = getDictionaryZone(wordCount);
    const label = wordCount === 1 ? t('1 word') : t('{{count}} words', { count: wordCount });

    return (
        <>
            <p
                className={clsx(
                    'text-xs',
                    zone === DictionaryZone.Optimal && 'text-emerald-400',
                    zone === DictionaryZone.Reduced && 'text-yellow-300/90',
                    zone === DictionaryZone.Diluted && 'text-red-400'
                )}
                data-testid="dictionary-word-counter"
            >
                {label}
            </p>
            {zone === DictionaryZone.Reduced && (
                <div
                    className="flex items-start gap-1.5 text-xs text-yellow-300/90"
                    data-testid="dictionary-counter-warning"
                >
                    <AlertTriangle className="w-3 h-3 shrink-0 mt-0.5" />
                    <span>
                        {t(
                            'Boost strength decreases as the list grows. For best results, keep only the words you really need.'
                        )}
                    </span>
                </div>
            )}
            {zone === DictionaryZone.Diluted && (
                <div
                    className="flex items-start gap-1.5 text-xs text-yellow-300/90"
                    data-testid="dictionary-counter-warning"
                >
                    <AlertTriangle className="w-3 h-3 shrink-0 mt-0.5" />
                    <span>
                        {t(
                            'Above 100 words, each word gets a much weaker boost. Trim the list to the terms that matter.'
                        )}
                    </span>
                </div>
            )}
        </>
    );
};
