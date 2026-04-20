import { invoke } from '@tauri-apps/api/core';
import { useState, useCallback, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export interface NetworkInterface {
    name: string;
    ip: string;
}

interface UseBindAddressOptions {
    enabled: boolean;
    onChange: () => Promise<void> | void;
}

export const useBindAddress = ({ enabled, onChange }: UseBindAddressOptions) => {
    const [bindAddress, setBindAddress] = useState<string | null>(null);
    const [availableInterfaces, setAvailableInterfaces] = useState<NetworkInterface[]>([]);
    const { t } = useTranslation();

    const load = useCallback(async () => {
        try {
            const value = await invoke<string | null>('get_smartmic_bind_address');
            const interfaces = await invoke<NetworkInterface[]>('list_smartmic_network_interfaces');
            setBindAddress(value);
            setAvailableInterfaces(interfaces);
        } catch (error) {
            console.error('Failed to load bind address:', error);
        }
    }, []);

    useEffect(() => {
        load();
    }, [load]);

    const saveBindAddress = useCallback(
        async (value: string | null) => {
            try {
                setBindAddress(value);
                await invoke('set_smartmic_bind_address', { address: value });

                if (enabled) {
                    await onChange();
                }
            } catch (error) {
                console.error('Failed to set Smart Mic bind address:', error);
                toast.error(t('Failed to save Smart Mic bind address'));
            }
        },
        [enabled, onChange, t]
    );

    return {
        bindAddress,
        availableInterfaces,
        saveBindAddress,
        load,
    };
};
