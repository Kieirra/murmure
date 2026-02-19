import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { MatchMode } from '../types';

export const useRegexValidation = (
    trigger: string,
    matchMode: MatchMode
): string | null => {
    const [regexError, setRegexError] = useState<string | null>(null);

    useEffect(() => {
        if (matchMode !== 'regex' || trigger.trim().length === 0) {
            setRegexError(null);
            return;
        }

        const timeout = setTimeout(async () => {
            try {
                await invoke('validate_regex', { pattern: trigger });
                setRegexError(null);
            } catch (error) {
                setRegexError(String(error));
            }
        }, 300);

        return () => clearTimeout(timeout);
    }, [trigger, matchMode]);

    return regexError;
};
