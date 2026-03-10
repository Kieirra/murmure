import { useState, useEffect, useMemo } from 'react';
import { Info, Loader2 } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { Page } from '@/components/page';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { CategoryTree } from '../components/category-tree';
import { FormattingRulesSubItems } from '../components/formatting-rules-sub-items';
import { LlmConnectSubItems } from '../components/llm-connect-sub-items';
import { DictionarySubItems } from '../components/dictionary-sub-items';
import { useExport } from './hooks/use-export';
import { CATEGORY_DEFINITIONS, subItemKey } from '../constants';
import { CategoryDefinition, AppSettings } from '../types';
import { buildCategoriesWithDynamic } from '../helpers';
import { FormattingSettings, FormattingRule } from '@/features/settings/formatting-rules/types';
import { LLMConnectSettings, LLMMode } from '@/features/llm-connect/hooks/use-llm-connect';

const buildInitialSelection = (
    definitions: CategoryDefinition[]
) => {
    const selection: Record<
        string,
        { selected: boolean; subItems: Record<string, boolean> }
    > = {};
    for (const def of definitions) {
        const subItems: Record<string, boolean> = {};
        for (const sub of def.subItems) {
            subItems[sub.key] = true;
        }
        selection[def.key] = { selected: true, subItems };
    }
    return selection;
}

export const ExportSection = () => {
    const [selection, setSelection] = useState(() =>
        buildInitialSelection(CATEGORY_DEFINITIONS)
    );
    const [rules, setRules] = useState<FormattingRule[]>([]);
    const [llmModes, setLlmModes] = useState<LLMMode[]>([]);
    const [dictionaryWords, setDictionaryWords] = useState<string[]>([]);
    const [allSettings, setAllSettings] = useState<AppSettings | null>(null);
    const [showAllWords, setShowAllWords] = useState(false);
    const { isExporting, handleExport } = useExport();
    const { t } = useTranslation();

    const counters = useMemo(() => ({
        formatting_rules: rules.length,
        llm_connect: llmModes.length,
        dictionary: dictionaryWords.length,
    }), [rules, llmModes, dictionaryWords]);

    useEffect(() => {
        const loadData = async () => {
            try {
                const [formattingRules, llmSettings, dictionary, settings] =
                    await Promise.all([
                        invoke<FormattingSettings>('get_formatting_settings'),
                        invoke<LLMConnectSettings>(
                            'get_llm_connect_settings'
                        ),
                        invoke<string[]>('get_dictionary'),
                        invoke<AppSettings>('get_all_settings'),
                    ]);

                setRules(formattingRules.rules);
                setLlmModes(llmSettings.modes);
                setDictionaryWords(dictionary);
                setAllSettings(settings);

                // Build dynamic sub-items for selection
                setSelection((prev) => {
                    const next = { ...prev };

                    // Add rule sub-items
                    const ruleSubItems: Record<string, boolean> = {
                        built_in: prev.formatting_rules?.subItems?.built_in ?? true,
                    };
                    for (const rule of formattingRules.rules) {
                        const key = subItemKey.rule(rule.id);
                        ruleSubItems[key] =
                            prev.formatting_rules?.subItems?.[key] ?? true;
                    }
                    next.formatting_rules = {
                        selected: prev.formatting_rules?.selected ?? true,
                        subItems: ruleSubItems,
                    };

                    // Add mode sub-items
                    const modeSubItems: Record<string, boolean> = {
                        connection: prev.llm_connect?.subItems?.connection ?? true,
                    };
                    for (let i = 0; i < llmSettings.modes.length; i++) {
                        const key = subItemKey.mode(i);
                        modeSubItems[key] =
                            prev.llm_connect?.subItems?.[key] ?? true;
                    }
                    next.llm_connect = {
                        selected: prev.llm_connect?.selected ?? true,
                        subItems: modeSubItems,
                    };

                    // Add word sub-items
                    const wordSubItems: Record<string, boolean> = {};
                    for (const word of dictionary) {
                        const key = subItemKey.word(word);
                        wordSubItems[key] =
                            prev.dictionary?.subItems?.[key] ?? true;
                    }
                    next.dictionary = {
                        selected: prev.dictionary?.selected ?? true,
                        subItems: wordSubItems,
                    };

                    return next;
                });
            } catch {
                // Data loading is best-effort, fail silently
            }
        };

        loadData();
    }, []);

    const categoriesWithDynamic = useMemo(() => {
        return buildCategoriesWithDynamic(CATEGORY_DEFINITIONS, {
            ...(rules.length > 0 && {
                formatting_rules: (props) => (
                    <FormattingRulesSubItems
                        rules={rules}
                        selection={props.selection}
                        onToggle={props.onToggle}
                        disabled={props.disabled}
                    />
                ),
            }),
            ...(llmModes.length > 0 && {
                llm_connect: (props) => (
                    <LlmConnectSubItems
                        modes={llmModes}
                        selection={props.selection}
                        onToggle={props.onToggle}
                        disabled={props.disabled}
                    />
                ),
            }),
            ...(dictionaryWords.length > 0 && {
                dictionary: (props) => (
                    <DictionarySubItems
                        words={dictionaryWords}
                        selection={props.selection}
                        onToggle={props.onToggle}
                        disabled={props.disabled}
                        showAll={showAllWords}
                        onShowAll={() => setShowAllWords(true)}
                    />
                ),
            }),
        });
    }, [rules, llmModes, dictionaryWords, showAllWords]);

    const selectedCategories = useMemo(() => {
        return CATEGORY_DEFINITIONS.filter(
            (def) => selection[def.key]?.selected
        ).map((def) => def.key);
    }, [selection]);

    const hasSelection = selectedCategories.length > 0;

    const onExport = () => {
        handleExport(selectedCategories, selection, { allSettings });
    };

    return (
        <div className="space-y-4">
            <Typography.Title className="font-semibold text-sky-400!">{t('Export')}</Typography.Title>

            <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
                <Info className="h-3.5 w-3.5 shrink-0" />
                <span>{t('API key and microphone selection are never exported.')}</span>
            </div>

            <SettingsUI.Container>
                <CategoryTree
                    categories={categoriesWithDynamic}
                    selection={selection}
                    onSelectionChange={setSelection}
                    disabled={isExporting}
                    counters={counters}
                />
            </SettingsUI.Container>

            <div className="flex justify-end mt-2">
                <Page.PrimaryButton
                    onClick={onExport}
                    disabled={!hasSelection || isExporting}
                    aria-disabled={!hasSelection || isExporting}
                >
                    {isExporting ? (
                        <>
                            <Loader2 className="h-4 w-4 animate-spin mr-2" />
                            {t('Exporting...')}
                        </>
                    ) : (
                        t('Export')
                    )}
                </Page.PrimaryButton>
            </div>
        </div>
    );
};
