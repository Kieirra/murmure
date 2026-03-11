import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef, useState } from 'react';
import { useTranslation } from '@/i18n';
import { toast } from 'react-toastify';

interface UseWakeWordBaseOptions {
    getCommand: string;
    setCommand: string;
    commandParams?: Record<string, unknown>;
    defaultWord: string;
}

export const useWakeWordBase = ({
    getCommand,
    setCommand,
    commandParams = {},
    defaultWord,
}: UseWakeWordBaseOptions) => {
    const [wakeWord, setWakeWord] = useState('');
    const [isEnabled, setIsEnabled] = useState(true);
    const previousValue = useRef('');
    const savedWord = useRef('');
    const debounceTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<string>(getCommand, commandParams)
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
            .catch((err) => console.error(`Failed to load wake word (${getCommand}):`, err));
    }, [getCommand, defaultWord]);

    useEffect(() => {
        return () => {
            if (debounceTimer.current != null) {
                clearTimeout(debounceTimer.current);
            }
        };
    }, []);

    const updateWakeWord = (value: string) => {
        setWakeWord(value);

        if (debounceTimer.current != null) {
            clearTimeout(debounceTimer.current);
        }

        debounceTimer.current = setTimeout(async () => {
            try {
                await invoke(setCommand, { ...commandParams, word: value });
                previousValue.current = value;
                savedWord.current = value;
            } catch {
                toast.error(t('This trigger word is already used by another action'));
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
        invoke(setCommand, { ...commandParams, word: current })
            .then(() => {
                previousValue.current = current;
                savedWord.current = current;
            })
            .catch(() => {
                toast.error(t('This trigger word is already used by another action'));
                setWakeWord(previousValue.current);
            });
    };

    const toggleEnabled = () => {
        if (isEnabled) {
            savedWord.current = wakeWord || defaultWord;
            setWakeWord('');
            previousValue.current = '';
            invoke(setCommand, { ...commandParams, word: '' }).catch(() => {});
            setIsEnabled(false);
        } else {
            const restored = savedWord.current || defaultWord;
            setWakeWord(restored);
            previousValue.current = restored;
            setIsEnabled(true);
            invoke(setCommand, { ...commandParams, word: restored })
                .then(() => {
                    savedWord.current = restored;
                })
                .catch(() => {
                    toast.error(t('This trigger word is already used by another action'));
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
        invoke(setCommand, { ...commandParams, word: defaultWord }).catch(() => {
            toast.error(t('This trigger word is already used by another action'));
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
