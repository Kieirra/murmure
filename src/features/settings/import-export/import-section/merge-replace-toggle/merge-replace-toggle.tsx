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

    return (
        <div
            className="inline-flex rounded-md border border-border"
            role="radiogroup"
            aria-label={t('Import strategy')}
        >
            <button
                type="button"
                role="radio"
                aria-checked={value === 'replace'}
                disabled={disabled}
                className={clsx(
                    'px-3 py-1 text-xs font-medium transition-colors rounded-l-md',
                    value === 'replace'
                        ? 'bg-sky-600 text-white'
                        : 'bg-transparent text-muted-foreground hover:bg-accent',
                    disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
                )}
                onClick={() => onChange('replace')}
            >
                {t('Replace')}
            </button>
            <button
                type="button"
                role="radio"
                aria-checked={value === 'merge'}
                disabled={disabled}
                className={clsx(
                    'px-3 py-1 text-xs font-medium transition-colors rounded-r-md',
                    value === 'merge'
                        ? 'bg-sky-600 text-white'
                        : 'bg-transparent text-muted-foreground hover:bg-accent',
                    disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
                )}
                onClick={() => onChange('merge')}
            >
                {t('Merge')}
            </button>
        </div>
    );
};
