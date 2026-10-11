import clsx from 'clsx';
import { X } from 'lucide-react';
import { useTranslation } from '@/i18n';

interface WordTagRemovableProps {
    word: string;
    onClick: () => void;
    variant: 'removable';
    disabled?: boolean;
    'data-testid'?: string;
}

interface WordTagSelectableProps {
    word: string;
    onClick: () => void;
    variant: 'selectable';
    selected: boolean;
    disabled?: boolean;
    'data-testid'?: string;
}

type WordTagProps = WordTagRemovableProps | WordTagSelectableProps;

export const WordTag = (props: WordTagProps) => {
    const { word, onClick, variant, disabled } = props;
    const { t } = useTranslation();

    if (variant === 'removable') {
        return (
            <span className="inline-flex items-center gap-0.5 py-0.5 pl-3 pr-0.5 text-xs bg-card hover:bg-accent text-foreground rounded-md border border-transparent transition-colors">
                <span>{word}</span>
                <button
                    type="button"
                    onClick={onClick}
                    disabled={disabled}
                    aria-label={t('Remove {{word}}', { word })}
                    className="inline-flex size-6 items-center justify-center rounded-sm text-muted-foreground hover:text-red-400 cursor-pointer transition-colors"
                    data-testid={props['data-testid']}
                >
                    <X className="size-3.5" />
                </button>
            </span>
        );
    }

    const isSelected = props.selected;

    return (
        <button
            type="button"
            onClick={onClick}
            disabled={disabled}
            className={clsx(
                'inline-flex items-center px-3 py-1.5 text-xs rounded-md border transition-colors',
                isSelected ? 'bg-primary/10 border-primary text-foreground' : 'bg-card border-border opacity-50',
                disabled ? 'cursor-not-allowed' : 'cursor-pointer'
            )}
            data-testid={props['data-testid']}
        >
            {word}
        </button>
    );
};
