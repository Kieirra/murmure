import { buildSubItems } from '../import-export.helpers';
import { SUB_ITEM_KEY } from '../import-export.constants';
import { CategoryDefinition, CategorySelection } from '../import-export.types';
import { FormattingSettings } from '@/features/personalize/formatting-rules/types';
import { LLMConnectSettings } from '@/features/personalize/llm-connect/hooks/use-llm-connect';

export const getSelectedCategoryKeys = (definitions: CategoryDefinition[], selection: CategorySelection) => {
    return definitions.filter((def) => selection[def.key]?.selected).map((def) => def.key);
};

export const buildInitialSelection = (
    formattingRules: FormattingSettings,
    llmSettings: LLMConnectSettings,
    dictionary: string[]
): CategorySelection => ({
    settings: { selected: true, subItems: {} },
    shortcuts: { selected: true, subItems: {} },
    formatting_rules: {
        selected: true,
        subItems: buildSubItems(
            formattingRules.rules.map((r) => SUB_ITEM_KEY.rule(r.id)),
            ['built_in']
        ),
    },
    llm_connect: {
        selected: true,
        subItems: buildSubItems(
            llmSettings.modes.map((_, i) => SUB_ITEM_KEY.mode(i)),
            ['connection']
        ),
    },
    dictionary: {
        selected: true,
        subItems: buildSubItems(dictionary.map((w) => SUB_ITEM_KEY.word(w))),
    },
});
