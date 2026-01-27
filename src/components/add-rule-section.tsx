import React, { useState } from 'react';
import { Plus } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { RuleFormFields } from './rule-form-fields';

interface AddRuleSectionProps {
    onAdd: (trigger: string, replacement: string, exactMatch: boolean, isRegex: boolean) => void;
}

export const AddRuleSection: React.FC<AddRuleSectionProps> = ({ onAdd }) => {
    const [trigger, setTrigger] = useState('');
    const [replacement, setReplacement] = useState('');
    const [exactMatch, setExactMatch] = useState(false);
    const [isRegex, setIsRegex] = useState(false);
    const { t } = useTranslation();

    const handleAdd = () => {
        if (!trigger.trim()) return;
        onAdd(trigger, replacement, exactMatch, isRegex);
        setTrigger('');
        setReplacement('');
        setExactMatch(false);
        setIsRegex(false);
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
                exactMatch={exactMatch}
                isRegex={isRegex}
                onTriggerChange={setTrigger}
                onReplacementChange={setReplacement}
                onExactMatchChange={setExactMatch}
                onIsRegexChange={setIsRegex}
                onKeyDown={handleKeyDown}
                testIdPrefix="add-rule"
            />

            <div className="mt-3">
                <Page.SecondaryButton
                    onClick={handleAdd}
                    disabled={!trigger.trim()}
                    data-testid="add-rule-button"
                >
                    {t('Add rule')}
                </Page.SecondaryButton>
            </div>
        </div>
    );
};
