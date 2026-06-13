const MODIFIER_ORDER = ['win', 'ctrl', 'alt', 'shift'];

export const sortBindingKeys = (keys: string[]): string[] =>
    [...keys].sort((a, b) => {
        const aIdx = MODIFIER_ORDER.indexOf(a);
        const bIdx = MODIFIER_ORDER.indexOf(b);
        if (aIdx !== -1 && bIdx !== -1) return aIdx - bIdx;
        if (aIdx !== -1) return -1;
        if (bIdx !== -1) return 1;
        return a.localeCompare(b);
    });

export const normalizeBinding = (binding: string): string => {
    if (binding.length === 0) return '';
    const keys = binding
        .split('+')
        .map((key) => key.trim().toLowerCase())
        .filter((key) => key.length > 0);
    return sortBindingKeys(keys).join('+');
};

export interface ExistingShortcut {
    name: string;
    value: string;
}

export const findConflict = (binding: string, existingShortcuts: ExistingShortcut[]): string | null => {
    const normalized = normalizeBinding(binding);
    if (normalized.length === 0) return null;
    const match = existingShortcuts.find((entry) => normalizeBinding(entry.value) === normalized);
    return match ? match.name : null;
};
