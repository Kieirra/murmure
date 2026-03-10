import { useState, useMemo, useCallback } from 'react';
import { Loader2, Info } from 'lucide-react';
import { Page } from '@/components/page';
import { SettingsUI } from '@/components/settings-ui';
import { Checkbox } from '@/components/checkbox';
import { useTranslation } from '@/i18n';
import { CategoryTree } from '../components/category-tree';
import { MergeReplaceToggle } from '../components/merge-replace-toggle';
import { CATEGORY_DEFINITIONS } from '../constants';
import {
    CategoryKey,
    CategoryDefinition,
    ImportStrategy,
    MurmureConfigFile,
    ExportedCategories,
} from '../types';
import { FormattingRule } from '@/features/settings/formatting-rules/types';
import { LLMMode } from '@/features/llm-connect/hooks/use-llm-connect';

function formatRuleLabel(rule: FormattingRule): string {
    const trigger = rule.trigger || '(empty)';
    const replacement = rule.replacement.length > 20
        ? `${rule.replacement.replaceAll('\n', '\u21B5').substring(0, 20)}...`
        : rule.replacement.replaceAll('\n', '\u21B5') || '(delete)';
    return `${trigger} \u2192 ${replacement}`;
}

interface ImportPreviewProps {
    configData: MurmureConfigFile;
    fileName: string;
    isImporting: boolean;
    onImport: (
        selectedCategories: CategoryKey[],
        strategies: Partial<Record<CategoryKey, ImportStrategy>>
    ) => void;
    onCancel: () => void;
    onChangeFile: () => void;
}

function buildImportSelection(categories: ExportedCategories) {
    const selection: Record<
        string,
        { selected: boolean; subItems: Record<string, boolean> }
    > = {};

    for (const def of CATEGORY_DEFINITIONS) {
        const isPresent =
            categories[def.key as keyof ExportedCategories] != null;
        const subItems: Record<string, boolean> = {};

        if (def.key === 'formatting_rules' && categories.formatting_rules != null) {
            subItems['built_in'] = true;
            for (const rule of categories.formatting_rules.rules) {
                subItems[`rule_${rule.id}`] = true;
            }
        } else if (def.key === 'llm_connect' && categories.llm_connect != null) {
            subItems['connection'] = true;
            for (let i = 0; i < categories.llm_connect.modes.length; i++) {
                subItems[`mode_${i}`] = true;
            }
        } else if (def.key === 'dictionary' && categories.dictionary != null) {
            for (const word of Object.keys(categories.dictionary)) {
                subItems[`word_${word}`] = true;
            }
        } else {
            for (const sub of def.subItems) {
                subItems[sub.key] = isPresent;
            }
        }

        selection[def.key] = { selected: isPresent, subItems };
    }

    return selection;
}

function getCounters(
    categories: ExportedCategories
): Partial<Record<CategoryKey, number>> {
    const counters: Partial<Record<CategoryKey, number>> = {};

    if (categories.formatting_rules != null) {
        counters.formatting_rules = categories.formatting_rules.rules.length;
    }
    if (categories.dictionary != null) {
        counters.dictionary = Object.keys(categories.dictionary).length;
    }
    if (categories.llm_connect != null) {
        counters.llm_connect = categories.llm_connect.modes.length;
    }

    return counters;
}

export const ImportPreview = ({
    configData,
    fileName,
    isImporting,
    onImport,
    onCancel,
    onChangeFile,
}: ImportPreviewProps) => {
    const { t } = useTranslation();
    const categories = configData.categories;

    const [selection, setSelection] = useState(() =>
        buildImportSelection(categories)
    );
    const [strategies, setStrategies] = useState<
        Partial<Record<CategoryKey, ImportStrategy>>
    >({
        formatting_rules: 'replace',
        dictionary: 'replace',
    });
    const [showAllWords, setShowAllWords] = useState(false);

    const fileRules: FormattingRule[] = categories.formatting_rules?.rules ?? [];
    const fileModes: LLMMode[] = categories.llm_connect?.modes ?? [];
    const fileWords: string[] = categories.dictionary != null
        ? Object.keys(categories.dictionary)
        : [];

    const disabledCategories = useMemo(() => {
        const disabled = new Set<CategoryKey>();
        for (const def of CATEGORY_DEFINITIONS) {
            if (categories[def.key as keyof ExportedCategories] == null) {
                disabled.add(def.key);
            }
        }
        return disabled;
    }, [categories]);

    const counters = useMemo(() => getCounters(categories), [categories]);

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
                {fileRules.map((rule) => (
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
        [fileRules, t]
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
                {fileModes.map((mode, index) => (
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
        [fileModes, t]
    );

    const renderDictionarySubItems = useCallback(
        (props: {
            selection: Record<string, boolean>;
            onToggle: (key: string, checked: boolean) => void;
            disabled?: boolean;
        }) => {
            const wordsToShow = showAllWords
                ? fileWords
                : fileWords.slice(0, 15);
            const hiddenCount = fileWords.length - 15;

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
        [fileWords, showAllWords, t]
    );

    const categoriesWithDynamic = useMemo((): CategoryDefinition[] => {
        return CATEGORY_DEFINITIONS.map((def) => {
            if (def.key === 'formatting_rules' && fileRules.length > 0) {
                return {
                    ...def,
                    dynamicSubItems: renderFormattingRulesSubItems,
                };
            }
            if (def.key === 'llm_connect' && fileModes.length > 0) {
                return {
                    ...def,
                    dynamicSubItems: renderLlmConnectSubItems,
                };
            }
            if (def.key === 'dictionary' && fileWords.length > 0) {
                return {
                    ...def,
                    dynamicSubItems: renderDictionarySubItems,
                };
            }
            return def;
        });
    }, [
        fileRules,
        fileModes,
        fileWords,
        renderFormattingRulesSubItems,
        renderLlmConnectSubItems,
        renderDictionarySubItems,
    ]);

    const selectedCategories = useMemo(() => {
        return CATEGORY_DEFINITIONS.filter(
            (def) =>
                selection[def.key]?.selected &&
                !disabledCategories.has(def.key)
        ).map((def) => def.key);
    }, [selection, disabledCategories]);

    const hasSelection = selectedCategories.length > 0;
    const isEmptyFile = Object.keys(categories).length === 0;

    const exportDate = configData.exported_at
        ? new Date(configData.exported_at).toLocaleDateString()
        : '';

    const handleImport = () => {
        onImport(selectedCategories, strategies);
    };

    const mergeableSelected = CATEGORY_DEFINITIONS.filter(
        (def) =>
            def.supportsMerge &&
            selection[def.key]?.selected &&
            !disabledCategories.has(def.key)
    );

    return (
        <div className="space-y-4">
            <div className="flex items-center justify-between text-sm">
                <div className="space-y-1">
                    <p className="text-foreground font-medium">
                        {t('Loaded')}: {fileName}
                    </p>
                    <p className="text-muted-foreground">
                        {t('Version')}: {configData.version} |{' '}
                        {t('Created')}: {exportDate} |{' '}
                        <button
                            className="text-sky-400 hover:text-sky-300 underline"
                            onClick={onChangeFile}
                            disabled={isImporting}
                        >
                            {t('Change file')}
                        </button>
                    </p>
                </div>
            </div>

            {isEmptyFile ? (
                <div className="border border-border rounded-md p-8 text-center">
                    <p className="text-sm text-muted-foreground">
                        {t('This file contains no configuration data.')}
                    </p>
                </div>
            ) : (
                <>
                    <SettingsUI.Container>
                        <CategoryTree
                            categories={categoriesWithDynamic}
                            selection={selection}
                            onSelectionChange={setSelection}
                            disabled={isImporting}
                            counters={counters}
                            disabledCategories={disabledCategories}
                            fileCategories={categories}
                        />
                    </SettingsUI.Container>

                    {mergeableSelected.length > 0 && (
                        <div className="space-y-3">
                            {mergeableSelected.map((def) => (
                                <div
                                    key={def.key}
                                    className="flex items-center gap-3"
                                >
                                    <span className="text-sm text-foreground w-40">
                                        {t(def.label)}:
                                    </span>
                                    <MergeReplaceToggle
                                        value={
                                            strategies[def.key] ?? 'replace'
                                        }
                                        onChange={(strategy) =>
                                            setStrategies((prev) => ({
                                                ...prev,
                                                [def.key]: strategy,
                                            }))
                                        }
                                        disabled={isImporting}
                                    />
                                </div>
                            ))}
                            <div className="flex items-center gap-2 text-sm text-muted-foreground">
                                <Info className="h-4 w-4 shrink-0" />
                                <span>
                                    {t(
                                        'Merge adds to existing data. Replace overwrites completely.'
                                    )}
                                </span>
                            </div>
                        </div>
                    )}
                </>
            )}

            <div className="flex justify-end gap-2">
                <Page.SecondaryButton
                    onClick={onCancel}
                    disabled={isImporting}
                >
                    {t('Cancel')}
                </Page.SecondaryButton>
                <Page.PrimaryButton
                    onClick={handleImport}
                    disabled={!hasSelection || isImporting}
                    aria-disabled={!hasSelection || isImporting}
                >
                    {isImporting ? (
                        <>
                            <Loader2 className="h-4 w-4 animate-spin mr-2" />
                            {t('Importing...')}
                        </>
                    ) : (
                        t('Import')
                    )}
                </Page.PrimaryButton>
            </div>
        </div>
    );
};
