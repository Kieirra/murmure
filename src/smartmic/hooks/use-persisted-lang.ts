import { useState } from 'react';
import { LANGUAGES } from '../constants/languages';

export const usePersistedLang = (
    storageKey: string,
    fallback: string,
): [string, (code: string) => void] => {
    const [lang, setLang] = useState<string>(() => {
        const stored = localStorage.getItem(storageKey);
        return LANGUAGES.some((l) => l.code === stored) ? (stored as string) : fallback;
    });

    const setPersistedLang = (code: string) => {
        if (!LANGUAGES.some((l) => l.code === code)) return;
        setLang(code);
        localStorage.setItem(storageKey, code);
    };

    return [lang, setPersistedLang];
};
