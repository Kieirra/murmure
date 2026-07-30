import { RenderKeys } from '@/components/render-keys';
import { useShortcut, LLM_MODE_SHORTCUT_CONFIGS } from '@/features/settings/shortcuts/hooks/use-shortcut';
import { VoiceTriggerItem } from '../../voice-trigger-item/voice-trigger-item';
import { useLlmWakeWord } from '../../hooks/use-llm-wake-word';
import type { LLMMode } from '@/features/extensions/llm-connect/hooks/use-llm-connect';

interface LlmTriggerItemProps {
    index: number;
    mode: LLMMode;
}

export const LlmTriggerItem = ({ index, mode }: LlmTriggerItemProps) => {
    const { wakeWord, setWakeWord, handleBlur, isEnabled, toggleEnabled, defaultWord, resetToDefault } = useLlmWakeWord(
        { index, modeName: mode.name }
    );
    const { shortcut } = useShortcut(LLM_MODE_SHORTCUT_CONFIGS[index]);

    return (
        <VoiceTriggerItem
            title={mode.name}
            description={
                <span className="inline-flex items-center gap-1.5">
                    {`Slot ${index + 1}`}
                    {shortcut.length > 0 && (
                        <>
                            <span>-</span>
                            <RenderKeys keyString={shortcut} />
                        </>
                    )}
                </span>
            }
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
