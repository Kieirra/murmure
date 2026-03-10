import { useState } from 'react';
import { ChevronRight, ChevronDown, Minus } from 'lucide-react';
import { Checkbox } from '@/components/checkbox';
import { useTranslation } from '@/i18n';
import { CategoryKey, CategoryDefinition, ExportedCategories } from '../types';

interface CategorySelection {
    [categoryKey: string]: {
        selected: boolean;
        subItems: Record<string, boolean>;
    };
}

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

    const getCategoryState = (
        categoryKey: string
    ): 'checked' | 'unchecked' | 'indeterminate' => {
        const cat = selection[categoryKey];
        if (cat == null) {
            return 'unchecked';
        }
        if (!cat.selected) {
            return 'unchecked';
        }
        const subValues = Object.values(cat.subItems);
        if (subValues.length === 0) {
            return cat.selected ? 'checked' : 'unchecked';
        }
        const allChecked = subValues.every((v) => v);
        const someChecked = subValues.some((v) => v);

        if (allChecked) {
            return 'checked';
        }
        if (someChecked) {
            return 'indeterminate';
        }
        return 'unchecked';
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

    const handleSubItemToggle = (
        categoryKey: string,
        subKey: string,
        checked: boolean
    ) => {
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

    const getCounterText = (def: CategoryDefinition): string | null => {
        if (counters == null) {
            return null;
        }
        const count = counters[def.key];
        if (count == null) {
            return null;
        }
        switch (def.key) {
            case 'formatting_rules':
                return `${count} ${t('rules')}`;
            case 'dictionary':
                return `${count} ${t('words')}`;
            case 'llm_connect':
                return `${count} ${t('modes')}`;
            default:
                return null;
        }
    };

    return (
        <div className="divide-y divide-border">
            {categories.map((def) => {
                const isDisabled =
                    disabled || disabledCategories?.has(def.key);
                const isNotIncluded =
                    fileCategories != null &&
                    fileCategories[def.key as keyof ExportedCategories] == null;
                const state = getCategoryState(def.key);
                const isExpanded = expanded.has(def.key);
                const counterText = getCounterText(def);
                const IconComponent = def.icon;
                const hasSubs = hasSubItems(def);

                return (
                    <div key={def.key}>
                        <div className="flex items-center gap-3 p-3">
                            <Checkbox
                                checked={
                                    state === 'checked'
                                        ? true
                                        : state === 'indeterminate'
                                          ? 'indeterminate'
                                          : false
                                }
                                onCheckedChange={(checked) =>
                                    handleCategoryToggle(
                                        def.key,
                                        checked === true
                                    )
                                }
                                disabled={isDisabled || isNotIncluded}
                                aria-label={t(def.label)}
                            />
                            {state === 'indeterminate' && (
                                <Minus className="h-3 w-3 text-primary absolute pointer-events-none" />
                            )}
                            <div
                                className="flex items-center gap-2 flex-1 cursor-pointer select-none"
                                onClick={() => {
                                    if (!isNotIncluded && hasSubs) {
                                        toggleExpand(def.key);
                                    }
                                }}
                            >
                                <IconComponent className="h-4 w-4 text-muted-foreground" />
                                <span
                                    className={
                                        isNotIncluded
                                            ? 'text-muted-foreground/50'
                                            : 'text-foreground'
                                    }
                                >
                                    {t(def.label)}
                                </span>
                                {counterText != null && (
                                    <span className="text-xs text-muted-foreground">
                                        ({counterText})
                                    </span>
                                )}
                                {isNotIncluded && (
                                    <span className="text-xs text-muted-foreground/50 italic">
                                        ({t('not included in this file')})
                                    </span>
                                )}
                            </div>
                            {!isNotIncluded && hasSubs && (
                                <button
                                    className="p-1 hover:bg-accent rounded"
                                    onClick={() => toggleExpand(def.key)}
                                    aria-expanded={isExpanded}
                                    aria-label={
                                        isExpanded
                                            ? t('Collapse')
                                            : t('Expand')
                                    }
                                >
                                    {isExpanded ? (
                                        <ChevronDown className="h-4 w-4 text-muted-foreground" />
                                    ) : (
                                        <ChevronRight className="h-4 w-4 text-muted-foreground" />
                                    )}
                                </button>
                            )}
                        </div>
                        {isExpanded && !isNotIncluded && hasSubs && (
                            <div className="pl-10 pb-2 space-y-1">
                                {def.dynamicSubItems != null ? (
                                    def.dynamicSubItems({
                                        selection:
                                            selection[def.key]?.subItems ?? {},
                                        onToggle: (subKey, checked) =>
                                            handleSubItemToggle(
                                                def.key,
                                                subKey,
                                                checked
                                            ),
                                        disabled: isDisabled,
                                    })
                                ) : (
                                    def.subItems.map((sub) => {
                                        const subChecked =
                                            selection[def.key]?.subItems[
                                                sub.key
                                            ] ?? false;
                                        return (
                                            <div
                                                key={sub.key}
                                                className="flex items-center gap-2 py-1"
                                            >
                                                <Checkbox
                                                    checked={subChecked}
                                                    onCheckedChange={(
                                                        checked
                                                    ) =>
                                                        handleSubItemToggle(
                                                            def.key,
                                                            sub.key,
                                                            checked === true
                                                        )
                                                    }
                                                    disabled={isDisabled}
                                                    aria-label={t(sub.label)}
                                                />
                                                <span className="text-sm text-muted-foreground">
                                                    {t(sub.label)}
                                                </span>
                                            </div>
                                        );
                                    })
                                )}
                            </div>
                        )}
                    </div>
                );
            })}
        </div>
    );
};
