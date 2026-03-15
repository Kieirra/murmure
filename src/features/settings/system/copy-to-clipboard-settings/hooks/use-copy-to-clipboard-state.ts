import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useCopyToClipboardState = () => {
    const [copyToClipboard, setCopyToClipboard] = useState<boolean>(false);
    const { t } = useTranslation();

    const loadClipboardState = async () => {
        try {
            const enabled = await invoke<boolean>('get_copy_to_clipboard');
            setCopyToClipboard(enabled);
        } catch (error) {
            console.error('Failed to load copy to clipboard state:', error);
        }
    };

    useEffect(() => {
        loadClipboardState();
    }, []);

    const handleSetCopyToClipboard = async (enabled: boolean) => {
        try {
            setCopyToClipboard(enabled);
            await invoke('set_copy_to_clipboard', { enabled });
        } catch (error) {
            console.error('Failed to set copy to clipboard:', error);
            toast.error(t('Failed to save clipboard setting'));
            // Revert the state on error
            setCopyToClipboard(!enabled);
        }
    };

    return {
        copyToClipboard,
        setCopyToClipboard: handleSetCopyToClipboard,
    };
};
