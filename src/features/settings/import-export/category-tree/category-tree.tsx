import { useState } from 'react';
import { ChevronRight, ChevronDown } from 'lucide-react';
import { Switch } from '@/components/switch';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';
import { CategoryKey, CategoryDefinition, CategorySelection, ExportedCategories } from '../types';

interface CategoryTreeProps {
    categories: CategoryDefinition[];
    selection: CategorySelection;
    onSelectionChange: (selection: CategorySelection) => void;
    disabled?: boolean;
    counters?: Partial<Record<CategoryKey, number>>;
    disabledCategories?: Set<CategoryKey>;
    fileCategories?: ExportedCategories | null;
}

export const CategoryTree = ({
    categories,
    selection,
    onSelectionChange,
    disabled = false,
    counters,
    disabledCategories,
    fileCategories,
}: CategoryTreeProps) => {
    const [expanded, setExpanded] = useState<Set<string>>(new Set());
    const { t } = useTranslation();

    const toggleExpand = (key: string) => {
        setExpanded((prev) => {
            const next = new Set(prev);
            if (next.has(key)) {
                next.delete(key);
            } else {
                next.add(key);
            }
            return next;
        });
    };

    const hasSubItems = (def: CategoryDefinition): boolean => {
        const subKeys = Object.keys(selection[def.key]?.subItems ?? {});
        return subKeys.length > 0;
    };

    const isCategoryOn = (categoryKey: string): boolean => {
        const cat = selection[categoryKey];
        if (cat == null) {
            return false;
        }
        if (!cat.selected) {
            return false;
        }
        const subValues = Object.values(cat.subItems);
        if (subValues.length === 0) {
            return cat.selected;
        }
        return subValues.some((v) => v);
    };

    const handleCategoryToggle = (categoryKey: string, checked: boolean) => {
        const cat = selection[categoryKey];
        if (cat == null) {
            return;
        }

        const newSubItems: Record<string, boolean> = {};
        for (const key of Object.keys(cat.subItems)) {
            newSubItems[key] = checked;
        }

        onSelectionChange({
            ...selection,
            [categoryKey]: {
                selected: checked,
                subItems: newSubItems,
            },
        });
    };

    const handleSubItemToggle = (categoryKey: string, subKey: string, checked: boolean) => {
        const cat = selection[categoryKey];
        if (cat == null) {
            return;
        }

        const newSubItems = { ...cat.subItems, [subKey]: checked };
        const anySelected = Object.values(newSubItems).some((v) => v);

        onSelectionChange({
            ...selection,
            [categoryKey]: {
                selected: anySelected,
                subItems: newSubItems,
            },
        });
    };

    const getCounterValue = (def: CategoryDefinition): number | null => {
        if (counters == null) {
            return null;
        }
        const count = counters[def.key];
        if (count == null) {
            return null;
        }
        return count;
    };

    return (
        <div className="divide-y divide-border">
            {categories.map((def) => {
                const isDisabled = disabled || disabledCategories?.has(def.key);
                const isNotIncluded =
                    fileCategories != null && fileCategories[def.key as keyof ExportedCategories] == null;
                const checked = isCategoryOn(def.key);
                const isExpanded = expanded.has(def.key);
                const counterValue = getCounterValue(def);
                const IconComponent = def.icon;
                const hasSubs = hasSubItems(def);

                return (
                    <div key={def.key}>
                        <div className="flex items-center gap-3 px-3 py-2.5">
                            <Switch
                                checked={checked}
                                onCheckedChange={(value) => handleCategoryToggle(def.key, value)}
                                disabled={isDisabled || isNotIncluded}
                                aria-label={t(def.label)}
                            />
                            <div
                                className={clsx(
                                    'flex items-center gap-2 flex-1 select-none',
                                    !isNotIncluded && hasSubs ? 'cursor-pointer' : undefined
                                )}
                                onClick={() => {
                                    if (!isNotIncluded && hasSubs) {
                                        toggleExpand(def.key);
                                    }
                                }}
                            >
                                <IconComponent className="h-4 w-4 text-muted-foreground" />
                                <span className={isNotIncluded ? 'text-muted-foreground/50' : 'text-foreground'}>
                                    {t(def.label)}
                                </span>
                                {isNotIncluded && (
                                    <span className="text-xs text-muted-foreground/50 italic">
                                        ({t('not included in this file')})
                                    </span>
                                )}
                            </div>
                            {counterValue != null && (
                                <span className="text-xs text-muted-foreground bg-muted px-1.5 py-0.5 rounded-full">
                                    {counterValue}
                                </span>
                            )}
                            {!isNotIncluded && hasSubs && (
                                <button
                                    className="p-1 hover:bg-accent rounded cursor-pointer"
                                    onClick={() => toggleExpand(def.key)}
                                    aria-expanded={isExpanded}
                                    aria-label={isExpanded ? t('Collapse') : t('Expand')}
                                >
                                    {isExpanded ? (
                                        <ChevronDown className="h-4 w-4 text-muted-foreground" />
                                    ) : (
                                        <ChevronRight className="h-4 w-4 text-muted-foreground" />
                                    )}
                                </button>
                            )}
                        </div>
                        {!isNotIncluded && hasSubs && (
                            <div
                                className={clsx(
                                    'grid transition-all duration-200',
                                    isExpanded ? 'grid-rows-[1fr] opacity-100' : 'grid-rows-[0fr] opacity-0'
                                )}
                            >
                                <div className="overflow-hidden">
                                    <div className="pl-10 pb-2 space-y-1">
                                        {def.dynamicSubItems != null
                                            ? def.dynamicSubItems({
                                                  selection: selection[def.key]?.subItems ?? {},
                                                  onToggle: (subKey, checked) =>
                                                      handleSubItemToggle(def.key, subKey, checked),
                                                  disabled: isDisabled,
                                              })
                                            : def.subItems.map((sub) => {
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
                                              })}
                                    </div>
                                </div>
                            </div>
                        )}
                    </div>
                );
            })}
        </div>
    );
};
