import { useState, useEffect, useMemo, useCallback } from 'react';
import { Info, Loader2 } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { Page } from '@/components/page';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Checkbox } from '@/components/checkbox';
import { useTranslation } from '@/i18n';
import { CategoryTree } from '../components/category-tree';
import { useExport } from './hooks/use-export';
import { CATEGORY_DEFINITIONS } from '../constants';
import { CategoryKey, CategoryDefinition } from '../types';
import { FormattingSettings, FormattingRule } from '@/features/settings/formatting-rules/types';
import { LLMConnectSettings, LLMMode } from '@/features/llm-connect/hooks/use-llm-connect';

function formatRuleLabel(rule: FormattingRule): string {
    const trigger = rule.trigger || '(empty)';
    const replacement = rule.replacement.length > 20
        ? `${rule.replacement.replaceAll('\n', '\u21B5').substring(0, 20)}...`
        : rule.replacement.replaceAll('\n', '\u21B5') || '(delete)';
    return `${trigger} \u2192 ${replacement}`;
}

function buildInitialSelection(
    definitions: CategoryDefinition[]
) {
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
    const [counters, setCounters] = useState<
        Partial<Record<CategoryKey, number>>
    >({});
    const [rules, setRules] = useState<FormattingRule[]>([]);
    const [llmModes, setLlmModes] = useState<LLMMode[]>([]);
    const [dictionaryWords, setDictionaryWords] = useState<string[]>([]);
    const [showAllWords, setShowAllWords] = useState(false);
    const { isExporting, handleExport } = useExport();
    const { t } = useTranslation();

    useEffect(() => {
        const loadData = async () => {
            try {
                const [formattingRules, llmSettings, dictionary] =
                    await Promise.all([
                        invoke<FormattingSettings>('get_formatting_settings'),
                        invoke<LLMConnectSettings>(
                            'get_llm_connect_settings'
                        ),
                        invoke<string[]>('get_dictionary'),
                    ]);

                setRules(formattingRules.rules);
                setLlmModes(llmSettings.modes);
                setDictionaryWords(dictionary);

                setCounters({
                    formatting_rules: formattingRules.rules.length,
                    llm_connect: llmSettings.modes.length,
                    dictionary: dictionary.length,
                });

                // Build dynamic sub-items for selection
                setSelection((prev) => {
                    const next = { ...prev };

                    // Add rule sub-items
                    const ruleSubItems: Record<string, boolean> = {
                        built_in: prev.formatting_rules?.subItems?.built_in ?? true,
                    };
                    for (const rule of formattingRules.rules) {
                        ruleSubItems[`rule_${rule.id}`] =
                            prev.formatting_rules?.subItems?.[`rule_${rule.id}`] ?? true;
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
                        modeSubItems[`mode_${i}`] =
                            prev.llm_connect?.subItems?.[`mode_${i}`] ?? true;
                    }
                    next.llm_connect = {
                        selected: prev.llm_connect?.selected ?? true,
                        subItems: modeSubItems,
                    };

                    // Add word sub-items
                    const wordSubItems: Record<string, boolean> = {};
                    for (const word of dictionary) {
                        wordSubItems[`word_${word}`] =
                            prev.dictionary?.subItems?.[`word_${word}`] ?? true;
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

    const renderFormattingRulesSubItems = useCallback(
        (props: {
            selection: Record<string, boolean>;
            onToggle: (key: string, checked: boolean) => void;
            disabled?: boolean;
        }) => (
            <>
                <div className="flex items-center gap-2 py-1">
                    <Checkbox
                        checked={props.selection['built_in'] ?? false}
                        onCheckedChange={(checked) =>
                            props.onToggle('built_in', checked === true)
                        }
                        disabled={props.disabled}
                        aria-label={t('Built-in Options')}
                    />
                    <span className="text-sm text-muted-foreground">
                        {t('Built-in Options')}
                    </span>
                </div>
                {rules.map((rule) => (
                    <div
                        key={rule.id}
                        className="flex items-center gap-2 py-1"
                        style={rule.enabled ? undefined : { opacity: 0.5 }}
                    >
                        <Checkbox
                            checked={props.selection[`rule_${rule.id}`] ?? false}
                            onCheckedChange={(checked) =>
                                props.onToggle(`rule_${rule.id}`, checked === true)
                            }
                            disabled={props.disabled}
                            aria-label={formatRuleLabel(rule)}
                        />
                        <span className="text-sm text-muted-foreground truncate">
                            <span className="font-medium text-foreground">
                                {rule.trigger || t('(empty trigger)')}
                            </span>
                            {' \u2192 '}
                            {rule.replacement.length > 20
                                ? `${rule.replacement.replaceAll('\n', '\u21B5').substring(0, 20)}...`
                                : rule.replacement.replaceAll('\n', '\u21B5') || t('(delete)')}
                        </span>
                    </div>
                ))}
            </>
        ),
        [rules, t]
    );

    const renderLlmConnectSubItems = useCallback(
        (props: {
            selection: Record<string, boolean>;
            onToggle: (key: string, checked: boolean) => void;
            disabled?: boolean;
        }) => (
            <>
                <div className="flex items-center gap-2 py-1">
                    <Checkbox
                        checked={props.selection['connection'] ?? false}
                        onCheckedChange={(checked) =>
                            props.onToggle('connection', checked === true)
                        }
                        disabled={props.disabled}
                        aria-label={t('Connection Settings')}
                    />
                    <span className="text-sm text-muted-foreground">
                        {t('Connection Settings')}
                    </span>
                </div>
                {llmModes.map((mode, index) => (
                    <div
                        key={index}
                        className="flex items-center gap-2 py-1"
                    >
                        <Checkbox
                            checked={props.selection[`mode_${index}`] ?? false}
                            onCheckedChange={(checked) =>
                                props.onToggle(`mode_${index}`, checked === true)
                            }
                            disabled={props.disabled}
                            aria-label={mode.name}
                        />
                        <span className="text-sm text-muted-foreground">
                            {mode.name}
                        </span>
                    </div>
                ))}
            </>
        ),
        [llmModes, t]
    );

    const renderDictionarySubItems = useCallback(
        (props: {
            selection: Record<string, boolean>;
            onToggle: (key: string, checked: boolean) => void;
            disabled?: boolean;
        }) => {
            const wordsToShow = showAllWords
                ? dictionaryWords
                : dictionaryWords.slice(0, 15);
            const hiddenCount = dictionaryWords.length - 15;

            return (
                <div className="flex flex-wrap gap-2 py-1">
                    {wordsToShow.map((word) => {
                        const isSelected =
                            props.selection[`word_${word}`] ?? true;
                        return (
                            <button
                                key={word}
                                type="button"
                                onClick={() =>
                                    props.onToggle(
                                        `word_${word}`,
                                        !isSelected
                                    )
                                }
                                disabled={props.disabled}
                                className={`inline-flex items-center px-3 py-1.5 text-xs rounded-md border transition-colors ${
                                    isSelected
                                        ? 'bg-primary/10 border-primary text-foreground'
                                        : 'bg-card border-border opacity-50'
                                }`}
                            >
                                {word}
                            </button>
                        );
                    })}
                    {!showAllWords && hiddenCount > 0 && (
                        <button
                            type="button"
                            onClick={() => setShowAllWords(true)}
                            className="inline-flex items-center px-3 py-1.5 text-xs rounded-md border border-border text-muted-foreground hover:bg-accent transition-colors"
                        >
                            +{hiddenCount} {t('more...')}
                        </button>
                    )}
                </div>
            );
        },
        [dictionaryWords, showAllWords, t]
    );

    const categoriesWithDynamic = useMemo((): CategoryDefinition[] => {
        return CATEGORY_DEFINITIONS.map((def) => {
            if (def.key === 'formatting_rules' && rules.length > 0) {
                return {
                    ...def,
                    dynamicSubItems: renderFormattingRulesSubItems,
                };
            }
            if (def.key === 'llm_connect' && llmModes.length > 0) {
                return {
                    ...def,
                    dynamicSubItems: renderLlmConnectSubItems,
                };
            }
            if (def.key === 'dictionary' && dictionaryWords.length > 0) {
                return {
                    ...def,
                    dynamicSubItems: renderDictionarySubItems,
                };
            }
            return def;
        });
    }, [
        rules,
        llmModes,
        dictionaryWords,
        renderFormattingRulesSubItems,
        renderLlmConnectSubItems,
        renderDictionarySubItems,
    ]);

    const selectedCategories = useMemo(() => {
        return CATEGORY_DEFINITIONS.filter(
            (def) => selection[def.key]?.selected
        ).map((def) => def.key);
    }, [selection]);

    const hasSelection = selectedCategories.length > 0;

    const onExport = () => {
        handleExport(selectedCategories, selection);
    };

    return (
        <div className="space-y-4">
            <Typography.Title>{t('Export')}</Typography.Title>

            <SettingsUI.Container>
                <CategoryTree
                    categories={categoriesWithDynamic}
                    selection={selection}
                    onSelectionChange={setSelection}
                    disabled={isExporting}
                    counters={counters}
                />
            </SettingsUI.Container>

            <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <Info className="h-4 w-4 shrink-0" />
                <span>
                    {t(
                        'API key and microphone selection are never exported.'
                    )}
                </span>
            </div>

            <div className="flex justify-end">
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
