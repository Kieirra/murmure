import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { toast } from 'react-toastify';
import {
    findConflict,
    ExistingShortcut,
} from '../shortcuts-regular/shortcut-button/hooks/use-shortcut-interactions.helpers';

interface UseShortcutOptions {
    defaultShortcut: string;
    getCommand: string;
    setCommand: string;
    index?: number;
}

export const useShortcut = ({ defaultShortcut, getCommand, setCommand, index }: UseShortcutOptions) => {
    const [shortcut, setShortcut] = useState(defaultShortcut);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<string>(getCommand, { index })
            .then((val) => val != null && setShortcut(val))
            .catch((err) => console.error(`Failed to load shortcut (${getCommand}):`, err));
    }, [getCommand, index]);

    const saveShortcut = async (value: string) => {
        if (value == null) return;
        try {
            const normalized = await invoke<string>(setCommand, {
                index,
                binding: value,
            });
            setShortcut(normalized);
        } catch {
            toast.error(t('Failed to save shortcut'));
        }
    };

    const resetShortcut = (existingShortcuts: ExistingShortcut[] = []) => {
        const conflict = findConflict(defaultShortcut, existingShortcuts);
        if (conflict != null) {
            toast.error(t('Cannot reset: default shortcut is already used by "{{name}}".', { name: conflict }));
            return;
        }
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
    command: {
        defaultShortcut: 'ctrl+shift+x',
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
    llmTransform1: {
        defaultShortcut: 'ctrl+alt+shift+1',
        getCommand: 'get_llm_transform_shortcut',
        setCommand: 'set_llm_transform_shortcut',
        index: 0,
    },
    llmTransform2: {
        defaultShortcut: 'ctrl+alt+shift+2',
        getCommand: 'get_llm_transform_shortcut',
        setCommand: 'set_llm_transform_shortcut',
        index: 1,
    },
    llmTransform3: {
        defaultShortcut: 'ctrl+alt+shift+3',
        getCommand: 'get_llm_transform_shortcut',
        setCommand: 'set_llm_transform_shortcut',
        index: 2,
    },
    llmTransform4: {
        defaultShortcut: 'ctrl+alt+shift+4',
        getCommand: 'get_llm_transform_shortcut',
        setCommand: 'set_llm_transform_shortcut',
        index: 3,
    },
    cancel: {
        defaultShortcut: 'ctrl+backspace',
        getCommand: 'get_cancel_shortcut',
        setCommand: 'set_cancel_shortcut',
    },
    voiceModeToggle: {
        defaultShortcut: 'ctrl+shift+0',
        getCommand: 'get_voice_mode_toggle_shortcut',
        setCommand: 'set_voice_mode_toggle_shortcut',
    },
};
