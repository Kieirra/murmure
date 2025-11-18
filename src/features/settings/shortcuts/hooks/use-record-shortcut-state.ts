import { useTranslation } from '@/i18n';
import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';

export const useRecordShortcutState = () => {
    const [shortcut, setShortcut] = useState('ctrl+space');
    const { t } = useTranslation();

    const loadShortcut = async () => {
        try {
            const value = await invoke<string>('get_record_shortcut');
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
            const normalized = await invoke<string>('set_record_shortcut', {
                binding: value,
            });
            if (normalized) setShortcut(normalized);
        } catch {
            toast.error(t('Failed to save shortcut'));
        }
    };

    const resetShortcut = () => {
        setShortcut('ctrl+space');
        saveShortcut('ctrl+space');
    };

    return {
        recordShortcut: shortcut,
        setRecordShortcut: saveShortcut,
        resetRecordShortcut: resetShortcut,
    };
};
