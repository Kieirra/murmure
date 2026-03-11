import { Settings, Keyboard, AlignLeft, Sparkles, BookText } from 'lucide-react';
import { CategoryDefinition } from './types';

export const CURRENT_MURMURE_FORMAT_VERSION = 1;

export const DICTIONARY_PREVIEW_LIMIT = 15;

export const subItemKey = {
    rule: (id: string) => `rule_${id}`,
    mode: (index: number) => `mode_${index}`,
    word: (word: string) => `word_${word}`,
} as const;

export const CATEGORY_DEFINITIONS: CategoryDefinition[] = [
    {
        key: 'settings',
        label: 'System Settings',
        icon: Settings,
        supportsMerge: false,
        subItems: [],
    },
    {
        key: 'shortcuts',
        label: 'Shortcuts',
        icon: Keyboard,
        supportsMerge: false,
        subItems: [],
    },
    {
        key: 'formatting_rules',
        label: 'Formatting Rules',
        icon: AlignLeft,
        supportsMerge: true,
        subItems: [{ key: 'built_in', label: 'Built-in Options' }],
    },
    {
        key: 'llm_connect',
        label: 'LLM Connect',
        icon: Sparkles,
        supportsMerge: true,
        subItems: [{ key: 'connection', label: 'Connection Settings' }],
    },
    {
        key: 'dictionary',
        label: 'Custom Dictionary',
        icon: BookText,
        supportsMerge: true,
        subItems: [],
    },
];
