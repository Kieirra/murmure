import { useState, useEffect } from 'react';
import { Info, Loader2 } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { Page } from '@/components/page';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { CategoryTree } from '../category-tree/category-tree';
import { FormattingRulesSubItems } from '../formatting-rules-sub-items/formatting-rules-sub-items';
import { LlmConnectSubItems } from '../llm-connect-sub-items/llm-connect-sub-items';
import { DictionarySubItems } from '../dictionary-sub-items/dictionary-sub-items';
import { useExport } from './hooks/use-export';
import { getSelectedCategoryKeys } from './helpers';
import { CATEGORY_DEFINITIONS, subItemKey } from '../constants';
import { AppSettings, CategoryDefinition } from '../types';
import { FormattingSettings, FormattingRule } from '@/features/settings/formatting-rules/types';
import { LLMConnectSettings, LLMMode } from '@/features/llm-connect/hooks/use-llm-connect';

export const ExportSection = () => {
    const [exportSelection, setExportSelection] = useState(() =>
        Object.fromEntries(
            CATEGORY_DEFINITIONS.map((def) => [def.key, { selected: true, subItems: {} }])
        )
    );
    const [rules, setRules] = useState<FormattingRule[]>([]);
    const [llmModes, setLlmModes] = useState<LLMMode[]>([]);
    const [dictionaryWords, setDictionaryWords] = useState<string[]>([]);
    const [allSettings, setAllSettings] = useState<AppSettings | null>(null);
    const [showAllWords, setShowAllWords] = useState(false);
    const { isExporting, handleExport } = useExport();
    const { t } = useTranslation();

    const counters = {
        formatting_rules: rules.length,
        llm_connect: llmModes.length,
        dictionary: dictionaryWords.length,
    };

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

                setExportSelection(() => ({
                    settings: { selected: true, subItems: {} },
                    shortcuts: { selected: true, subItems: {} },
                    formatting_rules: {
                        selected: true,
                        subItems: Object.fromEntries([
                            ['built_in', true],
                            ...formattingRules.rules.map((r) => [subItemKey.rule(r.id), true]),
                        ]),
                    },
                    llm_connect: {
                        selected: true,
                        subItems: Object.fromEntries([
                            ['connection', true],
                            ...llmSettings.modes.map((_, i) => [subItemKey.mode(i), true]),
                        ]),
                    },
                    dictionary: {
                        selected: true,
                        subItems: Object.fromEntries(
                            dictionary.map((w) => [subItemKey.word(w), true])
                        ),
                    },
                }));
            } catch {
                // Data loading is best-effort, fail silently
            }
        };

        loadData();
    }, []);

    const categories: CategoryDefinition[] = CATEGORY_DEFINITIONS.map((def) => {
        if (def.key === 'formatting_rules' && rules.length > 0) {
            return { ...def, dynamicSubItems: (props) => (
                <FormattingRulesSubItems rules={rules} selection={props.selection} onToggle={props.onToggle} disabled={props.disabled} />
            )};
        }
        if (def.key === 'llm_connect' && llmModes.length > 0) {
            return { ...def, dynamicSubItems: (props) => (
                <LlmConnectSubItems modes={llmModes} selection={props.selection} onToggle={props.onToggle} disabled={props.disabled} />
            )};
        }
        if (def.key === 'dictionary' && dictionaryWords.length > 0) {
            return { ...def, dynamicSubItems: (props) => (
                <DictionarySubItems words={dictionaryWords} selection={props.selection} onToggle={props.onToggle} disabled={props.disabled} showAll={showAllWords} onShowAll={() => setShowAllWords(true)} />
            )};
        }
        return def;
    });

    const selectedCategories = getSelectedCategoryKeys(CATEGORY_DEFINITIONS, exportSelection);

    const hasSelection = selectedCategories.length > 0;

    return (
        <div className="space-y-4">
            <Typography.Title className="font-semibold text-sky-400!">{t('Export')}</Typography.Title>
            <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
                <Info className="h-3.5 w-3.5 shrink-0" />
                <span>{t('API key and microphone selection are never exported.')}</span>
            </div>
            <SettingsUI.Container>
                <CategoryTree
                    categories={categories}
                    selection={exportSelection}
                    onSelectionChange={setExportSelection}
                    disabled={isExporting}
                    counters={counters}
                />
            </SettingsUI.Container>
            <div className="flex justify-end mt-2">
                <Page.PrimaryButton
                    onClick={() => handleExport(selectedCategories, exportSelection, { allSettings })}
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
