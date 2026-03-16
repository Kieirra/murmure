import { FormattingRulesSubItems } from './formatting-rules-sub-items/formatting-rules-sub-items';
import { LlmConnectSubItems } from './llm-connect-sub-items/llm-connect-sub-items';
import { SelectableWordList } from './selectable-word-list/selectable-word-list';
import { CategoryKey, DynamicSubItemsRenderer } from './import-export.types';
import { FormattingRule } from '@/features/personalize/formatting-rules/types';
import { LLMMode } from '@/features/personalize/llm-connect/hooks/use-llm-connect';

export const buildRenderers = (
    rules: FormattingRule[],
    modes: LLMMode[],
    words: string[]
): Partial<Record<CategoryKey, DynamicSubItemsRenderer>> => {
    const renderers: Partial<Record<CategoryKey, DynamicSubItemsRenderer>> = {};
    if (rules.length > 0) {
        renderers.formatting_rules = (props) => <FormattingRulesSubItems rules={rules} {...props} />;
    }
    if (modes.length > 0) {
        renderers.llm_connect = (props) => <LlmConnectSubItems modes={modes} {...props} />;
    }
    if (words.length > 0) {
        renderers.dictionary = (props) => <SelectableWordList words={words} {...props} />;
    }
    return renderers;
};
