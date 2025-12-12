import React from 'react';
import { Input } from '@/components/input';
import { Switch } from '@/components/switch';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';

interface RuleFormFieldsProps {
    trigger: string;
    replacement: string;
    exactMatch: boolean;
    onTriggerChange: (value: string) => void;
    onReplacementChange: (value: string) => void;
    onExactMatchChange: (value: boolean) => void;
    onKeyDown?: (e: React.KeyboardEvent) => void;
    testIdPrefix?: string;
}

export const RuleFormFields: React.FC<RuleFormFieldsProps> = ({
    trigger,
    replacement,
    exactMatch,
    onTriggerChange,
    onReplacementChange,
    onExactMatchChange,
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
                <Typography.Paragraph className="text-sm">
                    {t('Replacement text')}
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
        </div>
    );
};
