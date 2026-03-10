import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { subItemKey, DICTIONARY_PREVIEW_LIMIT } from '../constants';

interface DictionarySubItemsProps {
    words: string[];
    selection: Record<string, boolean>;
    onToggle: (key: string, checked: boolean) => void;
    disabled?: boolean;
    showAll: boolean;
    onShowAll: () => void;
}

export const DictionarySubItems = ({
    words,
    selection,
    onToggle,
    disabled,
    showAll,
    onShowAll,
}: DictionarySubItemsProps) => {
    const { t } = useTranslation();

    const wordsToShow = showAll
        ? words
        : words.slice(0, DICTIONARY_PREVIEW_LIMIT);
    const hiddenCount = words.length - DICTIONARY_PREVIEW_LIMIT;

    return (
        <div className="flex flex-wrap gap-2 py-1">
            {wordsToShow.map((word) => {
                const key = subItemKey.word(word);
                const isSelected = selection[key] ?? true;
                return (
                    <button
                        key={word}
                        type="button"
                        onClick={() =>
                            onToggle(key, !isSelected)
                        }
                        disabled={disabled}
                        className={clsx(
                            'inline-flex items-center px-3 py-1.5 text-xs rounded-md border transition-colors',
                            isSelected
                                ? 'bg-primary/10 border-primary text-foreground'
                                : 'bg-card border-border opacity-50',
                            disabled ? 'cursor-not-allowed' : 'cursor-pointer'
                        )}
                    >
                        {word}
                    </button>
                );
            })}
            {!showAll && hiddenCount > 0 && (
                <button
                    type="button"
                    onClick={onShowAll}
                    className="inline-flex items-center px-3 py-1.5 text-xs rounded-md border border-border text-muted-foreground hover:bg-accent transition-colors cursor-pointer"
                >
                    +{hiddenCount} {t('more...')}
                </button>
            )}
        </div>
    );
};
