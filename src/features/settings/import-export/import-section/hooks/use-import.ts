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
import type { ImportProgressStep } from '../import-progress-modal/import-progress-modal';

const CATEGORY_LABEL_KEYS: Record<CategoryKey, string> = {
    formatting_rules: 'Importing formatting rules...',
    dictionary: 'Importing dictionary...',
    shortcuts: 'Importing shortcuts...',
    llm_connect: 'Importing LLM Connect settings...',
    settings: 'Importing system settings...',
};

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
    const [importSteps, setImportSteps] = useState<ImportProgressStep[]>([]);
    const [isImportComplete, setIsImportComplete] = useState(false);
    const [hasImportError, setHasImportError] = useState(false);
    const { t } = useTranslation();

    const isImporting = state === 'importing';

    const reset = () => {
        setState('idle');
        setConfigData(null);
        setFileName('');
        setErrorMessage('');
        setImportSteps([]);
        setIsImportComplete(false);
        setHasImportError(false);
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

            await loadFile(file);
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
        setIsImportComplete(false);
        setHasImportError(false);

        const steps: ImportProgressStep[] = selectedCategories
            .filter((key) => filteredCategories[key as keyof ExportedCategories] != null)
            .map((key) => ({
                label: t(CATEGORY_LABEL_KEYS[key]),
                status: 'pending' as const,
            }));
        setImportSteps([...steps]);

        const imported: string[] = [];
        const failed: string[] = [];

        let stepIndex = 0;
        for (const categoryKey of selectedCategories) {
            const categoryData = filteredCategories[categoryKey as keyof ExportedCategories];
            if (categoryData == null) {
                continue;
            }

            const definition = CATEGORY_DEFINITIONS.find((d) => d.key === categoryKey);
            const label = definition?.label ?? categoryKey;

            steps[stepIndex] = { ...steps[stepIndex], status: 'in_progress' };
            setImportSteps([...steps]);

            // Intentional 400ms delay: "labor illusion" so the user perceives each import step
            await new Promise((r) => setTimeout(r, 400));

            try {
                const skipped = await applySingleCategory(categoryKey, filteredCategories, strategies);
                if (skipped > 0) {
                    toast.warning(
                        t('{{count}} mode(s) could not be imported (limit of 4 reached).', { count: skipped })
                    );
                }
                imported.push(label);
                steps[stepIndex] = { ...steps[stepIndex], status: 'done' };
            } catch (error) {
                failed.push(`${label} (${String(error)})`);
                steps[stepIndex] = { ...steps[stepIndex], status: 'error' };
            }

            setImportSteps([...steps]);
            stepIndex++;
        }

        if (failed.length > 0) {
            setState('partial_error');
            setHasImportError(true);
        } else {
            setState('done');
        }

        setIsImportComplete(true);

        // Auto-complete onboarding: importing users are already advanced
        if (imported.length > 0) {
            invoke('set_onboarding_used_home_shortcut').catch(() => {});
            invoke('set_onboarding_transcribed_outside_app').catch(() => {});
            invoke('set_onboarding_added_dictionary_word').catch(() => {});
            invoke('set_onboarding_congrats_dismissed').catch(() => {});
        }
    };

    return {
        state,
        configData,
        fileName,
        errorMessage,
        isImporting,
        importSteps,
        isImportComplete,
        hasImportError,
        loadFile,
        browseFile,
        applyImport,
        reset,
    };
};
