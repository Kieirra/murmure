import { BookText } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { EXAMPLE_WORDS } from './dictionary-empty-state.helpers';

interface DictionaryEmptyStateProps {
    onAdd: (word: string) => void;
}

export const DictionaryEmptyState = ({ onAdd }: DictionaryEmptyStateProps) => {
    const { t } = useTranslation();

    return (
        <div className="flex flex-col items-center gap-4 py-10 text-center" data-testid="custom-dictionary-empty-state">
            <BookText className="w-8 h-8 text-muted-foreground" />
            <p className="text-sm text-muted-foreground">{t('No words yet')}</p>
            <div className="space-y-2">
                <p className="text-sm text-muted-foreground">{t('Try adding one of these:')}</p>
                <div className="flex flex-wrap justify-center gap-2">
                    {EXAMPLE_WORDS.map((word) => (
                        <button
                            key={word}
                            type="button"
                            onClick={() => onAdd(word)}
                            className="inline-flex items-center px-3 py-1.5 text-xs bg-card hover:bg-accent text-foreground rounded-md border border-border transition-colors cursor-pointer"
                            data-testid={`custom-dictionary-example-${word}`}
                        >
                            {word}
                        </button>
                    ))}
                </div>
            </div>
        </div>
    );
};
