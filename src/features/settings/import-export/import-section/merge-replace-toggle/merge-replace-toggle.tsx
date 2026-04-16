import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { ImportStrategy } from '../../import-export.types';

interface MergeReplaceToggleProps {
    value: ImportStrategy;
    onChange: (strategy: ImportStrategy) => void;
    disabled?: boolean;
}

export const MergeReplaceToggle = ({ value, onChange, disabled = false }: MergeReplaceToggleProps) => {
    const { t } = useTranslation();

    const options: { value: ImportStrategy; label: string; rounded: string }[] = [
        { value: 'replace', label: t('Replace'), rounded: 'rounded-l-md' },
        { value: 'merge', label: t('Merge'), rounded: 'rounded-r-md' },
    ];

    return (
        <div
            className="inline-flex rounded-md border border-border"
            role="radiogroup"
            aria-label={t('Import strategy')}
        >
            {options.map((opt) => (
                <button
                    key={opt.value}
                    type="button"
                    role="radio"
                    aria-checked={value === opt.value}
                    disabled={disabled}
                    className={clsx(
                        'px-3 py-1 text-xs font-medium transition-colors',
                        opt.rounded,
                        value === opt.value
                            ? 'bg-sky-600 text-white'
                            : 'bg-transparent text-muted-foreground hover:bg-accent',
                        disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
                    )}
                    onClick={() => onChange(opt.value)}
                >
                    {opt.label}
                </button>
            ))}
        </div>
    );
};
