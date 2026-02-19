import React from 'react';
import { CircleHelp } from 'lucide-react';
import { Input } from '@/components/input';
import { Typography } from '@/components/typography';
import {
    Tooltip,
    TooltipTrigger,
    TooltipContent,
} from '@/components/tooltip';
import { MatchModeToggle } from '@/components/match-mode-toggle';
import { useTranslation } from '@/i18n';
import { MatchMode } from '@/features/settings/formatting-rules/types';

interface RuleFormFieldsProps {
    trigger: string;
    replacement: string;
    matchMode: MatchMode;
    onTriggerChange: (value: string) => void;
    onReplacementChange: (value: string) => void;
    onMatchModeChange: (mode: MatchMode) => void;
    regexError?: string | null;
    onKeyDown?: (e: React.KeyboardEvent) => void;
    testIdPrefix?: string;
}

export const RuleFormFields: React.FC<RuleFormFieldsProps> = ({
    trigger,
    replacement,
    matchMode,
    onTriggerChange,
    onReplacementChange,
    onMatchModeChange,
    regexError,
    onKeyDown,
    testIdPrefix = 'rule',
}) => {
    const { t } = useTranslation();

    const triggerPlaceholder =
        matchMode === 'regex'
            ? t(String.raw`e.g., (?i)open(ing)?\s+quotes?`)
            : t('e.g., new line');

    return (
        <div className="space-y-3">
            <div className="space-y-1">
                <Typography.Paragraph className="text-sm">
                    {t('If the transcript contains')}
                </Typography.Paragraph>
                <Input
                    value={trigger}
                    onChange={(e) => onTriggerChange(e.target.value)}
                    onKeyDown={onKeyDown}
                    placeholder={triggerPlaceholder}
                    className={`bg-zinc-900! ${regexError == null ? '' : 'border-red-500'}`}
                    data-testid={`${testIdPrefix}-trigger`}
                />
                {regexError != null && (
                    <Typography.Paragraph className="text-xs text-red-400">
                        {t('Invalid regex: {{error}}', { error: regexError })}
                    </Typography.Paragraph>
                )}
            </div>
            <div className="space-y-1 mb-1">
                <div className="flex items-center gap-1.5">
                    <Typography.Paragraph className="text-sm">
                        {t('Then replace it with')}
                    </Typography.Paragraph>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <CircleHelp className="w-3.5 h-3.5 text-zinc-500 cursor-help" />
                        </TooltipTrigger>
                        <TooltipContent className="max-w-xs">
                            <p>{t(String.raw`Use real line breaks (Enter key) to insert new lines, not \n.`)}</p>
                            <p>{t('Leave empty to delete the matched text.')}</p>
                            {matchMode === 'regex' && (
                                <>
                                    <p>{t('Use $1, $2... to insert captured groups from the regex.')}</p>
                                    <p>{t('Use $$ for a literal dollar sign.')}</p>
                                </>
                            )}
                        </TooltipContent>
                    </Tooltip>
                </div>
                <textarea
                    value={replacement}
                    onChange={(e) => onReplacementChange(e.target.value)}
                    placeholder={t('e.g., (leave empty to delete)')}
                    className="w-full bg-zinc-900 border border-zinc-700 rounded-md px-3 py-2 text-sm text-white placeholder:text-zinc-500 focus:outline-none focus:ring-2 focus:ring-sky-500 min-h-[60px] resize-y"
                    data-testid={`${testIdPrefix}-replacement`}
                />
            </div>
            <MatchModeToggle
                value={matchMode}
                onChange={onMatchModeChange}
                testIdPrefix={`${testIdPrefix}-match-mode`}
            />
        </div>
    );
};
