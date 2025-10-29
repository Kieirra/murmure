import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'sonner';

export const useStartRecordingShortcutState = () => {
    const [shortcut, setShortcut] = useState<string | null>(null);

    const loadShortcut = async () => {
        try {
            const value = await invoke<string | null>(
                'get_start_recording_shortcut'
            );
            if (value && value.trim()) {
                setShortcut(value);
            } else {
                setShortcut(null);
            }
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
                'set_start_recording_shortcut',
                {
                    binding: value,
                }
            );
            if (normalized) setShortcut(normalized);
        } catch {
            toast('Failed to save shortcut');
        }
    };

    const resetShortcut = () => {
        setShortcut(null);
    };

    return {
        startRecordingShortcut: shortcut,
        setStartRecordingShortcut: saveShortcut,
        resetStartRecordingShortcut: resetShortcut,
    };
};
