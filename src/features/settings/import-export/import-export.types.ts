import type { ComponentType, ReactNode } from 'react';
import { BuiltInOptions, FormattingRule } from '@/features/personalize/formatting-rules/types';
import { LLMMode } from '@/features/personalize/llm-connect/hooks/use-llm-connect';

export type { SystemSettings, ShortcutSettings, AppSettings } from '../settings.types';
import type { SystemSettings, ShortcutSettings } from '../settings.types';

export interface MurmureExportData {
    version: number;
    app_version: string;
    exported_at: string;
    categories: ExportedCategories;
}

export interface ExportedCategories {
    settings?: SystemSettings;
    shortcuts?: ShortcutSettings;
    formatting_rules?: ExportedFormattingSettings;
    llm_connect?: ExportedLlmConnect;
    dictionary?: Record<string, string[]>;
}

export interface ExportedFormattingSettings {
    built_in?: BuiltInOptions;
    rules: FormattingRule[];
}

export interface ExportedLlmConnect {
    url: string;
    remote_url: string;
    remote_privacy_acknowledged: boolean;
    onboarding_completed: boolean;
    modes: LLMMode[];
    active_mode_index: number;
}

export type CategoryKey = 'settings' | 'shortcuts' | 'formatting_rules' | 'llm_connect' | 'dictionary';

export type ImportStrategy = 'replace' | 'merge';

export type ImportState =
    | 'idle'
    | 'previewing'
    | 'importing'
    | 'done'
    | 'partial_error'
    | 'file_error'
    | 'version_error';

export interface CategorySubItem {
    key: string;
    label: string;
}

export type DynamicSubItemsRenderer = (props: {
    selection: Record<string, boolean>;
    onToggle: (key: string, checked: boolean) => void;
    disabled?: boolean;
}) => ReactNode;

export interface CategoryDefinition {
    key: CategoryKey;
    label: string;
    icon: ComponentType<{ className?: string }>;
    subItems: CategorySubItem[];
    supportsMerge: boolean;
    dynamicSubItems?: DynamicSubItemsRenderer;
}

export interface CategorySelection {
    [categoryKey: string]: {
        selected: boolean;
        subItems: Record<string, boolean>;
    };
}
