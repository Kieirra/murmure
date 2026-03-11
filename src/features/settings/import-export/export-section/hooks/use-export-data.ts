import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { buildSubItems } from '../../import-export.helpers';
import { CATEGORY_DEFINITIONS, SUB_ITEM_KEY } from '../../import-export.constants';
import { AppSettings, CategorySelection } from '../../import-export.types';
import { FormattingSettings, FormattingRule } from '@/features/settings/formatting-rules/types';
import { LLMConnectSettings, LLMMode } from '@/features/llm-connect/hooks/use-llm-connect';

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

                setExportSelection(() => ({
                    settings: { selected: true, subItems: {} },
                    shortcuts: { selected: true, subItems: {} },
                    formatting_rules: {
                        selected: true,
                        subItems: buildSubItems(
                            formattingRules.rules.map((r) => SUB_ITEM_KEY.rule(r.id)),
                            ['built_in']
                        ),
                    },
                    llm_connect: {
                        selected: true,
                        subItems: buildSubItems(
                            llmSettings.modes.map((_, i) => SUB_ITEM_KEY.mode(i)),
                            ['connection']
                        ),
                    },
                    dictionary: {
                        selected: true,
                        subItems: buildSubItems(dictionary.map((w) => SUB_ITEM_KEY.word(w))),
                    },
                }));
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
