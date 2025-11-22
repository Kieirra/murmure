import { useTranslation } from '@/i18n';
import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';

export const useLLMShortcutState = () => {
    const [shortcut, setShortcut] = useState('ctrl+alt+space');
    const { t } = useTranslation();

    const loadShortcut = async () => {
        try {
            const value = await invoke<string>('get_llm_record_shortcut');
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
            const normalized = await invoke<string>('set_llm_record_shortcut', {
                binding: value,
            });
            if (normalized) setShortcut(normalized);
        } catch {
            toast.error(t('Failed to save shortcut'));
        }
    };

    const resetShortcut = () => {
        setShortcut('ctrl+alt+space');
        saveShortcut('ctrl+alt+space');
    };

    return {
        llmShortcut: shortcut,
        setLLMShortcut: saveShortcut,
        resetLLMShortcut: resetShortcut,
    };
};
