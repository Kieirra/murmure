import { WordTag } from '@/components/word-tag';
import { GROUPING_THRESHOLD, groupByLetter } from './dictionary-word-list.helpers';

interface DictionaryWordListProps {
    words: string[];
    onRemove: (word: string) => void;
}

export const DictionaryWordList = ({ words, onRemove }: DictionaryWordListProps) => {
    if (words.length > GROUPING_THRESHOLD) {
        return (
            <div className="space-y-2">
                {groupByLetter(words).map((group) => (
                    <div key={group.letter} className="flex items-start gap-3">
                        <div className="w-4 shrink-0 pt-1 text-sm font-semibold text-muted-foreground">
                            {group.letter}
                        </div>
                        <div className="flex flex-1 flex-wrap gap-1.5">
                            {group.words.map((word) => (
                                <WordTag
                                    key={word}
                                    word={word}
                                    variant="removable"
                                    onClick={() => onRemove(word)}
                                    data-testid={`custom-dictionary-remove-button-${word}`}
                                />
                            ))}
                        </div>
                    </div>
                ))}
            </div>
        );
    }

    return (
        <div className="flex flex-wrap gap-1.5">
            {words.map((word) => (
                <WordTag
                    key={word}
                    word={word}
                    variant="removable"
                    onClick={() => onRemove(word)}
                    data-testid={`custom-dictionary-remove-button-${word}`}
                />
            ))}
        </div>
    );
};
