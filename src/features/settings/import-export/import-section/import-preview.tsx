import { useState, useMemo } from 'react';
import { Loader2, Info } from 'lucide-react';
import { Page } from '@/components/page';
import { SettingsUI } from '@/components/settings-ui';
import { useTranslation } from '@/i18n';
import { CategoryTree } from '../category-tree/category-tree';
import { FormattingRulesSubItems } from '../formatting-rules-sub-items/formatting-rules-sub-items';
import { LlmConnectSubItems } from '../llm-connect-sub-items/llm-connect-sub-items';
import { DictionarySubItems } from '../dictionary-sub-items/dictionary-sub-items';
import { MergeReplaceToggle } from './merge-replace-toggle/merge-replace-toggle';
import { CATEGORY_DEFINITIONS, subItemKey } from '../constants';
import {
    CategoryKey,
    ImportStrategy,
    MurmureConfigFile,
    ExportedCategories,
} from '../types';
import { buildCategoriesWithDynamic } from '../helpers';
import { FormattingRule } from '@/features/settings/formatting-rules/types';
import { LLMMode } from '@/features/llm-connect/hooks/use-llm-connect';

interface ImportPreviewProps {
    configData: MurmureConfigFile;
    fileName: string;
    isImporting: boolean;
    onImport: (
        selectedCategories: CategoryKey[],
        strategies: Partial<Record<CategoryKey, ImportStrategy>>
    ) => void;
    onCancel: () => void;
}

const buildImportSelection = (categories: ExportedCategories) => {
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
                subItems[subItemKey.rule(rule.id)] = true;
            }
        } else if (def.key === 'llm_connect' && categories.llm_connect != null) {
            subItems['connection'] = true;
            for (let i = 0; i < categories.llm_connect.modes.length; i++) {
                subItems[subItemKey.mode(i)] = true;
            }
        } else if (def.key === 'dictionary' && categories.dictionary != null) {
            for (const word of Object.keys(categories.dictionary)) {
                subItems[subItemKey.word(word)] = true;
            }
        } else {
            for (const sub of def.subItems) {
                subItems[sub.key] = isPresent;
            }
        }

        selection[def.key] = { selected: isPresent, subItems };
    }

    return selection;
};

const getCounters = (
    categories: ExportedCategories
): Partial<Record<CategoryKey, number>> => {
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
};

export const ImportPreview = ({
    configData,
    fileName,
    isImporting,
    onImport,
    onCancel,
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
        llm_connect: 'replace',
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

    const categoriesWithDynamic = useMemo(() => {
        return buildCategoriesWithDynamic(CATEGORY_DEFINITIONS, {
            ...(fileRules.length > 0 && {
                formatting_rules: (props) => (
                    <FormattingRulesSubItems
                        rules={fileRules}
                        selection={props.selection}
                        onToggle={props.onToggle}
                        disabled={props.disabled}
                    />
                ),
            }),
            ...(fileModes.length > 0 && {
                llm_connect: (props) => (
                    <LlmConnectSubItems
                        modes={fileModes}
                        selection={props.selection}
                        onToggle={props.onToggle}
                        disabled={props.disabled}
                    />
                ),
            }),
            ...(fileWords.length > 0 && {
                dictionary: (props) => (
                    <DictionarySubItems
                        words={fileWords}
                        selection={props.selection}
                        onToggle={props.onToggle}
                        disabled={props.disabled}
                        showAll={showAllWords}
                        onShowAll={() => setShowAllWords(true)}
                    />
                ),
            }),
        });
    }, [fileRules, fileModes, fileWords, showAllWords]);

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
                        {t('Created')}: {exportDate}
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
