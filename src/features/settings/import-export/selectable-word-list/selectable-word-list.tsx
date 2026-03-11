import { useTranslation } from '@/i18n';
import { WordTag } from '@/components/word-tag';
import { subItemKey, DICTIONARY_PREVIEW_LIMIT } from '../import-export.constants';

interface SelectableWordListProps {
    words: string[];
    selection: Record<string, boolean>;
    onToggle: (key: string, checked: boolean) => void;
    disabled?: boolean;
    showAll: boolean;
    onShowAll: () => void;
}

export const SelectableWordList = ({
    words,
    selection,
    onToggle,
    disabled,
    showAll,
    onShowAll,
}: SelectableWordListProps) => {
    const { t } = useTranslation();

    const wordsToShow = showAll ? words : words.slice(0, DICTIONARY_PREVIEW_LIMIT);
    const hiddenCount = words.length - DICTIONARY_PREVIEW_LIMIT;

    return (
        <div className="flex flex-wrap gap-2 py-1">
            {wordsToShow.map((word) => {
                const key = subItemKey.word(word);
                const isSelected = selection[key] ?? true;
                return (
                    <WordTag
                        key={word}
                        word={word}
                        variant="selectable"
                        selected={isSelected}
                        onClick={() => onToggle(key, !isSelected)}
                        disabled={disabled}
                    />
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
