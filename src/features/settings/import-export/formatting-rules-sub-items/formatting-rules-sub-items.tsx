import { Switch } from '@/components/switch';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { FormattingRule } from '@/features/settings/formatting-rules/types';
import { subItemKey } from '../constants';
import { formatRuleLabel } from '../helpers';

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
                        checked={selection[subItemKey.rule(rule.id)] ?? false}
                        onCheckedChange={(checked) => onToggle(subItemKey.rule(rule.id), checked)}
                        disabled={disabled}
                        aria-label={formatRuleLabel(rule)}
                    />
                    <span className="text-sm text-muted-foreground truncate">
                        <span className="font-medium text-foreground">{rule.trigger || t('(empty trigger)')}</span>
                        {' \u2192 '}
                        {rule.replacement.length > 20
                            ? `${rule.replacement.replaceAll('\n', '\u21B5').substring(0, 20)}...`
                            : rule.replacement.replaceAll('\n', '\u21B5') || t('(delete)')}
                    </span>
                </label>
            ))}
        </>
    );
};
