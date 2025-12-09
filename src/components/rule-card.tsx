import React, { useState } from 'react';
import { FormattingRule } from '../features/settings/formatting-rules/types';
import { Switch } from '@/components/switch';
import { Input } from '@/components/input';
import { Trash2, Copy, ChevronDown, ChevronUp } from 'lucide-react';
import { useTranslation } from '@/i18n';

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
    const [isExpanded, setIsExpanded] = useState(true);
    const { t } = useTranslation();

    return (
        <div
            className={`border rounded-lg p-4 transition-all ${
                rule.enabled
                    ? 'border-zinc-700 bg-zinc-800/50'
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
                    <button
                        className="flex items-center gap-2 text-left flex-1 min-w-0"
                        onClick={() => setIsExpanded(!isExpanded)}
                    >
                        <span className="text-sm font-medium text-white truncate">
                            {rule.trigger || t('(empty trigger)')}
                        </span>
                        <span className="text-zinc-500">→</span>
                        <span className="text-sm text-zinc-400 truncate">
                            {rule.replacement.length > 20
                                ? `${rule.replacement
                                      .replace(/\n/g, '↵')
                                      .substring(0, 20)}...`
                                : rule.replacement.replace(/\n/g, '↵') ||
                                  t('(delete)')}
                        </span>
                        {isExpanded ? (
                            <ChevronUp className="w-4 h-4 text-zinc-500 flex-shrink-0" />
                        ) : (
                            <ChevronDown className="w-4 h-4 text-zinc-500 flex-shrink-0" />
                        )}
                    </button>
                </div>
                <div className="flex items-center gap-1">
                    <button
                        onClick={() => onDuplicate(rule.id)}
                        className="p-2 text-zinc-500 hover:text-zinc-300 hover:bg-zinc-700 rounded-md transition-colors"
                        title={t('Duplicate')}
                        data-testid={`rule-duplicate-${rule.id}`}
                    >
                        <Copy className="w-4 h-4" />
                    </button>
                    <button
                        onClick={() => onDelete(rule.id)}
                        className="p-2 text-zinc-500 hover:text-red-400 hover:bg-zinc-700 rounded-md transition-colors"
                        title={t('Delete')}
                        data-testid={`rule-delete-${rule.id}`}
                    >
                        <Trash2 className="w-4 h-4" />
                    </button>
                </div>
            </div>

            {isExpanded && (
                <div className="mt-4 space-y-3">
                    <div>
                        <label className="block text-xs text-zinc-400 mb-1">
                            {t('Trigger text (what to find)')}
                        </label>
                        <Input
                            value={rule.trigger}
                            onChange={(e) =>
                                onUpdate(rule.id, { trigger: e.target.value })
                            }
                            placeholder={t('e.g., new line')}
                            className="bg-zinc-900"
                            data-testid={`rule-trigger-${rule.id}`}
                        />
                    </div>
                    <div>
                        <label className="block text-xs text-zinc-400 mb-1">
                            {t('Replacement text')}
                        </label>
                        <textarea
                            value={rule.replacement}
                            onChange={(e) =>
                                onUpdate(rule.id, {
                                    replacement: e.target.value,
                                })
                            }
                            placeholder={t('e.g., (leave empty to delete)')}
                            className="w-full bg-zinc-900 border border-zinc-700 rounded-md px-3 py-2 text-sm text-white placeholder:text-zinc-500 focus:outline-none focus:ring-2 focus:ring-sky-500 min-h-[60px] resize-y"
                            data-testid={`rule-replacement-${rule.id}`}
                        />
                    </div>
                </div>
            )}
        </div>
    );
};
