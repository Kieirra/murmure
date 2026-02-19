import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { MatchMode } from '@/features/settings/formatting-rules/types';
import clsx from 'clsx';

interface MatchModeToggleProps {
    value: MatchMode;
    onChange: (mode: MatchMode) => void;
    testIdPrefix?: string;
}

const modes: MatchMode[] = ['smart', 'exact', 'regex'];

const modeLabels: Record<MatchMode, string> = {
    smart: 'Smart',
    exact: 'Exact',
    regex: 'Regex',
};

const modeDescriptions: Record<MatchMode, string> = {
    smart: 'Handles surrounding punctuation and case-insensitive.',
    exact: 'Matches the exact text, case-sensitive.',
    regex: 'Use a regular expression pattern. Case-sensitive by default, add (?i) for case-insensitive.',
};

export const MatchModeToggle: React.FC<MatchModeToggleProps> = ({
    value,
    onChange,
    testIdPrefix = 'match-mode',
}) => {
    const { t } = useTranslation();

    return (
        <div className="space-y-1">
            <Typography.Paragraph className="text-sm">
                {t('Match mode')}
            </Typography.Paragraph>
            <fieldset
                className="inline-flex rounded-md border border-zinc-700"
                aria-label={t('Match mode')}
            >
                {modes.map((mode) => (
                    <Button
                        key={mode}
                        variant="ghost"
                        size="sm"
                        className={clsx(
                            'rounded-none first:rounded-l-md last:rounded-r-md border-0',
                            value === mode
                                ? 'bg-zinc-700 text-white hover:bg-zinc-700 hover:text-white'
                                : 'text-zinc-400'
                        )}
                        aria-pressed={value === mode}
                        onClick={() => onChange(mode)}
                        data-testid={`${testIdPrefix}-${mode}`}
                    >
                        {t(modeLabels[mode])}
                    </Button>
                ))}
            </fieldset>
            <Typography.Paragraph className="text-xs italic text-zinc-500">
                {t(modeDescriptions[value])}
            </Typography.Paragraph>
        </div>
    );
};
