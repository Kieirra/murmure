import { useWakeWordBase } from './use-wake-word-base';

interface UseLlmWakeWordOptions {
    index: number;
    modeName: string;
}

export const useLlmWakeWord = ({ index, modeName }: UseLlmWakeWordOptions) => {
    const defaultWord = `alix ${modeName.toLowerCase()}`;

    return useWakeWordBase({
        getCommand: 'get_llm_mode_wake_word',
        setCommand: 'set_llm_mode_wake_word',
        commandParams: { index },
        defaultWord,
    });
};
