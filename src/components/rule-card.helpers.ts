export const MAX_RULE_NAME_LENGTH = 60;
export const RULE_NAME_COUNTER_THRESHOLD = 48;

export const normalizeRuleName = (input: string): string | undefined => {
    const trimmed = input.trim();
    return trimmed.length === 0 ? undefined : trimmed;
};
