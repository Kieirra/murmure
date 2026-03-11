import { CategorySelection } from '../../import-export.types';

interface UseCategorySelectionParams {
    selection: CategorySelection;
    onSelectionChange: (selection: CategorySelection) => void;
}

export const useCategorySelection = ({ selection, onSelectionChange }: UseCategorySelectionParams) => {
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
        const anySelected = Object.values(newSubItems).some(Boolean);

        onSelectionChange({
            ...selection,
            [categoryKey]: {
                selected: anySelected,
                subItems: newSubItems,
            },
        });
    };

    return {
        handleCategoryToggle,
        handleSubItemToggle,
    };
};
