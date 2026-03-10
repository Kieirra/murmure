import { FormattingRule } from '@/features/settings/formatting-rules/types';
import { CategoryKey, CategoryDefinition, DynamicSubItemsRenderer } from './types';

export const formatRuleLabel = (rule: FormattingRule): string => {
    const trigger = rule.trigger || '(empty)';
    const replacement = rule.replacement.length > 20
        ? `${rule.replacement.replaceAll('\n', '\u21B5').substring(0, 20)}...`
        : rule.replacement.replaceAll('\n', '\u21B5') || '(delete)';
    return `${trigger} \u2192 ${replacement}`;
};

export const buildCategoriesWithDynamic = (
    definitions: CategoryDefinition[],
    renderers: Partial<Record<CategoryKey, DynamicSubItemsRenderer>>
): CategoryDefinition[] => {
    return definitions.map((def) => {
        const renderer = renderers[def.key];
        if (renderer != null) {
            return { ...def, dynamicSubItems: renderer };
        }
        return def;
    });
};
