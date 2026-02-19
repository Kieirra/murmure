import React, { useState } from 'react';
import { Plus } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { RuleFormFields } from './rule-form-fields';
import { MatchMode } from '@/features/settings/formatting-rules/types';
import { useRegexValidation } from '@/features/settings/formatting-rules/hooks/use-regex-validation';

interface AddRuleSectionProps {
    onAdd: (trigger: string, replacement: string, matchMode: MatchMode) => void;
}

export const AddRuleSection: React.FC<AddRuleSectionProps> = ({ onAdd }) => {
    const [trigger, setTrigger] = useState('');
    const [replacement, setReplacement] = useState('');
    const [matchMode, setMatchMode] = useState<MatchMode>('smart');
    const { t } = useTranslation();

    const regexError = useRegexValidation(trigger, matchMode);

    const isAddDisabled =
        trigger.trim().length === 0 || (matchMode === 'regex' && regexError != null);

    const handleAdd = () => {
        if (isAddDisabled) return;
        onAdd(trigger, replacement, matchMode);
        setTrigger('');
        setReplacement('');
        setMatchMode('smart');
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
                    {t('Add a custom rule')}
                </span>
            </div>

            <RuleFormFields
                trigger={trigger}
                replacement={replacement}
                matchMode={matchMode}
                onTriggerChange={setTrigger}
                onReplacementChange={setReplacement}
                onMatchModeChange={setMatchMode}
                regexError={regexError}
                onKeyDown={handleKeyDown}
                testIdPrefix="add-rule"
            />

            <div className="mt-3">
                <Page.SecondaryButton
                    onClick={handleAdd}
                    disabled={isAddDisabled}
                    data-testid="add-rule-button"
                >
                    {t('Add rule')}
                </Page.SecondaryButton>
            </div>
        </div>
    );
};
