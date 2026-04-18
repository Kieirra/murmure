import type { TranslationEntry } from '../types';
import { LANGUAGES } from '../constants/languages';

export const nameForCode = (code: string): string =>
    LANGUAGES.find((l) => l.code === code)?.name ?? code.toUpperCase();

export const isOnLeft = (entry: TranslationEntry, langB: string): boolean =>
    entry.detectedLang === null || entry.detectedLang !== langB;
