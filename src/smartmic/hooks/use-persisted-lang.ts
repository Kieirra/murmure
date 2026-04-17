import { useState } from 'react';
import { LANGUAGES } from '../constants/languages';

export const usePersistedLang = (
    storageKey: string,
    fallback: string,
): [string, (code: string) => void] => {
    const [lang, setLang] = useState<string>(() => {
        const stored = localStorage.getItem(storageKey);
        const match = LANGUAGES.find((l) => l.code === stored);
        return match ? match.code : fallback;
    });

    const setPersistedLang = (code: string) => {
        const match = LANGUAGES.find((l) => l.code === code);
        if (!match) return;
        setLang(match.code);
        localStorage.setItem(storageKey, match.code);
    };

    return [lang, setPersistedLang];
};
