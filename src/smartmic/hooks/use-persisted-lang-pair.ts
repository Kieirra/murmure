import { useState } from 'react';
import { LANGUAGES } from '../constants/languages';

export interface LangPair {
    a: string;
    b: string;
}

const STORAGE_KEY = 'smartmic_translation_pair';

const isValidCode = (code: string): boolean => LANGUAGES.some((l) => l.code === code);

const readInitial = (fallback: LangPair): LangPair => {
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        if (raw === null) return fallback;
        const parsed: unknown = JSON.parse(raw);
        if (
            parsed !== null &&
            typeof parsed === 'object' &&
            'a' in parsed &&
            'b' in parsed &&
            typeof (parsed as { a: unknown }).a === 'string' &&
            typeof (parsed as { b: unknown }).b === 'string'
        ) {
            const a = (parsed as { a: string }).a;
            const b = (parsed as { b: string }).b;
            if (isValidCode(a) && isValidCode(b)) return { a, b };
        }
    } catch {
        // invalid JSON - fall through
    }
    return fallback;
};

export const usePersistedLangPair = (
    fallback: LangPair = { a: 'en', b: 'fr' }
): [LangPair, (next: LangPair) => void] => {
    const [pair, setPair] = useState<LangPair>(() => readInitial(fallback));

    const setPersisted = (next: LangPair) => {
        if (!isValidCode(next.a) || !isValidCode(next.b)) return;
        setPair(next);
        try {
            localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
        } catch {
            // localStorage unavailable: keep in-memory state only
        }
    };

    return [pair, setPersisted];
};
