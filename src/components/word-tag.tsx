import clsx from 'clsx';

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

    if (variant === 'removable') {
        return (
            <button
                type="button"
                onClick={onClick}
                disabled={disabled}
                className="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs bg-card hover:bg-accent text-foreground rounded-md border border-border transition-colors"
                data-testid={props['data-testid']}
            >
                <span>{word}</span>
                <span className="text-muted-foreground">&times;</span>
            </button>
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
