import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'sonner';

export const useStopRecordingShortcutState = () => {
    const [shortcut, setShortcut] = useState<string | null>(null);

    const loadShortcut = async () => {
        try {
            const value = await invoke<string | null>(
                'get_stop_recording_shortcut'
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
                'set_stop_recording_shortcut',
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
        stopRecordingShortcut: shortcut,
        setStopRecordingShortcut: saveShortcut,
        resetStopRecordingShortcut: resetShortcut,
    };
};
