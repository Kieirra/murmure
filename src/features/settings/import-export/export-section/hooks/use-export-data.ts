import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { CATEGORY_DEFINITIONS } from '../../import-export.constants';
import { AppSettings, CategorySelection } from '../../import-export.types';
import { FormattingSettings, FormattingRule } from '@/features/personalize/formatting-rules/types';
import { LLMConnectSettings, LLMMode } from '@/features/personalize/llm-connect/hooks/use-llm-connect';
import { buildInitialSelection } from '../export-section.helpers';

export const useExportData = () => {
    const [exportSelection, setExportSelection] = useState<CategorySelection>(() =>
        Object.fromEntries(CATEGORY_DEFINITIONS.map((def) => [def.key, { selected: true, subItems: {} }]))
    );
    const [rules, setRules] = useState<FormattingRule[]>([]);
    const [llmModes, setLlmModes] = useState<LLMMode[]>([]);
    const [dictionaryWords, setDictionaryWords] = useState<string[]>([]);
    const [allSettings, setAllSettings] = useState<AppSettings | null>(null);

    useEffect(() => {
        const loadData = async () => {
            try {
                const [formattingRules, llmSettings, dictionary, settings] = await Promise.all([
                    invoke<FormattingSettings>('get_formatting_settings'),
                    invoke<LLMConnectSettings>('get_llm_connect_settings'),
                    invoke<string[]>('get_dictionary'),
                    invoke<AppSettings>('get_all_settings'),
                ]);

                setRules(formattingRules.rules);
                setLlmModes(llmSettings.modes);
                setDictionaryWords(dictionary);
                setAllSettings(settings);
                setExportSelection(buildInitialSelection(formattingRules, llmSettings, dictionary));
            } catch {
                // Data loading is best-effort, fail silently
            }
        };

        loadData();
    }, []);

    return {
        rules,
        llmModes,
        dictionaryWords,
        allSettings,
        exportSelection,
        setExportSelection,
    };
};
