import { Switch } from '@/components/switch';
import { RuleSummary } from '@/components/rule-summary';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { FormattingRule } from '@/features/personalize/formatting-rules/types';
import { SUB_ITEM_KEY } from '../import-export.constants';
import { formatRuleLabel } from '../import-export.helpers';

interface FormattingRulesSubItemsProps {
    rules: FormattingRule[];
    selection: Record<string, boolean>;
    onToggle: (key: string, checked: boolean) => void;
    disabled?: boolean;
}

export const FormattingRulesSubItems = ({ rules, selection, onToggle, disabled }: FormattingRulesSubItemsProps) => {
    const { t } = useTranslation();

    return (
        <>
            <label className={clsx('flex items-center gap-2 py-1', disabled ? 'cursor-not-allowed' : 'cursor-pointer')}>
                <Switch
                    checked={selection['built_in'] ?? false}
                    onCheckedChange={(checked) => onToggle('built_in', checked)}
                    disabled={disabled}
                    aria-label={t('Built-in Options')}
                />
                <span className="text-sm text-muted-foreground">{t('Built-in Options')}</span>
            </label>
            {rules.map((rule) => (
                <label
                    key={rule.id}
                    className={clsx('flex items-center gap-2 py-1', disabled ? 'cursor-not-allowed' : 'cursor-pointer')}
                    style={rule.enabled ? undefined : { opacity: 0.5 }}
                >
                    <Switch
                        checked={selection[SUB_ITEM_KEY.rule(rule.id)] ?? false}
                        onCheckedChange={(checked) => onToggle(SUB_ITEM_KEY.rule(rule.id), checked)}
                        disabled={disabled}
                        aria-label={formatRuleLabel(rule)}
                    />
                    <RuleSummary trigger={rule.trigger} replacement={rule.replacement} />
                </label>
            ))}
        </>
    );
};
