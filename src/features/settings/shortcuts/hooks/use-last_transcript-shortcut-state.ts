import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { toast } from 'react-toastify';

export const useLastTranscriptShortcutState = () => {
    const [shortcut, setShortcut] = useState('ctrl+shift+space');
    const { t } = useTranslation();

    const loadShortcut = async () => {
        try {
            const value = await invoke<string>('get_last_transcript_shortcut');
            if (value && value.trim()) setShortcut(value);
        } catch (error) {
            console.error('Failed to load shortcut:', error);
        }
    };

    useEffect(() => {
        loadShortcut();
    }, []);

    const saveShortcut = async (value: string) => {
        if (value == null) return;
        try {
            const normalized = await invoke<string>(
                'set_last_transcript_shortcut',
                {
                    binding: value,
                }
            );
            if (normalized) setShortcut(normalized);
        } catch {
            toast.error(t('Failed to save shortcut'));
        }
    };

    const resetShortcut = () => {
        setShortcut('ctrl+shift+space');
        saveShortcut('ctrl+shift+space');
    };

    return {
        lastTranscriptShortcut: shortcut,
        setLastTranscriptShortcut: saveShortcut,
        resetLastTranscriptShortcut: resetShortcut,
    };
};
