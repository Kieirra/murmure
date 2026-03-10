import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { CURRENT_MURMURE_FORMAT_VERSION } from '../../constants';
import {
    CategoryKey,
    ImportState,
    ImportStrategy,
    MurmureConfigFile,
    ExportedCategories,
} from '../../types';
import { FormattingSettings } from '@/features/settings/formatting-rules/types';
import { LLMConnectSettings } from '@/features/llm-connect/hooks/use-llm-connect';
import { CATEGORY_DEFINITIONS } from '../../constants';

const isValidConfigFile = (data: unknown): data is MurmureConfigFile => {
    if (typeof data !== 'object' || data == null) {
        return false;
    }
    const obj = data as Record<string, unknown>;
    return (
        typeof obj.version === 'number' &&
        typeof obj.app_version === 'string' &&
        typeof obj.exported_at === 'string' &&
        typeof obj.categories === 'object' &&
        obj.categories != null
    );
};

export const useImport = () => {
    const [state, setState] = useState<ImportState>('idle');
    const [configData, setConfigData] = useState<MurmureConfigFile | null>(
        null
    );
    const [fileName, setFileName] = useState('');
    const [errorMessage, setErrorMessage] = useState('');
    const { t } = useTranslation();

    const isImporting = state === 'importing';

    const reset = useCallback(() => {
        setState('idle');
        setConfigData(null);
        setFileName('');
        setErrorMessage('');
    }, []);

    const loadFile = useCallback(
        async (filePath: string) => {
            try {
                const pathLower = filePath.toLowerCase();
                if (!pathLower.endsWith('.murmure')) {
                    setState('file_error');
                    setErrorMessage(
                        t(
                            'Invalid file. Please select a valid .murmure file.'
                        )
                    );
                    return;
                }

                const content = await invoke<string>('read_config_file', {
                    filePath,
                });

                let parsed: unknown;
                try {
                    parsed = JSON.parse(content);
                } catch {
                    setState('file_error');
                    setErrorMessage(
                        t(
                            'Invalid file. Please select a valid .murmure file.'
                        )
                    );
                    return;
                }

                if (!isValidConfigFile(parsed)) {
                    setState('file_error');
                    setErrorMessage(
                        t(
                            'Invalid file. Please select a valid .murmure file.'
                        )
                    );
                    return;
                }

                if (parsed.version > CURRENT_MURMURE_FORMAT_VERSION) {
                    setState('version_error');
                    setErrorMessage(
                        t(
                            'This file was created with a newer version of Murmure'
                        ) +
                            ` (v${parsed.version}). ` +
                            t('Your version supports files up to') +
                            ` v${CURRENT_MURMURE_FORMAT_VERSION}.`
                    );
                    return;
                }

                const parts = filePath.split(/[\\/]/);
                setFileName(parts[parts.length - 1]);
                setConfigData(parsed);
                setState('previewing');
            } catch {
                setState('file_error');
                setErrorMessage(
                    t('Invalid file. Please select a valid .murmure file.')
                );
            }
        },
        [t]
    );

    const browseFile = useCallback(async () => {
        try {
            const file = await open({
                directory: false,
                multiple: false,
                title: t('Select a .murmure file'),
                filters: [
                    {
                        name: 'Murmure Config',
                        extensions: ['murmure'],
                    },
                ],
            });

            if (file == null) {
                return;
            }

            await loadFile(file as string);
        } catch {
            setState('file_error');
            setErrorMessage(
                t('Invalid file. Please select a valid .murmure file.')
            );
        }
    }, [loadFile, t]);

    const applyImport = useCallback(
        async (
            selectedCategories: CategoryKey[],
            strategies: Partial<Record<CategoryKey, ImportStrategy>>
        ) => {
            if (configData == null) {
                return;
            }

            setState('importing');

            const categories = configData.categories;
            const imported: string[] = [];
            const failed: string[] = [];

            for (const categoryKey of selectedCategories) {
                const categoryData =
                    categories[categoryKey as keyof ExportedCategories];
                if (categoryData == null) {
                    continue;
                }

                const definition = CATEGORY_DEFINITIONS.find(
                    (d) => d.key === categoryKey
                );
                const label = definition?.label ?? categoryKey;

                try {
                    switch (categoryKey) {
                        case 'settings':
                            await applySettings(categories);
                            break;
                        case 'shortcuts':
                            await applyShortcuts(categories);
                            break;
                        case 'formatting_rules':
                            await applyFormattingRules(
                                categories,
                                strategies.formatting_rules ?? 'replace'
                            );
                            break;
                        case 'llm_connect':
                            await applyLlmConnect(categories);
                            break;
                        case 'dictionary':
                            await applyDictionary(
                                categories,
                                strategies.dictionary ?? 'replace'
                            );
                            break;
                    }
                    imported.push(label);
                } catch (error) {
                    failed.push(`${label} (${String(error)})`);
                }
            }

            if (failed.length > 0 && imported.length > 0) {
                setState('partial_error');
                toast.warning(
                    t('Import partially completed.') +
                        ' ' +
                        t('Updated') +
                        ': ' +
                        imported.join(', ') +
                        '. ' +
                        t('Failed') +
                        ': ' +
                        failed.join(', ') +
                        '.'
                );
            } else if (failed.length > 0) {
                setState('partial_error');
                toast.error(
                    t('Import failed.') +
                        ' ' +
                        t('Failed') +
                        ': ' +
                        failed.join(', ') +
                        '.'
                );
            } else {
                setState('done');
                toast.success(
                    t('Configuration imported successfully.') +
                        ' ' +
                        t('Updated') +
                        ': ' +
                        imported.join(', ') +
                        '.',
                    { autoClose: 3000 }
                );
            }

            // Reset to idle after success only (not on partial errors)
            if (failed.length === 0) {
                setTimeout(() => {
                    reset();
                }, 500);
            }
        },
        [configData, reset, t]
    );

    return {
        state,
        configData,
        fileName,
        errorMessage,
        isImporting,
        loadFile,
        browseFile,
        applyImport,
        reset,
    };
};

const applySettings = async (categories: ExportedCategories): Promise<void> => {
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

const applyShortcuts = async (categories: ExportedCategories): Promise<void> => {
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

const applyFormattingRules = async (
    categories: ExportedCategories,
    strategy: ImportStrategy
): Promise<void> => {
    const imported = categories.formatting_rules;
    if (imported == null) {
        return;
    }

    if (strategy === 'merge') {
        const current = await invoke<FormattingSettings>(
            'get_formatting_settings'
        );
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
            built_in: imported.built_in,
            rules: mergedRules,
        };
        await invoke('set_formatting_settings', { settings: merged });
    } else {
        await invoke('set_formatting_settings', { settings: imported });
    }
};

const applyLlmConnect = async (
    categories: ExportedCategories
): Promise<void> => {
    const imported = categories.llm_connect;
    if (imported == null) {
        return;
    }

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
};

const applyDictionary = async (
    categories: ExportedCategories,
    strategy: ImportStrategy
): Promise<void> => {
    const imported = categories.dictionary;
    if (imported == null) {
        return;
    }

    if (strategy === 'merge') {
        const current = await invoke<Record<string, string[]>>(
            'get_dictionary_with_languages'
        );
        const existingLower = new Set(
            Object.keys(current).map((w) => w.toLowerCase())
        );
        const merged: Record<string, string[]> = { ...current };

        for (const [word, languages] of Object.entries(imported)) {
            if (!existingLower.has(word.toLowerCase())) {
                merged[word] = languages;
            }
        }

        await invoke('set_dictionary_with_languages', { dictionary: merged });
    } else {
        await invoke('set_dictionary_with_languages', { dictionary: imported });
    }
};
