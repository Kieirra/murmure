import React, { useState } from 'react';
import { Plus, Lightbulb } from 'lucide-react';
import { Input } from '@/components/input';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';

interface AddRuleSectionProps {
    onAdd: (trigger: string, replacement: string) => void;
}

export const AddRuleSection: React.FC<AddRuleSectionProps> = ({ onAdd }) => {
    const [trigger, setTrigger] = useState('');
    const [replacement, setReplacement] = useState('');
    const { t } = useTranslation();

    const handleAdd = () => {
        if (!trigger.trim()) return;
        onAdd(trigger, replacement);
        setTrigger('');
        setReplacement('');
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleAdd();
        }
    };

    return (
        <div className="border border-dashed border-zinc-700 rounded-lg p-4 bg-zinc-800/30">
            <div className="flex items-center gap-2 mb-4">
                <Plus className="w-5 h-5 text-sky-500" />
                <span className="font-medium text-white">
                    {t('Add a new rule')}
                </span>
            </div>

            <div className="space-y-3">
                <div>
                    <label className="block text-xs text-zinc-400 mb-1">
                        {t('Trigger text (what to find)')}
                    </label>
                    <Input
                        value={trigger}
                        onChange={(e) => setTrigger(e.target.value)}
                        onKeyDown={handleKeyDown}
                        placeholder={t('e.g., new line')}
                        className="bg-zinc-900"
                        data-testid="add-rule-trigger"
                    />
                </div>
                <div>
                    <label className="block text-xs text-zinc-400 mb-1">
                        {t('Replacement text')}
                    </label>
                    <textarea
                        value={replacement}
                        onChange={(e) => setReplacement(e.target.value)}
                        placeholder={t(
                            'e.g., (leave empty to delete the trigger)'
                        )}
                        className="w-full bg-zinc-900 border border-zinc-700 rounded-md px-3 py-2 text-sm text-white placeholder:text-zinc-500 focus:outline-none focus:ring-2 focus:ring-sky-500 min-h-[60px] resize-y"
                        data-testid="add-rule-replacement"
                    />
                </div>
                <Page.SecondaryButton
                    onClick={handleAdd}
                    disabled={!trigger.trim()}
                    data-testid="add-rule-button"
                >
                    {t('Add rule')}
                </Page.SecondaryButton>
            </div>

            {/* Examples section */}
            <div className="mt-6 pt-4 border-t border-zinc-700">
                <div className="flex items-center gap-2 mb-3">
                    <Lightbulb className="w-4 h-4 text-amber-500" />
                    <span className="text-sm font-medium text-zinc-300">
                        {t('Examples of what you can do')}
                    </span>
                </div>
                <ul className="text-xs text-zinc-400 space-y-2">
                    <li>
                        <span className="text-zinc-300">
                            {t('Line break')}:
                        </span>{' '}
                        {t('"new line" → ↵ (one line break)')}
                    </li>
                    <li>
                        <span className="text-zinc-300">
                            {t('New paragraph')}:
                        </span>{' '}
                        {t('"new paragraph" → ↵↵ (two line breaks)')}
                    </li>
                    <li>
                        <span className="text-zinc-300">
                            {t('Remove markers')}:
                        </span>{' '}
                        {t('"[silence]" → (empty, deletes the marker)')}
                    </li>
                    <li>
                        <span className="text-zinc-300">{t('Templates')}:</span>{' '}
                        {t(
                            '"meeting notes" → formatted block with Date, Participants, Decisions fields'
                        )}
                    </li>
                    <li>
                        <span className="text-zinc-300">
                            {t('Typography')}:
                        </span>{' '}
                        {t("Replace '\"' with « » for French quotes")}
                    </li>
                </ul>
            </div>
        </div>
    );
};
