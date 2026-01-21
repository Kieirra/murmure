import React from 'react';
import { Input } from '@/components/input';
import { Switch } from '@/components/switch';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from '@/components/tooltip';
import { CircleHelp } from 'lucide-react';

interface RuleFormFieldsProps {
    trigger: string;
    replacement: string;
    exactMatch: boolean;
    useRegex: boolean;
    onTriggerChange: (value: string) => void;
    onReplacementChange: (value: string) => void;
    onExactMatchChange: (value: boolean) => void;
    onUseRegexChange: (value: boolean) => void;
    onKeyDown?: (e: React.KeyboardEvent) => void;
    testIdPrefix?: string;
}

export const RuleFormFields: React.FC<RuleFormFieldsProps> = ({
    trigger,
    replacement,
    exactMatch,
    useRegex,
    onTriggerChange,
    onReplacementChange,
    onExactMatchChange,
    onUseRegexChange,
    onKeyDown,
    testIdPrefix = 'rule',
}) => {
    const { t } = useTranslation();

    return (
        <div className="space-y-3">
            <div className="space-y-1">
                <Typography.Paragraph className="text-sm">
                    {t('Text to search')}
                </Typography.Paragraph>
                <Input
                    value={trigger}
                    onChange={(e) => onTriggerChange(e.target.value)}
                    onKeyDown={onKeyDown}
                    placeholder={t('e.g., new line')}
                    className="bg-zinc-900!"
                    data-testid={`${testIdPrefix}-trigger`}
                />
            </div>
            <div className="space-y-1 mb-1">
                <div className="flex items-center gap-2">
                    <Typography.Paragraph className="text-sm">
                        {t('Replacement text')}
                    </Typography.Paragraph>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <CircleHelp className="w-4 h-4 text-zinc-500 hover:text-zinc-300 cursor-help" />
                        </TooltipTrigger>
                        <TooltipContent className="max-w-[300px]">
                            <p>
                                {t(
                                    'You can write in natural language.'
                                )}
                            </p>
                            <p className="mt-1">
                                {t(
                                    'Use the Enter key for real line breaks (instead of \\n).'
                                )}
                            </p>
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
            <div className="flex items-center justify-between">
                <div className="space-y-1">
                    <Typography.Paragraph className="text-sm">
                        {t('Exact match')}
                    </Typography.Paragraph>
                    <Typography.Paragraph className="text-xs italic text-zinc-500">
                        {t(
                            'Enable for exact match. Disable for smart matching (handles surrounding punctuation).'
                        )}
                    </Typography.Paragraph>
                </div>
                <Switch
                    checked={exactMatch}
                    onCheckedChange={onExactMatchChange}
                    data-testid={`${testIdPrefix}-exact-match`}
                />
            </div>

            <div className="flex items-center justify-between">
                <div className="space-y-1">
                    <Typography.Paragraph className="text-sm">
                        {t('Regex')}
                    </Typography.Paragraph>
                    <Typography.Paragraph className="text-xs italic text-zinc-500">
                        {t(
                            'Enable to treat the "Text to search" as a regular expression.'
                        )}
                    </Typography.Paragraph>
                </div>
                <Switch
                    checked={useRegex}
                    onCheckedChange={onUseRegexChange}
                    data-testid={`${testIdPrefix}-use-regex`}
                />
            </div>
        </div>
    );
};
