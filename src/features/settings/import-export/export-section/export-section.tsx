import { Info, Loader2 } from 'lucide-react';
import { Page } from '@/components/page';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { CategoryTree } from '../category-tree/category-tree';
import { FormattingRulesSubItems } from '../formatting-rules-sub-items/formatting-rules-sub-items';
import { LlmConnectSubItems } from '../llm-connect-sub-items/llm-connect-sub-items';
import { SelectableWordList } from '../selectable-word-list/selectable-word-list';
import { useExport } from './hooks/use-export';
import { useExportData } from './hooks/use-export-data';
import { getSelectedCategoryKeys } from './export-section.helpers';
import { buildCategoriesWithDynamic } from '../import-export.helpers';
import { CATEGORY_DEFINITIONS } from '../import-export.constants';

export const ExportSection = () => {
    const { rules, llmModes, dictionaryWords, allSettings, exportSelection, setExportSelection } = useExportData();
    const { isExporting, handleExport } = useExport();
    const { t } = useTranslation();

    const counters = {
        formatting_rules: rules.length,
        llm_connect: llmModes.length,
        dictionary: dictionaryWords.length,
    };

    const categories = buildCategoriesWithDynamic(CATEGORY_DEFINITIONS, {
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
                <SelectableWordList
                    words={dictionaryWords}
                    selection={props.selection}
                    onToggle={props.onToggle}
                    disabled={props.disabled}
                />
            ),
        }),
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
