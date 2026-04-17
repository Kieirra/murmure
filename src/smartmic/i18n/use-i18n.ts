import { STRINGS, type Lang, type StringKey } from './strings';

const STORAGE_KEY = 'smartmic_lang';

const isLang = (value: string | null): value is Lang => value === 'en' || value === 'fr';

/**
 * Read the active language once per page load.
 * Priority: URL ?lang= param (persisted), then localStorage, then 'en'.
 */
const getLang = (): Lang => {
    try {
        const params = new URLSearchParams(globalThis.location.search);
        const urlLang = params.get('lang');
        if (urlLang !== null && isLang(urlLang)) {
            localStorage.setItem(STORAGE_KEY, urlLang);
            return urlLang;
        }
        const stored = localStorage.getItem(STORAGE_KEY);
        if (isLang(stored)) return stored;
    } catch {
        // localStorage unavailable (private mode): fall through
    }
    return 'en';
};

const format = (template: string, params?: Record<string, string | number>): string => {
    if (params == null) return template;
    return template.replace(/\{(\w+)\}/g, (match, key: string) => {
        const value = params[key];
        return value === undefined ? match : String(value);
    });
};

export interface I18n {
    lang: Lang;
    t: (key: StringKey, params?: Record<string, string | number>) => string;
}

// Language is read once per page load and never changes during the session,
// so the dict and `t` are constants — no memoization needed.
const LANG: Lang = getLang();
const DICT = STRINGS[LANG];
const t = (key: StringKey, params?: Record<string, string | number>): string =>
    format(DICT[key], params);

export const useI18n = (): I18n => ({ lang: LANG, t });
