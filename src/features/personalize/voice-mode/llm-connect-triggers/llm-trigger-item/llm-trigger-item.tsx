import { VoiceTriggerItem } from '../../voice-trigger-item/voice-trigger-item';
import { useLlmWakeWord } from '../../hooks/use-llm-wake-word';
import type { LLMMode } from '@/features/personalize/llm-connect/hooks/use-llm-connect';

interface LlmTriggerItemProps {
    index: number;
    mode: LLMMode;
}

export const LlmTriggerItem = ({ index, mode }: LlmTriggerItemProps) => {
    const { wakeWord, setWakeWord, handleBlur, isEnabled, toggleEnabled, defaultWord, resetToDefault } = useLlmWakeWord(
        { index, modeName: mode.name }
    );

    return (
        <VoiceTriggerItem
            title={mode.name}
            description={`Slot ${index + 1} - ${mode.shortcut}`}
            wakeWord={wakeWord}
            onWakeWordChange={setWakeWord}
            onBlur={handleBlur}
            placeholder={defaultWord}
            dataTestId={`wake-word-llm-mode-${index}-input`}
            isEnabled={isEnabled}
            onToggleEnabled={toggleEnabled}
            defaultWord={defaultWord}
            onReset={resetToDefault}
        />
    );
};
