export type MatchMode = 'smart' | 'exact' | 'regex';

export interface FormattingRule {
    id: string;
    trigger: string;
    replacement: string;
    enabled: boolean;
    match_mode: MatchMode;
}

export interface BuiltInOptions {
    space_before_punctuation: boolean;
    trailing_space: boolean;
    convert_text_numbers: boolean;
    text_numbers_language: string;
    text_numbers_threshold: number;
}

export interface FormattingSettings {
    built_in: BuiltInOptions;
    rules: FormattingRule[];
}

export const defaultFormattingSettings: FormattingSettings = {
    built_in: {
        space_before_punctuation: false,
        trailing_space: false,
        convert_text_numbers: false,
        text_numbers_language: 'en',
        text_numbers_threshold: 2.0,
    },
    rules: [],
};

export function migrateRule(raw: Record<string, unknown>): FormattingRule {
    if (typeof raw.match_mode === 'string') {
        return raw as unknown as FormattingRule;
    }
    return {
        id: raw.id as string,
        trigger: raw.trigger as string,
        replacement: raw.replacement as string,
        enabled: raw.enabled as boolean,
        match_mode: raw.exact_match === true ? 'exact' : 'smart',
    };
}
