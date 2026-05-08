import { useState } from 'react';
import { FormattingRule } from '@/features/personalize/formatting-rules/types';
import {
    MAX_RULE_NAME_LENGTH,
    RULE_NAME_COUNTER_THRESHOLD,
    normalizeRuleName,
} from '../rule-card.helpers';

type RuleUpdate = (id: string, updates: Partial<Omit<FormattingRule, 'id'>>) => void;

export const useRenameRule = (rule: FormattingRule, onUpdate: RuleUpdate) => {
    const [isRenaming, setIsRenaming] = useState(false);
    const [draftName, setDraftName] = useState('');

    const startRenaming = () => {
        setDraftName(rule.name ?? '');
        setIsRenaming(true);
    };

    const cancelRenaming = () => {
        setIsRenaming(false);
    };

    const commitRename = () => {
        if (!isRenaming) {
            return;
        }
        const normalized = normalizeRuleName(draftName);
        if (normalized !== rule.name) {
            onUpdate(rule.id, { name: normalized });
        }
        setIsRenaming(false);
    };

    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            commitRename();
        } else if (e.key === 'Escape') {
            e.preventDefault();
            cancelRenaming();
        }
    };

    const showCounter = draftName.length >= RULE_NAME_COUNTER_THRESHOLD;

    return {
        isRenaming,
        draftName,
        setDraftName,
        startRenaming,
        cancelRenaming,
        commitRename,
        handleKeyDown,
        showCounter,
        maxLength: MAX_RULE_NAME_LENGTH,
    };
};
