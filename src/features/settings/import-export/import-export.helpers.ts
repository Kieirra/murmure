import { FormattingRule } from '@/features/personalize/formatting-rules/types';
import { LLMConnectSettings } from '@/features/extensions/llm-connect/hooks/use-llm-connect';
import {
    CategoryKey,
    CategoryDefinition,
    CategorySelection,
    DynamicSubItemsRenderer,
    ExportedCategories,
    ExportedSystemSettings,
    ShortcutSettings,
    VoiceModeSettings,
    SmartMicSettings,
    ExportedLlmConnect,
    AppSettings,
} from './import-export.types';
import { CATEGORY_DEFINITIONS, SUB_ITEM_KEY } from './import-export.constants';

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
    if (!cat?.selected) {
        return false;
    }
    const subValues = Object.values(cat.subItems);
    return subValues.length === 0 || subValues.some(Boolean);
};

export const getCounterValue = (
    def: CategoryDefinition,
    counters?: Partial<Record<CategoryKey, number>>
): number | null => {
    return counters?.[def.key] ?? null;
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
        show_in_dock: all.show_in_dock,
        streaming_preview: all.streaming_preview,
        overlay_size: all.overlay_size,
        streaming_text_width: all.streaming_text_width,
        streaming_font_size: all.streaming_font_size,
        streaming_max_lines: all.streaming_max_lines,
    };
};

export const extractVoiceMode = (all: AppSettings): VoiceModeSettings => {
    return {
        wake_word_enabled: all.wake_word_enabled,
        wake_word_record: all.wake_word_record,
        wake_word_command: all.wake_word_command,
        wake_word_cancel: all.wake_word_cancel,
        wake_word_validate: all.wake_word_validate,
        wake_word_submit: all.wake_word_submit,
        auto_enter_after_wake_word: all.auto_enter_after_wake_word,
        silence_timeout_ms: all.silence_timeout_ms,
    };
};

export const extractSmartMic = (all: AppSettings): SmartMicSettings => {
    return {
        smartmic_enabled: all.smartmic_enabled,
        smartmic_port: all.smartmic_port,
        smartmic_relay_enabled: all.smartmic_relay_enabled,
        smartmic_relay_url: all.smartmic_relay_url,
        smartmic_machine_id_enabled: all.smartmic_machine_id_enabled,
        smartmic_machine_id: all.smartmic_machine_id,
        smartmic_token_ttl_hours: all.smartmic_token_ttl_hours,
        smartmic_bind_address: all.smartmic_bind_address,
    };
};

export const extractShortcuts = (all: AppSettings): ShortcutSettings => {
    return {
        record_shortcut: all.record_shortcut,
        last_transcript_shortcut: all.last_transcript_shortcut,
        command_shortcut: all.command_shortcut,
        llm_mode_1_shortcut: all.llm_mode_1_shortcut,
        llm_mode_2_shortcut: all.llm_mode_2_shortcut,
        llm_mode_3_shortcut: all.llm_mode_3_shortcut,
        llm_mode_4_shortcut: all.llm_mode_4_shortcut,
        voice_mode_toggle_shortcut: all.voice_mode_toggle_shortcut,
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

const buildCategorySubItems = (def: CategoryDefinition, categories: ExportedCategories): Record<string, boolean> => {
    if (def.key === 'formatting_rules' && categories.formatting_rules != null) {
        return buildSubItems(
            categories.formatting_rules.rules.map((r) => SUB_ITEM_KEY.rule(r.id)),
            ['built_in']
        );
    }
    if (def.key === 'llm_connect' && categories.llm_connect != null) {
        return buildSubItems(
            categories.llm_connect.modes.map((_, i) => SUB_ITEM_KEY.mode(i)),
            ['connection']
        );
    }
    if (def.key === 'dictionary' && categories.dictionary != null) {
        return buildSubItems(Object.keys(categories.dictionary).map((w) => SUB_ITEM_KEY.word(w)));
    }
    const isPresent = categories[def.key as keyof ExportedCategories] != null;
    return Object.fromEntries(def.subItems.map((sub) => [sub.key, isPresent]));
};

export const buildImportSelection = (categories: ExportedCategories): CategorySelection => {
    const selection: CategorySelection = {};

    for (const def of CATEGORY_DEFINITIONS) {
        const isPresent = categories[def.key as keyof ExportedCategories] != null;
        selection[def.key] = { selected: isPresent, subItems: buildCategorySubItems(def, categories) };
    }

    return selection;
};

export const buildFilteredCategories = (
    categories: ExportedCategories,
    selection: CategorySelection
): ExportedCategories => {
    const filtered: ExportedCategories = {};
    if (selection.settings?.selected && categories.settings != null) {
        filtered.settings = categories.settings;
    }
    if (selection.shortcuts?.selected && categories.shortcuts != null) {
        filtered.shortcuts = categories.shortcuts;
    }
    if (selection.voice_mode?.selected && categories.voice_mode != null) {
        filtered.voice_mode = categories.voice_mode;
    }
    if (selection.smartmic?.selected && categories.smartmic != null) {
        filtered.smartmic = categories.smartmic;
    }
    if (selection.formatting_rules?.selected && categories.formatting_rules != null) {
        const subItems = selection.formatting_rules.subItems;
        filtered.formatting_rules = {
            built_in: subItems.built_in ? categories.formatting_rules.built_in : undefined,
            rules: categories.formatting_rules.rules.filter((rule) => subItems[SUB_ITEM_KEY.rule(rule.id)] === true),
        };
    }
    if (selection.llm_connect?.selected && categories.llm_connect != null) {
        const subItems = selection.llm_connect.subItems;
        const includeConnection = subItems.connection === true;
        const filteredModes = categories.llm_connect.modes.filter(
            (_, index) => subItems[SUB_ITEM_KEY.mode(index)] === true
        );

        const newActiveIndex = Math.max(
            0,
            filteredModes.findIndex(
                (m) => m === categories.llm_connect!.modes[categories.llm_connect!.active_mode_index]
            )
        );

        filtered.llm_connect = {
            url: includeConnection ? categories.llm_connect.url : undefined,
            remote_url: includeConnection ? categories.llm_connect.remote_url : undefined,
            remote_privacy_acknowledged: includeConnection
                ? categories.llm_connect.remote_privacy_acknowledged
                : undefined,
            onboarding_completed: includeConnection ? categories.llm_connect.onboarding_completed : undefined,
            modes: filteredModes,
            active_mode_index: filteredModes.length > 0 ? newActiveIndex : 0,
        };
    }
    if (selection.dictionary?.selected && categories.dictionary != null) {
        const subItems = selection.dictionary.subItems;
        const filteredDict: Record<string, string[]> = {};
        for (const [word, languages] of Object.entries(categories.dictionary)) {
            if (subItems[SUB_ITEM_KEY.word(word)] === true) {
                filteredDict[word] = languages;
            }
        }
        filtered.dictionary = filteredDict;
    }

    return filtered;
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
