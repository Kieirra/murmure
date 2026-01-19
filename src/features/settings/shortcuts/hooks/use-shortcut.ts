import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { toast } from 'react-toastify';

interface UseShortcutOptions {
    defaultShortcut: string;
    getCommand: string;
    setCommand: string;
}

export const useShortcut = ({
    defaultShortcut,
    getCommand,
    setCommand,
}: UseShortcutOptions) => {
    const [shortcut, setShortcut] = useState(defaultShortcut);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<string>(getCommand)
            .then((val) => val?.trim() && setShortcut(val))
            .catch((err) =>
                console.error(`Failed to load shortcut (${getCommand}):`, err)
            );
    }, [getCommand]);

    const saveShortcut = async (value: string) => {
        if (!value?.trim()) return;
        try {
            const normalized = await invoke<string>(setCommand, {
                binding: value,
            });
            if (normalized) setShortcut(normalized);
        } catch {
            toast.error(t('Failed to save shortcut'));
        }
    };

    const resetShortcut = () => {
        setShortcut(defaultShortcut);
        saveShortcut(defaultShortcut);
    };

    return {
        shortcut,
        setShortcut: saveShortcut,
        resetShortcut,
    };
};

export const SHORTCUT_CONFIGS = {
    lastTranscript: {
        defaultShortcut: 'ctrl+shift+space',
        getCommand: 'get_last_transcript_shortcut',
        setCommand: 'set_last_transcript_shortcut',
    },
    llm: {
        defaultShortcut: 'ctrl+alt+space',
        getCommand: 'get_llm_record_shortcut',
        setCommand: 'set_llm_record_shortcut',
    },
    command: {
        defaultShortcut: 'ctrl+shift+c',
        getCommand: 'get_command_shortcut',
        setCommand: 'set_command_shortcut',
    },
    record: {
        defaultShortcut: 'ctrl+space',
        getCommand: 'get_record_shortcut',
        setCommand: 'set_record_shortcut',
    },
    llmMode1: {
        defaultShortcut: 'ctrl+shift+1',
        getCommand: 'get_llm_mode_1_shortcut',
        setCommand: 'set_llm_mode_1_shortcut',
    },
    llmMode2: {
        defaultShortcut: 'ctrl+shift+2',
        getCommand: 'get_llm_mode_2_shortcut',
        setCommand: 'set_llm_mode_2_shortcut',
    },
    llmMode3: {
        defaultShortcut: 'ctrl+shift+3',
        getCommand: 'get_llm_mode_3_shortcut',
        setCommand: 'set_llm_mode_3_shortcut',
    },
    llmMode4: {
        defaultShortcut: 'ctrl+shift+4',
        getCommand: 'get_llm_mode_4_shortcut',
        setCommand: 'set_llm_mode_4_shortcut',
    },
};
