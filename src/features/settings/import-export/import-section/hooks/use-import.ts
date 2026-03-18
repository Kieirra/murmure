import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { CURRENT_MURMURE_FORMAT_VERSION, CATEGORY_DEFINITIONS } from '../../import-export.constants';
import {
    CategoryKey,
    ImportState,
    ImportStrategy,
    MurmureExportData,
    ExportedCategories,
} from '../../import-export.types';
import { applySingleCategory } from '../import-section.helpers';

const isValidConfigFile = (data: unknown): data is MurmureExportData => {
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
    const [configData, setConfigData] = useState<MurmureExportData | null>(null);
    const [fileName, setFileName] = useState('');
    const [errorMessage, setErrorMessage] = useState('');
    const { t } = useTranslation();

    const isImporting = state === 'importing';

    const reset = () => {
        setState('idle');
        setConfigData(null);
        setFileName('');
        setErrorMessage('');
    };

    const loadFile = async (filePath: string) => {
        try {
            const pathLower = filePath.toLowerCase();
            if (!pathLower.endsWith('.murmure')) {
                setState('file_error');
                setErrorMessage(t('Invalid file. Please select a valid .murmure file.'));
                return;
            }

            const content = await invoke<string>('read_murmure_file', {
                filePath,
            });

            let parsed: unknown;
            try {
                parsed = JSON.parse(content);
            } catch {
                setState('file_error');
                setErrorMessage(t('Invalid file. Please select a valid .murmure file.'));
                return;
            }

            if (!isValidConfigFile(parsed)) {
                setState('file_error');
                setErrorMessage(t('Invalid file. Please select a valid .murmure file.'));
                return;
            }

            if (parsed.version > CURRENT_MURMURE_FORMAT_VERSION) {
                setState('version_error');
                setErrorMessage(
                    t(
                        'This file was created with a newer version of Murmure (v{{fileVersion}}). Your version supports files up to v{{supportedVersion}}.',
                        {
                            fileVersion: parsed.version,
                            supportedVersion: CURRENT_MURMURE_FORMAT_VERSION,
                        }
                    )
                );
                return;
            }

            // Retrocompatibility: convert dictionary from string[] to Record<string, string[]>
            if (Array.isArray(parsed.categories.dictionary)) {
                const legacyWords = parsed.categories.dictionary as string[];
                const normalized: Record<string, string[]> = {};
                for (const word of legacyWords) {
                    normalized[word] = ['english', 'french'];
                }
                parsed.categories.dictionary = normalized;
            }

            const parts = filePath.split(/[\\/]/);
            setFileName(parts[parts.length - 1]);
            setConfigData(parsed);
            setState('previewing');
        } catch {
            setState('file_error');
            setErrorMessage(t('Invalid file. Please select a valid .murmure file.'));
        }
    };

    const browseFile = async () => {
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
            setErrorMessage(t('Invalid file. Please select a valid .murmure file.'));
        }
    };

    const applyImport = async (
        filteredCategories: ExportedCategories,
        selectedCategories: CategoryKey[],
        strategies: Partial<Record<CategoryKey, ImportStrategy>>
    ) => {
        setState('importing');

        const imported: string[] = [];
        const failed: string[] = [];

        for (const categoryKey of selectedCategories) {
            const categoryData = filteredCategories[categoryKey as keyof ExportedCategories];
            if (categoryData == null) {
                continue;
            }

            const definition = CATEGORY_DEFINITIONS.find((d) => d.key === categoryKey);
            const label = definition?.label ?? categoryKey;

            try {
                const skipped = await applySingleCategory(categoryKey, filteredCategories, strategies);
                if (skipped > 0) {
                    toast.warning(
                        t('{{count}} mode(s) could not be imported (limit of 4 reached).', { count: skipped })
                    );
                }
                imported.push(label);
            } catch (error) {
                failed.push(`${label} (${String(error)})`);
            }
        }

        if (failed.length > 0 && imported.length > 0) {
            setState('partial_error');
            toast.warning(
                t('Import partially completed. Updated: {{updated}}. Failed: {{failed}}.', {
                    updated: imported.join(', '),
                    failed: failed.join(', '),
                })
            );
        } else if (failed.length > 0) {
            setState('partial_error');
            toast.error(t('Import failed. Failed: {{failed}}.', { failed: failed.join(', ') }));
        } else {
            setState('done');
            toast.success(
                t('Configuration imported successfully. Updated: {{updated}}.', {
                    updated: imported.join(', '),
                }),
                { autoClose: 3000 }
            );
        }

        // Reset to idle after success only (not on partial errors)
        if (failed.length === 0) {
            setTimeout(() => {
                reset();
            }, 500);
        }
    };

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
