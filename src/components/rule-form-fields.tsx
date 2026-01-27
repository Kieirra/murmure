import React from 'react';
import { HelpCircle } from 'lucide-react';
import { Input } from '@/components/input';
import { Switch } from '@/components/switch';
import { Typography } from '@/components/typography';
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from '@/components/tooltip';
import { useTranslation } from '@/i18n';

interface RuleFormFieldsProps {
    trigger: string;
    replacement: string;
    exactMatch: boolean;
    isRegex: boolean;
    onTriggerChange: (value: string) => void;
    onReplacementChange: (value: string) => void;
    onExactMatchChange: (value: boolean) => void;
    onIsRegexChange: (value: boolean) => void;
    onKeyDown?: (e: React.KeyboardEvent) => void;
    testIdPrefix?: string;
}

export const RuleFormFields: React.FC<RuleFormFieldsProps> = ({
    trigger,
    replacement,
    exactMatch,
    isRegex,
    onTriggerChange,
    onReplacementChange,
    onExactMatchChange,
    onIsRegexChange,
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
                <Typography.Paragraph className="text-sm flex items-center gap-1">
                    {t('Replacement text')}
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <HelpCircle className="w-4 h-4 text-zinc-500 cursor-help hover:text-zinc-400" />
                        </TooltipTrigger>
                        <TooltipContent className="max-w-xs bg-zinc-800 text-zinc-200 border-zinc-700">
                            <p className="mb-1">{t('Type real line breaks (Enter key), not \\n')}</p>
                            <p>{t('Leave empty to delete the matched text')}</p>
                        </TooltipContent>
                    </Tooltip>
                </Typography.Paragraph>
                <textarea
                    value={replacement}
                    onChange={(e) => onReplacementChange(e.target.value)}
                    placeholder={t('e.g., (leave empty to delete)')}
                    className="w-full bg-zinc-900 border border-zinc-700 rounded-md px-3 py-2 text-sm text-white placeholder:text-zinc-500 focus:outline-none focus:ring-2 focus:ring-sky-500 min-h-[60px] resize-y"
                    data-testid={`${testIdPrefix}-replacement`}
                />
            </div>
            <div className="flex items-center justify-between">
                <div className="space-y-1 flex-1">
                    <Typography.Paragraph className="text-sm flex items-center gap-1">
                        {t('Regex')}
                        <Tooltip>
                            <TooltipTrigger asChild>
                                <HelpCircle className="w-4 h-4 text-zinc-500 cursor-help hover:text-zinc-400" />
                            </TooltipTrigger>
                            <TooltipContent className="max-w-sm bg-zinc-800 text-zinc-200 border-zinc-700">
                                <div className="space-y-2">
                                    <p className="text-xs text-zinc-400">
                                        {t('Use regular expression pattern for matching')}
                                    </p>
                                    <p className="text-xs italic">
                                        {t('Example: \\d+ matches numbers, guillemets? matches "guillemet" or "guillemets"')}
                                    </p>
                                </div>
                            </TooltipContent>
                        </Tooltip>
                    </Typography.Paragraph>
                    <Typography.Paragraph className="text-xs italic text-zinc-500">
                        {t('Enable to use regex pattern matching')}
                    </Typography.Paragraph>
                </div>
                <Switch
                    checked={isRegex}
                    onCheckedChange={onIsRegexChange}
                    data-testid={`${testIdPrefix}-is-regex`}
                />
            </div>
            <div className="flex items-center justify-between">
                <div className="space-y-1 flex-1">
                    <Typography.Paragraph className={`text-sm flex items-center gap-1 ${isRegex ? 'text-zinc-500' : ''}`}>
                        {t('Exact match')}
                        <Tooltip>
                            <TooltipTrigger asChild>
                                <HelpCircle className="w-4 h-4 text-zinc-500 cursor-help hover:text-zinc-400" />
                            </TooltipTrigger>
                            <TooltipContent className="max-w-sm bg-zinc-800 text-zinc-200 border-zinc-700">
                                <div className="space-y-2">
                                    <div>
                                        <p className="font-medium mb-1">
                                            {t('Exact match (enabled):')}
                                        </p>
                                        <p className="text-xs text-zinc-400">
                                            {t('Simple literal replacement')}
                                        </p>
                                        <p className="text-xs italic mt-1">
                                            {t('Example: "open quote" → "\""')}
                                        </p>
                                    </div>
                                    <div>
                                        <p className="font-medium mb-1">
                                            {t('Smart match (disabled):')}
                                        </p>
                                        <p className="text-xs text-zinc-400">
                                            {t('Ignores punctuation around the trigger')}
                                        </p>
                                        <p className="text-xs italic mt-1">
                                            {t('Example: "open quote." → "\"."')}
                                        </p>
                                    </div>
                                </div>
                            </TooltipContent>
                        </Tooltip>
                    </Typography.Paragraph>
                    <Typography.Paragraph className={`text-xs italic ${isRegex ? 'text-zinc-600' : 'text-zinc-500'}`}>
                        {isRegex
                            ? t('Disabled when regex is enabled')
                            : t('Enable for literal replacement. Disable to ignore surrounding punctuation.')}
                    </Typography.Paragraph>
                </div>
                <Switch
                    checked={exactMatch}
                    onCheckedChange={onExactMatchChange}
                    disabled={isRegex}
                    data-testid={`${testIdPrefix}-exact-match`}
                />
            </div>
        </div>
    );
};
