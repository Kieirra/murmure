import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { getVersion } from '@tauri-apps/api/app';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { CURRENT_MURMURE_FORMAT_VERSION, subItemKey } from '../../constants';
import {
    CategoryKey,
    CategorySelection,
    MurmureConfigFile,
    ExportedCategories,
    ExportedSystemSettings,
    ExportedShortcuts,
    ExportedLlmConnect,
    AppSettings,
} from '../../types';
import { FormattingSettings } from '@/features/settings/formatting-rules/types';
import { LLMConnectSettings } from '@/features/llm-connect/hooks/use-llm-connect';

interface PreloadedData {
    allSettings: AppSettings | null;
}

const extractSystemSettings = (all: AppSettings): ExportedSystemSettings => {
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

const extractShortcuts = (all: AppSettings): ExportedShortcuts => {
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

const extractLlmConnect = (raw: LLMConnectSettings): ExportedLlmConnect => {
    return {
        url: raw.url,
        remote_url: raw.remote_url,
        remote_privacy_acknowledged: raw.remote_privacy_acknowledged,
        onboarding_completed: raw.onboarding_completed,
        modes: raw.modes,
        active_mode_index: raw.active_mode_index,
    };
};

export const useExport = () => {
    const [isExporting, setIsExporting] = useState(false);
    const { t } = useTranslation();

    const handleExport = async (
        selectedCategories: CategoryKey[],
        selection: CategorySelection | undefined,
        preloaded: PreloadedData
    ) => {
        if (selectedCategories.length === 0) {
            return;
        }

        setIsExporting(true);

        try {
            const [allSettings, appVersion] = await Promise.all([
                preloaded.allSettings != null
                    ? Promise.resolve(preloaded.allSettings)
                    : invoke<AppSettings>('get_all_settings'),
                getVersion(),
            ]);

            const categories: ExportedCategories = {};

            const fetchPromises: Promise<void>[] = [];

            const getSubItems = (key: CategoryKey) =>
                selection?.[key]?.subItems;

            if (selectedCategories.includes('settings')) {
                categories.settings = extractSystemSettings(allSettings);
            }

            if (selectedCategories.includes('shortcuts')) {
                categories.shortcuts = extractShortcuts(allSettings);
            }

            if (selectedCategories.includes('formatting_rules')) {
                fetchPromises.push(
                    invoke<FormattingSettings>('get_formatting_settings').then(
                        (data) => {
                            const subItems = getSubItems('formatting_rules');
                            const includeBuiltIn =
                                subItems == null || subItems['built_in'] !== false;
                            const filteredRules = data.rules.filter((rule) => {
                                if (subItems == null) {
                                    return true;
                                }
                                return subItems[subItemKey.rule(rule.id)] !== false;
                            });

                            categories.formatting_rules = {
                                built_in: includeBuiltIn
                                    ? data.built_in
                                    : ({} as FormattingSettings['built_in']),
                                rules: filteredRules,
                            };
                        }
                    )
                );
            }

            if (selectedCategories.includes('llm_connect')) {
                fetchPromises.push(
                    invoke<LLMConnectSettings>('get_llm_connect_settings').then(
                        (data) => {
                            const subItems = getSubItems('llm_connect');
                            const full = extractLlmConnect(data);
                            const includeConnection =
                                subItems == null ||
                                subItems['connection'] !== false;
                            const filteredModes = full.modes.filter(
                                (_mode, index) => {
                                    if (subItems == null) {
                                        return true;
                                    }
                                    return (
                                        subItems[subItemKey.mode(index)] !== false
                                    );
                                }
                            );

                            categories.llm_connect = {
                                ...full,
                                url: includeConnection ? full.url : '',
                                remote_url: includeConnection
                                    ? full.remote_url
                                    : '',
                                modes: filteredModes,
                            };
                        }
                    )
                );
            }

            if (selectedCategories.includes('dictionary')) {
                fetchPromises.push(
                    invoke<Record<string, string[]>>(
                        'get_dictionary_with_languages'
                    ).then((data) => {
                        const subItems = getSubItems('dictionary');
                        if (subItems == null) {
                            categories.dictionary = data;
                        } else {
                            const filtered: Record<string, string[]> = {};
                            for (const [word, languages] of Object.entries(
                                data
                            )) {
                                if (subItems[subItemKey.word(word)] !== false) {
                                    filtered[word] = languages;
                                }
                            }
                            categories.dictionary = filtered;
                        }
                    })
                );
            }

            await Promise.all(fetchPromises);

            const configFile: MurmureConfigFile = {
                version: CURRENT_MURMURE_FORMAT_VERSION,
                app_version: appVersion,
                exported_at: new Date().toISOString(),
                categories,
            };

            const content = JSON.stringify(configFile, null, 2);

            const today = new Date().toISOString().slice(0, 10);
            const filePath = await save({
                title: t('Export Configuration'),
                filters: [
                    {
                        name: 'Murmure Config',
                        extensions: ['murmure'],
                    },
                ],
                defaultPath: `murmure-config-${today}.murmure`,
            });

            if (filePath == null) {
                setIsExporting(false);
                return;
            }

            await invoke('write_config_file', { filePath, content });

            toast.success(
                t('Configuration exported to {{path}}.', { path: filePath }),
                { autoClose: 3000 }
            );
        } catch (error) {
            toast.error(
                t('Failed to export configuration.') + ': ' + String(error)
            );
        } finally {
            setIsExporting(false);
        }
    };

    return {
        isExporting,
        handleExport,
    };
};
