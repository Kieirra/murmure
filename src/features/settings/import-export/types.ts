import type { ComponentType, ReactNode } from 'react';
import { FormattingSettings } from '@/features/settings/formatting-rules/types';
import { LLMMode } from '@/features/llm-connect/hooks/use-llm-connect';

export interface MurmureConfigFile {
    version: number;
    app_version: string;
    exported_at: string;
    categories: ExportedCategories;
}

export interface ExportedCategories {
    settings?: ExportedSystemSettings;
    shortcuts?: ExportedShortcuts;
    formatting_rules?: FormattingSettings;
    llm_connect?: ExportedLlmConnect;
    dictionary?: Record<string, string[]>;
}

export interface ExportedSystemSettings {
    record_mode: string;
    overlay_mode: string;
    overlay_position: string;
    api_enabled: boolean;
    api_port: number;
    copy_to_clipboard: boolean;
    paste_method: string;
    persist_history: boolean;
    language: string;
    sound_enabled: boolean;
    log_level: string;
}

export interface ExportedShortcuts {
    record_shortcut: string;
    last_transcript_shortcut: string;
    llm_record_shortcut: string;
    command_shortcut: string;
    llm_mode_1_shortcut: string;
    llm_mode_2_shortcut: string;
    llm_mode_3_shortcut: string;
    llm_mode_4_shortcut: string;
    cancel_shortcut: string;
}

export interface ExportedLlmConnect {
    url: string;
    remote_url: string;
    remote_privacy_acknowledged: boolean;
    onboarding_completed: boolean;
    modes: LLMMode[];
    active_mode_index: number;
}

export interface AppSettings {
    record_mode: string;
    overlay_mode: string;
    overlay_position: string;
    api_enabled: boolean;
    api_port: number;
    copy_to_clipboard: boolean;
    paste_method: string;
    persist_history: boolean;
    language: string;
    sound_enabled: boolean;
    log_level: string;
    record_shortcut: string;
    last_transcript_shortcut: string;
    llm_record_shortcut: string;
    command_shortcut: string;
    llm_mode_1_shortcut: string;
    llm_mode_2_shortcut: string;
    llm_mode_3_shortcut: string;
    llm_mode_4_shortcut: string;
    cancel_shortcut: string;
}

export type CategoryKey =
    | 'settings'
    | 'shortcuts'
    | 'formatting_rules'
    | 'llm_connect'
    | 'dictionary';

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
