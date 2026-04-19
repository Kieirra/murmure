import { invoke } from '@tauri-apps/api/core';
import { useState, useCallback, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

interface UseRelayConfigOptions {
    enabled: boolean;
    onChange: () => Promise<void> | void;
    onMachineIdBlurChange: () => Promise<void> | void;
}

export const useRelayConfig = ({ enabled, onChange, onMachineIdBlurChange }: UseRelayConfigOptions) => {
    const [relayEnabled, setRelayEnabledState] = useState<boolean>(false);
    const [relayUrl, setRelayUrl] = useState<string>('');
    const [machineIdEnabled, setMachineIdEnabledState] = useState<boolean>(false);
    const [machineId, setMachineId] = useState<string>('');
    const { t } = useTranslation();

    const load = useCallback(async () => {
        try {
            const relayOn = await invoke<boolean>('get_smartmic_relay_enabled');
            const relay = await invoke<string | null>('get_smartmic_relay_url');
            const machineIdOn = await invoke<boolean>('get_smartmic_machine_id_enabled');
            const machine = await invoke<string | null>('get_smartmic_machine_id');
            setRelayEnabledState(relayOn);
            setRelayUrl(relay ?? '');
            setMachineIdEnabledState(machineIdOn);
            setMachineId(machine ?? '');
        } catch (error) {
            console.error('Failed to load relay config:', error);
        }
    }, []);

    useEffect(() => {
        load();
    }, [load]);

    const setRelayEnabled = useCallback(
        async (value: boolean) => {
            try {
                setRelayEnabledState(value);
                await invoke('set_smartmic_relay_enabled', { enabled: value });

                if (enabled) {
                    await onChange();
                }
            } catch (error) {
                console.error('Failed to toggle relay:', error);
                toast.error(t('Failed to toggle relay'));
            }
        },
        [enabled, onChange, t]
    );

    const setMachineIdEnabled = useCallback(
        async (value: boolean) => {
            try {
                setMachineIdEnabledState(value);
                await invoke('set_smartmic_machine_id_enabled', { enabled: value });

                if (value && machineId.trim().length === 0) {
                    const hostname = await invoke<string>('get_smartmic_hostname');
                    setMachineId(hostname);
                    await invoke('set_smartmic_machine_id', { id: hostname || null });
                }

                if (enabled) {
                    await onChange();
                }
            } catch (error) {
                console.error('Failed to toggle machine ID:', error);
                toast.error(t('Failed to toggle machine ID'));
            }
        },
        [enabled, machineId, onChange, t]
    );

    const handleRelayUrlBlur = useCallback(async () => {
        try {
            const value = relayUrl.trim();
            await invoke('set_smartmic_relay_url', { url: value || null });
            if (enabled) {
                await onChange();
            }
        } catch (error) {
            console.error('Failed to save relay URL:', error);
            toast.error(t('Failed to save relay URL'));
        }
    }, [enabled, onChange, relayUrl, t]);

    const handleMachineIdBlur = useCallback(async () => {
        try {
            const value = machineId.trim();
            await invoke('set_smartmic_machine_id', { id: value || null });
            if (enabled) {
                await onMachineIdBlurChange();
            }
        } catch (error) {
            console.error('Failed to save machine ID:', error);
            toast.error(t('Failed to save machine ID'));
        }
    }, [enabled, machineId, onMachineIdBlurChange, t]);

    return {
        relayEnabled,
        relayUrl,
        machineIdEnabled,
        machineId,
        setRelayEnabled,
        setRelayUrl,
        setMachineIdEnabled,
        setMachineId,
        handleRelayUrlBlur,
        handleMachineIdBlur,
        load,
    };
};
