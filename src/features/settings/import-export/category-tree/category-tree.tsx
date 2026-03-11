import { ChevronRight } from 'lucide-react';
import { Switch } from '@/components/switch';
import { Collapsible, CollapsibleTrigger, CollapsibleContent } from '@/components/collapsible';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { CategoryKey, CategoryDefinition, CategorySelection, ExportedCategories } from '../import-export.types';
import { hasSubItems, isCategoryOn, getCounterValue } from '../import-export.helpers';
import { useCategorySelection } from './hooks/use-category-selection';

interface CategoryTreeProps {
    categories: CategoryDefinition[];
    selection: CategorySelection;
    onSelectionChange: (selection: CategorySelection) => void;
    disabled?: boolean;
    counters?: Partial<Record<CategoryKey, number>>;
    disabledCategories?: Set<CategoryKey>;
    importedCategories?: ExportedCategories | null;
}

export const CategoryTree = ({
    categories,
    selection,
    onSelectionChange,
    disabled = false,
    counters,
    disabledCategories,
    importedCategories,
}: CategoryTreeProps) => {
    const { t } = useTranslation();
    const { handleCategoryToggle, handleSubItemToggle } = useCategorySelection({ selection, onSelectionChange });

    return (
        <div className="divide-y divide-border">
            {categories.map((def) => {
                const isDisabled = disabled || disabledCategories?.has(def.key);
                const isAbsentFromImport =
                    importedCategories != null && importedCategories[def.key as keyof ExportedCategories] == null;
                const checked = isCategoryOn(def.key, selection);
                const counterValue = getCounterValue(def, counters);
                const IconComponent = def.icon;
                const hasSubs = hasSubItems(def, selection);

                return (
                    <Collapsible key={def.key}>
                        <div className="flex items-center gap-3 px-3 py-2.5">
                            <Switch
                                checked={checked}
                                onCheckedChange={(value) => handleCategoryToggle(def.key, value)}
                                disabled={isDisabled || isAbsentFromImport}
                                aria-label={t(def.label)}
                            />
                            <CollapsibleTrigger
                                disabled={isAbsentFromImport || !hasSubs}
                                className={clsx(
                                    'flex items-center gap-2 flex-1 select-none',
                                    !isAbsentFromImport && hasSubs ? 'cursor-pointer' : 'cursor-default'
                                )}
                            >
                                <IconComponent className="h-4 w-4 text-muted-foreground" />
                                <span className={isAbsentFromImport ? 'text-muted-foreground/50' : 'text-foreground'}>
                                    {t(def.label)}
                                </span>
                                {isAbsentFromImport && (
                                    <span className="text-xs text-muted-foreground/50 italic">
                                        ({t('not included in this file')})
                                    </span>
                                )}
                            </CollapsibleTrigger>
                            {counterValue != null && (
                                <span className="text-xs text-muted-foreground bg-muted px-1.5 py-0.5 rounded-full">
                                    {counterValue}
                                </span>
                            )}
                            {!isAbsentFromImport && hasSubs && (
                                <CollapsibleTrigger className="group/chevron p-1 hover:bg-accent rounded cursor-pointer">
                                    <ChevronRight className="h-4 w-4 text-muted-foreground transition-transform duration-200 group-data-[state=open]/chevron:rotate-90" />
                                </CollapsibleTrigger>
                            )}
                        </div>
                        {!isAbsentFromImport && hasSubs && (
                            <CollapsibleContent>
                                <div className="pl-10 pb-2 space-y-1">
                                    {def.dynamicSubItems == null
                                        ? def.subItems.map((sub) => {
                                              const subChecked = selection[def.key]?.subItems[sub.key] ?? false;
                                              return (
                                                  <label
                                                      key={sub.key}
                                                      className={clsx(
                                                          'flex items-center gap-2 py-1',
                                                          isDisabled ? 'cursor-not-allowed' : 'cursor-pointer'
                                                      )}
                                                  >
                                                      <Switch
                                                          checked={subChecked}
                                                          onCheckedChange={(value) =>
                                                              handleSubItemToggle(def.key, sub.key, value)
                                                          }
                                                          disabled={isDisabled}
                                                          aria-label={t(sub.label)}
                                                      />
                                                      <span className="text-sm text-muted-foreground">
                                                          {t(sub.label)}
                                                      </span>
                                                  </label>
                                              );
                                          })
                                        : def.dynamicSubItems({
                                              selection: selection[def.key]?.subItems ?? {},
                                              onToggle: (subKey, checked) =>
                                                  handleSubItemToggle(def.key, subKey, checked),
                                              disabled: isDisabled,
                                          })}
                                </div>
                            </CollapsibleContent>
                        )}
                    </Collapsible>
                );
            })}
        </div>
    );
};
