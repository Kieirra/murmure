import { useState } from 'react';
import { FormattingRule } from '../features/personalize/formatting-rules/types';
import { Switch } from '@/components/switch';
import { Trash2, Copy, ChevronDown, ChevronUp, Regex, GripVertical } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { Button } from './button';
import { RuleFormFields } from './rule-form-fields';
import { RuleSummary } from './rule-summary';
import { useRegexValidation } from '@/features/personalize/formatting-rules/hooks/use-regex-validation';

interface RuleCardProps {
    rule: FormattingRule;
    onUpdate: (id: string, updates: Partial<Omit<FormattingRule, 'id'>>) => void;
    onDelete: (id: string) => void;
    onDuplicate: (id: string) => void;
    dragHandleProps?: Record<string, unknown>;
}

export const RuleCard = ({ rule, onUpdate, onDelete, onDuplicate, dragHandleProps }: RuleCardProps) => {
    const [isExpanded, setIsExpanded] = useState(false);
    const { t } = useTranslation();

    const regexError = useRegexValidation(rule.trigger, rule.match_mode);

    return (
        <div
            className={`border rounded-lg p-4 ${
                rule.enabled ? 'border-border bg-card/25' : 'border-border bg-background/50 opacity-60'
            }`}
            data-testid={`rule-card-${rule.id}`}
        >
            <div className="flex items-center justify-between gap-4">
                <div className="flex items-center gap-3 flex-1 min-w-0">
                    <button
                        type="button"
                        className="cursor-grab text-muted-foreground hover:text-muted-foreground transition-colors active:cursor-grabbing p-2 -m-2"
                        title={t('Reorder')}
                        {...dragHandleProps}
                    >
                        <GripVertical className="w-4 h-4" />
                    </button>
                    <Switch
                        checked={rule.enabled}
                        onCheckedChange={(checked) => onUpdate(rule.id, { enabled: checked })}
                        data-testid={`rule-toggle-${rule.id}`}
                    />
                    {rule.match_mode === 'regex' && <Regex className="w-3.5 h-3.5 text-muted-foreground shrink-0" />}
                    <RuleSummary trigger={rule.trigger} replacement={rule.replacement} />
                </div>
                <div className="flex items-center gap-1">
                    <Button
                        variant="ghost"
                        onClick={() => onDuplicate(rule.id)}
                        className="p-2 text-muted-foreground hover:text-foreground hover:bg-accent rounded-md transition-colors"
                        title={t('Duplicate')}
                        data-testid={`rule-duplicate-${rule.id}`}
                    >
                        <Copy className="w-4 h-4" />
                    </Button>
                    <Button
                        variant="ghost"
                        onClick={() => onDelete(rule.id)}
                        className="p-2 text-muted-foreground hover:text-red-400 hover:bg-accent rounded-md transition-colors"
                        title={t('Delete')}
                        data-testid={`rule-delete-${rule.id}`}
                    >
                        <Trash2 className="w-4 h-4" />
                    </Button>
                    <Button
                        variant="ghost"
                        className="flex items-center gap-2 text-left flex-1 min-w-0"
                        onClick={() => setIsExpanded(!isExpanded)}
                    >
                        {isExpanded ? (
                            <ChevronUp className="w-4 h-4 text-muted-foreground flex-shrink-0" />
                        ) : (
                            <ChevronDown className="w-4 h-4 text-muted-foreground flex-shrink-0" />
                        )}
                    </Button>
                </div>
            </div>

            {isExpanded && (
                <div className="mt-4">
                    <RuleFormFields
                        trigger={rule.trigger}
                        replacement={rule.replacement}
                        matchMode={rule.match_mode}
                        onTriggerChange={(value) => onUpdate(rule.id, { trigger: value })}
                        onReplacementChange={(value) => onUpdate(rule.id, { replacement: value })}
                        onMatchModeChange={(mode) => onUpdate(rule.id, { match_mode: mode })}
                        regexError={regexError}
                        testIdPrefix={`rule-${rule.id}`}
                    />
                </div>
            )}
        </div>
    );
};
