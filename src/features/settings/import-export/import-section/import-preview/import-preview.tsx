import { useState } from 'react';
import { Loader2, Info } from 'lucide-react';
import { Page } from '@/components/page';
import { SettingsUI } from '@/components/settings-ui';
import { useTranslation } from '@/i18n';
import { CategoryTree } from '../../category-tree/category-tree';
import { MergeReplaceToggle } from '../merge-replace-toggle/merge-replace-toggle';
import { CATEGORY_DEFINITIONS } from '../../import-export.constants';
import { CategoryKey, ImportStrategy, MurmureExportData, ExportedCategories } from '../../import-export.types';
import { buildCategoriesWithDynamic, buildImportSelection, getCounters } from '../../import-export.helpers';
import { buildRenderers } from '../../import-export.renderers';
import { FormattingRule } from '@/features/personalize/formatting-rules/types';
import { LLMMode } from '@/features/personalize/llm-connect/hooks/use-llm-connect';

interface ImportPreviewProps {
    configData: MurmureExportData;
    fileName: string;
    isImporting: boolean;
    onImport: (selectedCategories: CategoryKey[], strategies: Partial<Record<CategoryKey, ImportStrategy>>) => void;
    onCancel: () => void;
}

export const ImportPreview = ({ configData, fileName, isImporting, onImport, onCancel }: ImportPreviewProps) => {
    const { t } = useTranslation();
    const categories = configData.categories;

    const [selection, setSelection] = useState(() => buildImportSelection(categories));
    const [strategies, setStrategies] = useState<Partial<Record<CategoryKey, ImportStrategy>>>({
        formatting_rules: 'replace',
        llm_connect: 'replace',
        dictionary: 'replace',
    });
    const fileRules: FormattingRule[] = categories.formatting_rules?.rules ?? [];
    const fileModes: LLMMode[] = categories.llm_connect?.modes ?? [];
    const fileWords: string[] = Object.keys(categories.dictionary ?? {});

    const disabledCategories = new Set<CategoryKey>(
        CATEGORY_DEFINITIONS.filter((def) => categories[def.key as keyof ExportedCategories] == null).map(
            (def) => def.key
        )
    );

    const counters = getCounters(categories);

    const categoriesWithDynamic = buildCategoriesWithDynamic(CATEGORY_DEFINITIONS, buildRenderers(fileRules, fileModes, fileWords));

    const selectedCategories = CATEGORY_DEFINITIONS.filter(
        (def) => selection[def.key]?.selected && !disabledCategories.has(def.key)
    ).map((def) => def.key);

    const hasSelection = selectedCategories.length > 0;
    const isEmptyFile = Object.keys(categories).length === 0;

    const exportDate = configData.exported_at ? new Date(configData.exported_at).toLocaleDateString() : '';

    const mergeableSelected = CATEGORY_DEFINITIONS.filter(
        (def) => def.supportsMerge && selection[def.key]?.selected && !disabledCategories.has(def.key)
    );

    return (
        <div className="space-y-4">
            <div className="flex items-center justify-between text-sm">
                <div className="space-y-1">
                    <p className="text-foreground font-medium">
                        {t('Loaded')}: {fileName}
                    </p>
                    <p className="text-muted-foreground">
                        {t('Version')}: {configData.version} | {t('Created')}: {exportDate}
                    </p>
                </div>
            </div>

            {isEmptyFile ? (
                <div className="border border-border rounded-md p-8 text-center">
                    <p className="text-sm text-muted-foreground">{t('This file contains no configuration data.')}</p>
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
                            importedCategories={categories}
                        />
                    </SettingsUI.Container>

                    {mergeableSelected.length > 0 && (
                        <div className="space-y-3">
                            {mergeableSelected.map((def) => (
                                <div key={def.key} className="flex items-center gap-3">
                                    <span className="text-sm text-foreground w-40">{t(def.label)}:</span>
                                    <MergeReplaceToggle
                                        value={strategies[def.key] ?? 'replace'}
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
                                <span>{t('Merge adds to existing data. Replace overwrites completely.')}</span>
                            </div>
                        </div>
                    )}
                </>
            )}

            <div className="flex justify-end gap-2">
                <Page.SecondaryButton onClick={onCancel} disabled={isImporting}>
                    {t('Cancel')}
                </Page.SecondaryButton>
                <Page.PrimaryButton
                    onClick={() => onImport(selectedCategories, strategies)}
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
