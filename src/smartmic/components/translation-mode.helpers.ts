// Helpers specific to translation-mode.tsx. Promote to ../helpers/ if a second consumer appears.
import type { TranslationEntry } from '../smartmic.types';
import { LANGUAGES } from '../constants/languages';

export const nameForCode = (code: string): string =>
    LANGUAGES.find((l) => l.code === code)?.name ?? code.toUpperCase();

export const isOnLeft = (entry: TranslationEntry, langB: string): boolean =>
    entry.detectedLang === null || entry.detectedLang !== langB;
