import { invoke } from '@tauri-apps/api/core';
import { useState, useCallback, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export interface PairedDevice {
    token: string;
    name: string;
    last_connected: string;
    created_at: string;
}

interface UsePairedDevicesOptions {
    enabled: boolean | null;
}

export const usePairedDevices = ({ enabled }: UsePairedDevicesOptions) => {
    const [devices, setDevices] = useState<PairedDevice[]>([]);
    const { t } = useTranslation();

    const refresh = useCallback(async () => {
        try {
            const list = await invoke<PairedDevice[]>('get_paired_devices');
            setDevices(list);
        } catch (error) {
            console.error('Failed to load paired devices:', error);
        }
    }, []);

    useEffect(() => {
        if (enabled === true) {
            refresh();
        }
    }, [enabled, refresh]);

    const remove = useCallback(
        async (token: string) => {
            try {
                await invoke('remove_paired_device', { token });
                await refresh();
            } catch (error) {
                console.error('Failed to remove paired device:', error);
                toast.error(t('Failed to remove device'));
            }
        },
        [refresh, t]
    );

    const resetTokens = useCallback(async () => {
        try {
            await invoke('reset_smartmic_tokens');
            await refresh();
        } catch (error) {
            console.error('Failed to reset SmartMic tokens:', error);
            toast.error(t('Failed to reset QR code'));
        }
    }, [refresh, t]);

    return {
        devices,
        remove,
        resetTokens,
        refresh,
    };
};
