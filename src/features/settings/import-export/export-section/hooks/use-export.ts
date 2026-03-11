import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { getVersion } from '@tauri-apps/api/app';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { CURRENT_MURMURE_FORMAT_VERSION, SUB_ITEM_KEY } from '../../import-export.constants';
import {
    CategoryKey,
    CategorySelection,
    MurmureExportData,
    ExportedCategories,
    AppSettings,
} from '../../import-export.types';
import { extractSystemSettings, extractShortcuts, extractLlmConnect } from '../../import-export.helpers';
import { FormattingSettings } from '@/features/settings/formatting-rules/types';
import { LLMConnectSettings } from '@/features/llm-connect/hooks/use-llm-connect';

interface PreloadedData {
    allSettings: AppSettings | null;
}

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

            const getSubItems = (key: CategoryKey) => selection?.[key]?.subItems;

            if (selectedCategories.includes('settings')) {
                categories.settings = extractSystemSettings(allSettings);
            }

            if (selectedCategories.includes('shortcuts')) {
                categories.shortcuts = extractShortcuts(allSettings);
            }

            if (selectedCategories.includes('formatting_rules')) {
                fetchPromises.push(
                    invoke<FormattingSettings>('get_formatting_settings').then((data) => {
                        const subItems = getSubItems('formatting_rules');
                        const includeBuiltIn = subItems == null || subItems['built_in'] !== false;
                        const filteredRules = data.rules.filter((rule) => {
                            if (subItems == null) {
                                return true;
                            }
                            return subItems[SUB_ITEM_KEY.rule(rule.id)] !== false;
                        });

                        categories.formatting_rules = {
                            built_in: includeBuiltIn ? data.built_in : ({} as FormattingSettings['built_in']),
                            rules: filteredRules,
                        };
                    })
                );
            }

            if (selectedCategories.includes('llm_connect')) {
                fetchPromises.push(
                    invoke<LLMConnectSettings>('get_llm_connect_settings').then((data) => {
                        const subItems = getSubItems('llm_connect');
                        const full = extractLlmConnect(data);
                        const includeConnection = subItems == null || subItems['connection'] !== false;
                        const filteredModes = full.modes.filter((_mode, index) => {
                            if (subItems == null) {
                                return true;
                            }
                            return subItems[SUB_ITEM_KEY.mode(index)] !== false;
                        });

                        categories.llm_connect = {
                            ...full,
                            url: includeConnection ? full.url : '',
                            remote_url: includeConnection ? full.remote_url : '',
                            modes: filteredModes,
                        };
                    })
                );
            }

            if (selectedCategories.includes('dictionary')) {
                fetchPromises.push(
                    invoke<string[]>('get_dictionary').then((data) => {
                        const subItems = getSubItems('dictionary');
                        if (subItems == null) {
                            categories.dictionary = data;
                        } else {
                            categories.dictionary = data.filter((word) => subItems[SUB_ITEM_KEY.word(word)] !== false);
                        }
                    })
                );
            }

            await Promise.all(fetchPromises);

            const configFile: MurmureExportData = {
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

            await invoke('write_murmure_file', { filePath, content });

            toast.success(t('Configuration exported to {{path}}.', { path: filePath }), { autoClose: 3000 });
        } catch (error) {
            toast.error(t('Failed to export configuration.') + ': ' + String(error));
        } finally {
            setIsExporting(false);
        }
    };

    return {
        isExporting,
        handleExport,
    };
};
