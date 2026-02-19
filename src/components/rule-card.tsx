import React, { useState } from 'react';
import { FormattingRule } from '../features/settings/formatting-rules/types';
import { Switch } from '@/components/switch';
import { Trash2, Copy, ChevronDown, ChevronUp, Regex } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { Button } from './button';
import { RuleFormFields } from './rule-form-fields';
import { useRegexValidation } from '@/features/settings/formatting-rules/hooks/use-regex-validation';

interface RuleCardProps {
    rule: FormattingRule;
    onUpdate: (
        id: string,
        updates: Partial<Omit<FormattingRule, 'id'>>
    ) => void;
    onDelete: (id: string) => void;
    onDuplicate: (id: string) => void;
}

export const RuleCard: React.FC<RuleCardProps> = ({
    rule,
    onUpdate,
    onDelete,
    onDuplicate,
}) => {
    const [isExpanded, setIsExpanded] = useState(false);
    const { t } = useTranslation();

    const regexError = useRegexValidation(rule.trigger, rule.match_mode);

    return (
        <div
            className={`border rounded-lg p-4 transition-all ${
                rule.enabled
                    ? 'border-zinc-700 bg-zinc-800/25'
                    : 'border-zinc-800 bg-zinc-900/50 opacity-60'
            }`}
            data-testid={`rule-card-${rule.id}`}
        >
            <div className="flex items-center justify-between gap-4">
                <div className="flex items-center gap-3 flex-1 min-w-0">
                    <Switch
                        checked={rule.enabled}
                        onCheckedChange={(checked) =>
                            onUpdate(rule.id, { enabled: checked })
                        }
                        data-testid={`rule-toggle-${rule.id}`}
                    />
                    {rule.match_mode === 'regex' && (
                        <Regex className="w-3.5 h-3.5 text-zinc-500 shrink-0" />
                    )}
                    <span className="text-sm font-medium text-white truncate">
                        {rule.trigger || t('(empty trigger)')}
                    </span>
                    <span className="text-zinc-500">→</span>
                    <span className="text-sm text-zinc-400 truncate">
                        {rule.replacement.length > 20
                            ? `${rule.replacement
                                  .replaceAll('\n', '↵')
                                  .substring(0, 20)}...`
                            : rule.replacement.replaceAll('\n', '↵') ||
                              t('(delete)')}
                    </span>
                </div>
                <div className="flex items-center gap-1">
                    <Button
                        variant="ghost"
                        onClick={() => onDuplicate(rule.id)}
                        className="p-2 text-zinc-500 hover:text-zinc-300 hover:bg-zinc-700 rounded-md transition-colors"
                        title={t('Duplicate')}
                        data-testid={`rule-duplicate-${rule.id}`}
                    >
                        <Copy className="w-4 h-4" />
                    </Button>
                    <Button
                        variant="ghost"
                        onClick={() => onDelete(rule.id)}
                        className="p-2 text-zinc-500 hover:text-red-400 hover:bg-zinc-700 rounded-md transition-colors"
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
                            <ChevronUp className="w-4 h-4 text-zinc-500 flex-shrink-0" />
                        ) : (
                            <ChevronDown className="w-4 h-4 text-zinc-500 flex-shrink-0" />
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
                        onTriggerChange={(value) =>
                            onUpdate(rule.id, { trigger: value })
                        }
                        onReplacementChange={(value) =>
                            onUpdate(rule.id, { replacement: value })
                        }
                        onMatchModeChange={(mode) =>
                            onUpdate(rule.id, { match_mode: mode })
                        }
                        regexError={regexError}
                        testIdPrefix={`rule-${rule.id}`}
                    />
                </div>
            )}
        </div>
    );
};
