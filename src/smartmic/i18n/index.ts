import { STRINGS, type Lang, type StringKey } from './strings';

export type { Lang, StringKey } from './strings';

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
        // Persist a hardcoded literal (not the tainted URL value) to rule out
        // any storage poisoning even in case of whitelist bypass.
        if (urlLang === 'en') {
            localStorage.setItem(STORAGE_KEY, 'en');
            return 'en';
        }
        if (urlLang === 'fr') {
            localStorage.setItem(STORAGE_KEY, 'fr');
            return 'fr';
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
    return template.replaceAll(/\{(\w+)\}/g, (match, key: string) => {
        const value = params[key];
        return value === undefined ? match : String(value);
    });
};

// Language is read once per page load and never changes during the session,
// so the dict and `t` are module-level constants.
export const lang: Lang = getLang();
const DICT = STRINGS[lang];
export const t = (key: StringKey, params?: Record<string, string | number>): string =>
    format(DICT[key], params);

export type TranslateFn = typeof t;
