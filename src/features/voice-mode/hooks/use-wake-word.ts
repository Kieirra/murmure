import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef, useState } from 'react';
import { useTranslation } from '@/i18n';
import { toast } from 'react-toastify';

interface UseWakeWordOptions {
    getCommand: string;
    setCommand: string;
    defaultWord: string;
}

export const useWakeWord = ({
    getCommand,
    setCommand,
    defaultWord,
}: UseWakeWordOptions) => {
    const [wakeWord, setWakeWord] = useState('');
    const [isEnabled, setIsEnabled] = useState(true);
    const previousValue = useRef('');
    const savedWord = useRef('');
    const debounceTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<string>(getCommand)
            .then((val) => {
                setWakeWord(val);
                previousValue.current = val;
                if (val.trim() === '') {
                    setIsEnabled(false);
                    savedWord.current = defaultWord;
                } else {
                    setIsEnabled(true);
                    savedWord.current = val;
                }
            })
            .catch((err) =>
                console.error(`Failed to load wake word (${getCommand}):`, err)
            );
    }, [getCommand, defaultWord]);

    const updateWakeWord = (value: string) => {
        setWakeWord(value);

        if (debounceTimer.current != null) {
            clearTimeout(debounceTimer.current);
        }

        debounceTimer.current = setTimeout(async () => {
            try {
                await invoke(setCommand, { word: value });
                previousValue.current = value;
                savedWord.current = value;
            } catch {
                toast.error(
                    t(
                        'This trigger word is already used by another action'
                    )
                );
                setWakeWord(previousValue.current);
            }
        }, 500);
    };

    const handleBlur = () => {
        if (debounceTimer.current != null) {
            clearTimeout(debounceTimer.current);
            debounceTimer.current = null;
        }

        const current = wakeWord;
        invoke(setCommand, { word: current })
            .then(() => {
                previousValue.current = current;
                savedWord.current = current;
            })
            .catch(() => {
                toast.error(
                    t(
                        'This trigger word is already used by another action'
                    )
                );
                setWakeWord(previousValue.current);
            });
    };

    const toggleEnabled = () => {
        if (isEnabled) {
            savedWord.current = wakeWord || defaultWord;
            setWakeWord('');
            previousValue.current = '';
            invoke(setCommand, { word: '' }).catch(() => {});
            setIsEnabled(false);
        } else {
            const restored = savedWord.current || defaultWord;
            setWakeWord(restored);
            previousValue.current = restored;
            setIsEnabled(true);
            invoke(setCommand, { word: restored })
                .then(() => {
                    savedWord.current = restored;
                })
                .catch(() => {
                    toast.error(
                        t(
                            'This trigger word is already used by another action'
                        )
                    );
                    setWakeWord('');
                    previousValue.current = '';
                    savedWord.current = defaultWord;
                    setIsEnabled(false);
                });
        }
    };

    const resetToDefault = () => {
        const oldValue = previousValue.current;
        const oldSavedWord = savedWord.current;
        const oldIsEnabled = isEnabled;
        setWakeWord(defaultWord);
        previousValue.current = defaultWord;
        savedWord.current = defaultWord;
        setIsEnabled(true);
        invoke(setCommand, { word: defaultWord })
            .catch(() => {
                toast.error(
                    t(
                        'This trigger word is already used by another action'
                    )
                );
                setWakeWord(oldValue);
                previousValue.current = oldValue;
                savedWord.current = oldSavedWord;
                setIsEnabled(oldIsEnabled);
            });
    };

    return {
        wakeWord,
        setWakeWord: updateWakeWord,
        handleBlur,
        isEnabled,
        toggleEnabled,
        defaultWord,
        resetToDefault,
    };
};

export const WAKE_WORD_CONFIGS = {
    record: {
        getCommand: 'get_wake_word_record',
        setCommand: 'set_wake_word_record',
        defaultWord: 'alix',
    },
    llm: {
        getCommand: 'get_wake_word_llm',
        setCommand: 'set_wake_word_llm',
        defaultWord: 'alix connect',
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
