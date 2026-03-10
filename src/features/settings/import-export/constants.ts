import {
    Settings,
    Keyboard,
    AlignLeft,
    Sparkles,
    BookText,
} from 'lucide-react';
import { CategoryDefinition } from './types';

export const CURRENT_MURMURE_FORMAT_VERSION = 1;

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
        subItems: [
            { key: 'built_in', label: 'Built-in Options' },
        ],
    },
    {
        key: 'llm_connect',
        label: 'LLM Connect',
        icon: Sparkles,
        supportsMerge: false,
        subItems: [
            { key: 'connection', label: 'Connection Settings' },
        ],
    },
    {
        key: 'dictionary',
        label: 'Custom Dictionary',
        icon: BookText,
        supportsMerge: true,
        subItems: [],
    },
];
