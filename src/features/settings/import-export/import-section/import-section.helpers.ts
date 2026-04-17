import { invoke } from '@tauri-apps/api/core';
import { MAX_LLM_MODES } from '../import-export.constants';
import { CategoryKey, ExportedCategories, ImportStrategy } from '../import-export.types';
import { FormattingRule, FormattingSettings } from '@/features/personalize/formatting-rules/types';
import { LLMConnectSettings } from '@/features/extensions/llm-connect/hooks/use-llm-connect';

const applySettings = async (categories: ExportedCategories): Promise<void> => {
    const settings = categories.settings;
    if (settings == null) {
        return;
    }
    await invoke('set_record_mode', { mode: settings.record_mode });
    await invoke('set_overlay_mode', { mode: settings.overlay_mode });
    await invoke('set_overlay_position', { position: settings.overlay_position });
    await invoke('set_api_enabled', { enabled: settings.api_enabled });
    await invoke('set_api_port', { port: settings.api_port });
    await invoke('set_copy_to_clipboard', { enabled: settings.copy_to_clipboard });
    await invoke('set_paste_method', { method: settings.paste_method });
    await invoke('set_persist_history', { enabled: settings.persist_history });
    await invoke('set_current_language', { lang: settings.language });
    await invoke('set_sound_enabled', { enabled: settings.sound_enabled });
    await invoke('set_log_level', { level: settings.log_level });
    await invoke('set_show_in_dock', { show: settings.show_in_dock });
    if (settings.wake_word_enabled != null) {
        await invoke('set_wake_word_enabled', { enabled: settings.wake_word_enabled });
    }
    if (settings.smartmic_enabled != null) {
        await invoke('set_smartmic_enabled', { enabled: settings.smartmic_enabled });
    }
    if (settings.streaming_preview != null) {
        await invoke('set_streaming_preview', { enabled: settings.streaming_preview });
    }
    if (settings.overlay_size != null) {
        await invoke('set_overlay_size', { size: settings.overlay_size });
    }
    if (settings.streaming_text_width != null && settings.streaming_font_size != null && settings.streaming_max_lines != null) {
        await invoke('set_streaming_text_settings', {
            textWidth: settings.streaming_text_width,
            fontSize: settings.streaming_font_size,
            maxLines: settings.streaming_max_lines,
        });
    }
};

const applyShortcuts = async (categories: ExportedCategories): Promise<void> => {
    const shortcuts = categories.shortcuts;
    if (shortcuts == null) {
        return;
    }
    // Sequential to avoid race conditions on shortcut re-registration
    await invoke('set_record_shortcut', { binding: shortcuts.record_shortcut });
    await invoke('set_last_transcript_shortcut', {
        binding: shortcuts.last_transcript_shortcut,
    });
    await invoke('set_llm_record_shortcut', {
        binding: shortcuts.llm_record_shortcut,
    });
    await invoke('set_command_shortcut', { binding: shortcuts.command_shortcut });
    await invoke('set_llm_mode_1_shortcut', {
        binding: shortcuts.llm_mode_1_shortcut,
    });
    await invoke('set_llm_mode_2_shortcut', {
        binding: shortcuts.llm_mode_2_shortcut,
    });
    await invoke('set_llm_mode_3_shortcut', {
        binding: shortcuts.llm_mode_3_shortcut,
    });
    await invoke('set_llm_mode_4_shortcut', {
        binding: shortcuts.llm_mode_4_shortcut,
    });
    await invoke('set_cancel_shortcut', { binding: shortcuts.cancel_shortcut });
};

const applyFormattingRules = async (categories: ExportedCategories, strategy: ImportStrategy): Promise<void> => {
    const imported = categories.formatting_rules;
    if (imported == null) {
        return;
    }

    const current = await invoke<FormattingSettings>('get_formatting_settings');

    let rules: FormattingRule[];
    if (strategy === 'merge') {
        const existingRuleIds = new Set(current.rules.map((r) => r.id));
        rules = [...current.rules];
        for (const rule of imported.rules) {
            if (existingRuleIds.has(rule.id)) {
                const idx = rules.findIndex((r) => r.id === rule.id);
                if (idx >= 0) {
                    rules[idx] = rule;
                }
            } else {
                rules.push(rule);
            }
        }
    } else {
        rules = imported.rules;
    }

    await invoke('set_formatting_settings', {
        settings: { built_in: imported.built_in ?? current.built_in, rules },
    });
};

const applyLlmConnect = async (categories: ExportedCategories, strategy: ImportStrategy): Promise<number> => {
    const imported = categories.llm_connect;
    if (imported == null) {
        return 0;
    }

    const current = await invoke<LLMConnectSettings>('get_llm_connect_settings');

    let modes: typeof current.modes;
    let activeIndex: number;
    let skipped = 0;

    if (strategy === 'merge') {
        const existingNames = new Set(current.modes.map((m) => m.name.toLowerCase()));
        modes = [...current.modes];
        for (const mode of imported.modes) {
            if (existingNames.has(mode.name.toLowerCase())) {
                continue;
            }
            if (modes.length >= MAX_LLM_MODES) {
                skipped++;
                continue;
            }
            modes.push(mode);
        }
        activeIndex = current.active_mode_index;
    } else {
        modes = imported.modes;
        activeIndex = imported.active_mode_index;
    }

    const settings: LLMConnectSettings = {
        url: imported.url ?? current.url,
        remote_url: imported.remote_url ?? current.remote_url,
        remote_privacy_acknowledged: imported.remote_privacy_acknowledged ?? current.remote_privacy_acknowledged,
        onboarding_completed:
            imported.modes.length > 0 ? true : (imported.onboarding_completed ?? current.onboarding_completed),
        modes,
        active_mode_index: activeIndex,
        model: '',
        prompt: '',
    };

    await invoke('set_llm_connect_settings', { settings });
    return skipped;
};

const mergeDictionaries = (
    current: Record<string, string[]>,
    imported: Record<string, string[]>
): Record<string, string[]> => {
    const existingLower = new Set(Object.keys(current).map((w) => w.toLowerCase()));
    const merged: Record<string, string[]> = { ...current };

    for (const [word, languages] of Object.entries(imported)) {
        if (!existingLower.has(word.toLowerCase())) {
            merged[word] = languages;
            continue;
        }
        const existingKey = Object.keys(merged).find((k) => k.toLowerCase() === word.toLowerCase());
        if (existingKey != null) {
            merged[existingKey] = [...new Set([...merged[existingKey], ...languages])];
        }
    }

    return merged;
};

const applyDictionary = async (categories: ExportedCategories, strategy: ImportStrategy): Promise<void> => {
    const imported = categories.dictionary;
    if (imported == null) {
        return;
    }

    if (strategy === 'merge') {
        const current = await invoke<Record<string, string[]>>('get_dictionary_with_languages');
        await invoke('set_dictionary_with_languages', { dictionary: mergeDictionaries(current, imported) });
    } else {
        await invoke('set_dictionary_with_languages', { dictionary: imported });
    }
};

export const applySingleCategory = async (
    categoryKey: CategoryKey,
    categories: ExportedCategories,
    strategies: Partial<Record<CategoryKey, ImportStrategy>>
): Promise<number> => {
    switch (categoryKey) {
        case 'settings':
            await applySettings(categories);
            return 0;
        case 'shortcuts':
            await applyShortcuts(categories);
            return 0;
        case 'formatting_rules':
            await applyFormattingRules(categories, strategies.formatting_rules ?? 'replace');
            return 0;
        case 'llm_connect':
            return applyLlmConnect(categories, strategies.llm_connect ?? 'replace');
        case 'dictionary':
            await applyDictionary(categories, strategies.dictionary ?? 'replace');
            return 0;
        default:
            return 0;
    }
};
