import { invoke } from '@tauri-apps/api/core';
import { CategoryKey, ExportedCategories, ImportStrategy } from '../import-export.types';
import { FormattingSettings } from '@/features/personalize/formatting-rules/types';
import { LLMConnectSettings } from '@/features/personalize/llm-connect/hooks/use-llm-connect';

export const applySettings = async (categories: ExportedCategories): Promise<void> => {
    const s = categories.settings;
    if (s == null) {
        return;
    }
    await invoke('set_record_mode', { mode: s.record_mode });
    await invoke('set_overlay_mode', { mode: s.overlay_mode });
    await invoke('set_overlay_position', { position: s.overlay_position });
    await invoke('set_api_enabled', { enabled: s.api_enabled });
    await invoke('set_api_port', { port: s.api_port });
    await invoke('set_copy_to_clipboard', { enabled: s.copy_to_clipboard });
    await invoke('set_paste_method', { method: s.paste_method });
    await invoke('set_persist_history', { enabled: s.persist_history });
    await invoke('set_current_language', { lang: s.language });
    await invoke('set_sound_enabled', { enabled: s.sound_enabled });
    await invoke('set_log_level', { level: s.log_level });
};

export const applyShortcuts = async (categories: ExportedCategories): Promise<void> => {
    const s = categories.shortcuts;
    if (s == null) {
        return;
    }
    // Sequential to avoid race conditions on shortcut re-registration
    await invoke('set_record_shortcut', { binding: s.record_shortcut });
    await invoke('set_last_transcript_shortcut', {
        binding: s.last_transcript_shortcut,
    });
    await invoke('set_llm_record_shortcut', {
        binding: s.llm_record_shortcut,
    });
    await invoke('set_command_shortcut', { binding: s.command_shortcut });
    await invoke('set_llm_mode_1_shortcut', {
        binding: s.llm_mode_1_shortcut,
    });
    await invoke('set_llm_mode_2_shortcut', {
        binding: s.llm_mode_2_shortcut,
    });
    await invoke('set_llm_mode_3_shortcut', {
        binding: s.llm_mode_3_shortcut,
    });
    await invoke('set_llm_mode_4_shortcut', {
        binding: s.llm_mode_4_shortcut,
    });
    await invoke('set_cancel_shortcut', { binding: s.cancel_shortcut });
};

export const applyFormattingRules = async (categories: ExportedCategories, strategy: ImportStrategy): Promise<void> => {
    const imported = categories.formatting_rules;
    if (imported == null) {
        return;
    }

    if (strategy === 'merge') {
        const current = await invoke<FormattingSettings>('get_formatting_settings');
        const existingRuleIds = new Set(current.rules.map((r) => r.id));
        const mergedRules = [...current.rules];

        for (const rule of imported.rules) {
            if (existingRuleIds.has(rule.id)) {
                const idx = mergedRules.findIndex((r) => r.id === rule.id);
                if (idx >= 0) {
                    mergedRules[idx] = rule;
                }
            } else {
                mergedRules.push(rule);
            }
        }

        const merged: FormattingSettings = {
            built_in: imported.built_in ?? current.built_in,
            rules: mergedRules,
        };
        await invoke('set_formatting_settings', { settings: merged });
    } else {
        const current = await invoke<FormattingSettings>('get_formatting_settings');
        const settings: FormattingSettings = {
            built_in: imported.built_in ?? current.built_in,
            rules: imported.rules,
        };
        await invoke('set_formatting_settings', { settings });
    }
};

/**
 * Applies LLM Connect settings import.
 * Returns the number of modes skipped during merge (due to the 4-mode limit).
 */
export const applyLlmConnect = async (categories: ExportedCategories, strategy: ImportStrategy): Promise<number> => {
    const imported = categories.llm_connect;
    if (imported == null) {
        return 0;
    }

    if (strategy === 'merge') {
        const current = await invoke<LLMConnectSettings>('get_llm_connect_settings');
        const existingNames = new Set(current.modes.map((m) => m.name.toLowerCase()));
        const mergedModes = [...current.modes];
        let skipped = 0;

        for (const mode of imported.modes) {
            if (existingNames.has(mode.name.toLowerCase())) {
                continue;
            }
            if (mergedModes.length >= 4) {
                skipped++;
                continue;
            }
            mergedModes.push(mode);
        }

        const settings: LLMConnectSettings = {
            url: imported.url === '' ? current.url : imported.url,
            remote_url: imported.remote_url === '' ? current.remote_url : imported.remote_url,
            remote_privacy_acknowledged: imported.remote_privacy_acknowledged,
            onboarding_completed: imported.onboarding_completed,
            modes: mergedModes,
            active_mode_index: current.active_mode_index,
            model: '',
            prompt: '',
        };

        await invoke('set_llm_connect_settings', { settings });
        return skipped;
    } else {
        const settings: LLMConnectSettings = {
            url: imported.url,
            remote_url: imported.remote_url,
            remote_privacy_acknowledged: imported.remote_privacy_acknowledged,
            onboarding_completed: imported.onboarding_completed,
            modes: imported.modes,
            active_mode_index: imported.active_mode_index,
            model: '',
            prompt: '',
        };

        await invoke('set_llm_connect_settings', { settings });
        return 0;
    }
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

export const applyDictionary = async (categories: ExportedCategories, strategy: ImportStrategy): Promise<void> => {
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

/**
 * Applies a single category import. Returns the number of skipped LLM modes (0 for other categories).
 */
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
