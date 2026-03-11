import { FormattingRule } from '@/features/settings/formatting-rules/types';
import { LLMConnectSettings } from '@/features/llm-connect/hooks/use-llm-connect';
import {
    CategoryKey,
    CategoryDefinition,
    CategorySelection,
    DynamicSubItemsRenderer,
    ExportedCategories,
    ExportedSystemSettings,
    ExportedShortcuts,
    ExportedLlmConnect,
    AppSettings,
} from './import-export.types';
import { CATEGORY_DEFINITIONS, SUB_ITEM_KEY } from './import-export.constants';

/**
 * Build a `Record<string, boolean>` from a list of keys, defaulting every
 * entry to `true`.  Static keys (e.g. "built_in", "connection") can be
 * prepended via `staticKeys`.
 */
export const buildSubItems = (keys: string[], staticKeys: string[] = []): Record<string, boolean> =>
    Object.fromEntries([...staticKeys, ...keys].map((k) => [k, true]));

export const formatRuleLabel = (rule: FormattingRule): string => {
    const trigger = rule.trigger || '(empty)';
    const replacement =
        rule.replacement.length > 20
            ? `${rule.replacement.replaceAll('\n', '\u21B5').substring(0, 20)}...`
            : rule.replacement.replaceAll('\n', '\u21B5') || '(delete)';
    return `${trigger} \u2192 ${replacement}`;
};

export const hasSubItems = (def: CategoryDefinition, selection: CategorySelection): boolean => {
    const subKeys = Object.keys(selection[def.key]?.subItems ?? {});
    return subKeys.length > 0;
};

export const isCategoryOn = (categoryKey: string, selection: CategorySelection): boolean => {
    const cat = selection[categoryKey];
    if (cat == null) {
        return false;
    }
    if (!cat.selected) {
        return false;
    }
    const subValues = Object.values(cat.subItems);
    if (subValues.length === 0) {
        return cat.selected;
    }
    return subValues.some((v) => v);
};

export const getCounterValue = (
    def: CategoryDefinition,
    counters?: Partial<Record<CategoryKey, number>>
): number | null => {
    if (counters == null) {
        return null;
    }
    const count = counters[def.key];
    if (count == null) {
        return null;
    }
    return count;
};

export const buildCategoriesWithDynamic = (
    definitions: CategoryDefinition[],
    renderers: Partial<Record<CategoryKey, DynamicSubItemsRenderer>>
): CategoryDefinition[] => {
    return definitions.map((def) => {
        const renderer = renderers[def.key];
        if (renderer != null) {
            return { ...def, dynamicSubItems: renderer };
        }
        return def;
    });
};

export const extractSystemSettings = (all: AppSettings): ExportedSystemSettings => {
    return {
        record_mode: all.record_mode,
        overlay_mode: all.overlay_mode,
        overlay_position: all.overlay_position,
        api_enabled: all.api_enabled,
        api_port: all.api_port,
        copy_to_clipboard: all.copy_to_clipboard,
        paste_method: all.paste_method,
        persist_history: all.persist_history,
        language: all.language,
        sound_enabled: all.sound_enabled,
        log_level: all.log_level,
    };
};

export const extractShortcuts = (all: AppSettings): ExportedShortcuts => {
    return {
        record_shortcut: all.record_shortcut,
        last_transcript_shortcut: all.last_transcript_shortcut,
        llm_record_shortcut: all.llm_record_shortcut,
        command_shortcut: all.command_shortcut,
        llm_mode_1_shortcut: all.llm_mode_1_shortcut,
        llm_mode_2_shortcut: all.llm_mode_2_shortcut,
        llm_mode_3_shortcut: all.llm_mode_3_shortcut,
        llm_mode_4_shortcut: all.llm_mode_4_shortcut,
        cancel_shortcut: all.cancel_shortcut,
    };
};

export const extractLlmConnect = (raw: LLMConnectSettings): ExportedLlmConnect => {
    return {
        url: raw.url,
        remote_url: raw.remote_url,
        remote_privacy_acknowledged: raw.remote_privacy_acknowledged,
        onboarding_completed: raw.onboarding_completed,
        modes: raw.modes,
        active_mode_index: raw.active_mode_index,
    };
};

export const buildImportSelection = (categories: ExportedCategories) => {
    const selection: Record<string, { selected: boolean; subItems: Record<string, boolean> }> = {};

    for (const def of CATEGORY_DEFINITIONS) {
        const isPresent = categories[def.key as keyof ExportedCategories] != null;
        const subItems: Record<string, boolean> = {};

        if (def.key === 'formatting_rules' && categories.formatting_rules != null) {
            subItems['built_in'] = true;
            for (const rule of categories.formatting_rules.rules) {
                subItems[SUB_ITEM_KEY.rule(rule.id)] = true;
            }
        } else if (def.key === 'llm_connect' && categories.llm_connect != null) {
            subItems['connection'] = true;
            for (let i = 0; i < categories.llm_connect.modes.length; i++) {
                subItems[SUB_ITEM_KEY.mode(i)] = true;
            }
        } else if (def.key === 'dictionary' && categories.dictionary != null) {
            for (const word of Object.keys(categories.dictionary)) {
                subItems[SUB_ITEM_KEY.word(word)] = true;
            }
        } else {
            for (const sub of def.subItems) {
                subItems[sub.key] = isPresent;
            }
        }

        selection[def.key] = { selected: isPresent, subItems };
    }

    return selection;
};

export const getCounters = (categories: ExportedCategories): Partial<Record<CategoryKey, number>> => {
    const counters: Partial<Record<CategoryKey, number>> = {};

    if (categories.formatting_rules != null) {
        counters.formatting_rules = categories.formatting_rules.rules.length;
    }
    if (categories.dictionary != null) {
        counters.dictionary = Object.keys(categories.dictionary).length;
    }
    if (categories.llm_connect != null) {
        counters.llm_connect = categories.llm_connect.modes.length;
    }

    return counters;
};
