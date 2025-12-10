export interface FormattingRule {
    id: string;
    trigger: string;
    replacement: string;
    enabled: boolean;
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
        text_numbers_threshold: 0.0,
    },
    rules: [],
};
