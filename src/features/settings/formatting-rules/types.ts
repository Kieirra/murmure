export interface FormattingRule {
    id: string;
    trigger: string;
    replacement: string;
    enabled: boolean;
}

export interface BuiltInOptions {
    space_before_punctuation: boolean;
    trailing_space: boolean;
}

export interface FormattingSettings {
    built_in: BuiltInOptions;
    rules: FormattingRule[];
}

export const defaultFormattingSettings: FormattingSettings = {
    built_in: {
        space_before_punctuation: false,
        trailing_space: false,
    },
    rules: [],
};
