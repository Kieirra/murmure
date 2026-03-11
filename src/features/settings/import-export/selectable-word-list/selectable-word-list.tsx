import { useState } from 'react';
import { useTranslation } from '@/i18n';
import { WordTag } from '@/components/word-tag';
import { SUB_ITEM_KEY, DICTIONARY_PREVIEW_LIMIT } from '../import-export.constants';

interface SelectableWordListProps {
    words: string[];
    selection: Record<string, boolean>;
    onToggle: (key: string, checked: boolean) => void;
    disabled?: boolean;
}

export const SelectableWordList = ({ words, selection, onToggle, disabled }: SelectableWordListProps) => {
    const [showAll, setShowAll] = useState(false);
    const { t } = useTranslation();

    const wordsToShow = showAll ? words : words.slice(0, DICTIONARY_PREVIEW_LIMIT);
    const hiddenCount = words.length - DICTIONARY_PREVIEW_LIMIT;

    return (
        <div className="flex flex-wrap gap-2 py-1">
            {wordsToShow.map((word) => {
                const key = SUB_ITEM_KEY.word(word);
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
                    onClick={() => setShowAll(true)}
                    className="inline-flex items-center px-3 py-1.5 text-xs rounded-md border border-border text-muted-foreground hover:bg-accent transition-colors cursor-pointer"
                >
                    +{hiddenCount} {t('more...')}
                </button>
            )}
        </div>
    );
};
