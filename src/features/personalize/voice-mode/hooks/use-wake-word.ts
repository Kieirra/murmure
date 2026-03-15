import { useWakeWordBase } from './use-wake-word-base';

interface UseWakeWordOptions {
    getCommand: string;
    setCommand: string;
    defaultWord: string;
}

export const useWakeWord = ({ getCommand, setCommand, defaultWord }: UseWakeWordOptions) => {
    return useWakeWordBase({
        getCommand,
        setCommand,
        defaultWord,
    });
};

export const WAKE_WORD_CONFIGS = {
    record: {
        getCommand: 'get_wake_word_record',
        setCommand: 'set_wake_word_record',
        defaultWord: 'ok alix',
    },
    command: {
        getCommand: 'get_wake_word_command',
        setCommand: 'set_wake_word_command',
        defaultWord: 'alix command',
    },
    cancel: {
        getCommand: 'get_wake_word_cancel',
        setCommand: 'set_wake_word_cancel',
        defaultWord: 'alix cancel',
    },
    validate: {
        getCommand: 'get_wake_word_validate',
        setCommand: 'set_wake_word_validate',
        defaultWord: 'alix validate',
    },
};
