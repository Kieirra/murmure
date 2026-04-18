import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect, useCallback } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

interface PairedDevice {
    token: string;
    name: string;
    last_connected: string;
    created_at: string;
}

export interface NetworkInterface {
    name: string;
    ip: string;
}

export const useSmartMicState = () => {
    const [smartMicEnabled, setSmartMicEnabled] = useState<boolean | null>(null);
    const [smartMicPort, setSmartMicPort] = useState<number>(4801);
    const [qrCodeDataUri, setQrCodeDataUri] = useState<string>('');
    const [pairedDevices, setPairedDevices] = useState<PairedDevice[]>([]);
    const [relayUrl, setRelayUrl] = useState<string>('');
    const [machineId, setMachineId] = useState<string>('');
    const [machineHostname, setMachineHostname] = useState<string>('');
    const [machineIdEnabled, setMachineIdEnabled] = useState<boolean>(false);
    const [tokenTtlHours, setTokenTtlHours] = useState<number>(0);
    const [bindAddress, setBindAddressState] = useState<string | null>(null);
    const [availableInterfaces, setAvailableInterfaces] = useState<NetworkInterface[]>([]);
    const [isAdvancedOpen, setIsAdvancedOpen] = useState<boolean>(false);
    const { t } = useTranslation();

    const loadQrCode = useCallback(async () => {
        try {
            const dataUri = await invoke<string>('get_smartmic_qr_code');
            setQrCodeDataUri(dataUri);
        } catch (error) {
            console.error('Failed to load Smart Mic QR code:', error);
        }
    }, []);

    const loadPairedDevices = useCallback(async () => {
        try {
            const devices = await invoke<PairedDevice[]>('get_paired_devices');
            setPairedDevices(devices);
        } catch (error) {
            console.error('Failed to load paired devices:', error);
        }
    }, []);

    const loadSmartMicState = useCallback(async () => {
        try {
            const enabled = await invoke<boolean>('get_smartmic_enabled');
            const port = await invoke<number>('get_smartmic_port');
            const relay = await invoke<string | null>('get_smartmic_relay_url');
            const machine = await invoke<string | null>('get_smartmic_machine_id');
            const machineIdEnabledVal = await invoke<boolean>('get_smartmic_machine_id_enabled');
            const ttl = await invoke<number | null>('get_smartmic_token_ttl_hours');
            const hostname = await invoke<string>('get_machine_hostname');
            const bindAddressValue = await invoke<string | null>('get_smartmic_bind_address');
            const interfaces = await invoke<NetworkInterface[]>('list_smartmic_network_interfaces');

            setSmartMicEnabled(enabled);
            setSmartMicPort(port);
            setRelayUrl(relay ?? '');
            setMachineId(machine ?? '');
            setMachineIdEnabled(machineIdEnabledVal);
            setMachineHostname(hostname);
            setTokenTtlHours(ttl ?? 0);
            setBindAddressState(bindAddressValue);
            setAvailableInterfaces(interfaces);

            if (enabled) {
                await loadQrCode();
                await loadPairedDevices();
            }
        } catch (error) {
            console.error('Failed to load Smart Mic state:', error);
        }
    }, [loadQrCode, loadPairedDevices]);

    useEffect(() => {
        loadSmartMicState();
    }, [loadSmartMicState]);

    const restartServerAndReloadQr = async () => {
        try {
            await invoke('stop_smartmic_server');
            await invoke('start_smartmic_server');
            await loadQrCode();
        } catch (error) {
            console.error('Failed to restart Smart Mic server:', error);
            toast.error(t('Failed to restart Smart Mic server'));
        }
    };

    const handleSetSmartMicEnabled = async (enabled: boolean) => {
        try {
            await invoke('set_smartmic_enabled', { enabled });
            setSmartMicEnabled(enabled);

            if (enabled) {
                await invoke('start_smartmic_server');
                await loadQrCode();
                await loadPairedDevices();
            } else {
                await invoke('stop_smartmic_server');
                setQrCodeDataUri('');
            }
        } catch (error) {
            console.error('Failed to toggle Smart Mic:', error);
            toast.error(t('Failed to toggle Smart Mic'));
            await invoke('set_smartmic_enabled', { enabled: false });
            setSmartMicEnabled(false);
            loadSmartMicState();
        }
    };

    const handleSetSmartMicPort = async (port: number) => {
        if (port >= 1024 && port <= 65535) {
            try {
                setSmartMicPort(port);
                await invoke('set_smartmic_port', { port });

                if (smartMicEnabled) {
                    await restartServerAndReloadQr();
                }
            } catch (error) {
                console.error('Failed to set Smart Mic port:', error);
                toast.error(t('Failed to save Smart Mic port'));
            }
        }
    };

    const handleRelayUrlBlur = async () => {
        try {
            const value = relayUrl.trim();
            await invoke('set_smartmic_relay_url', { url: value || null });
            if (smartMicEnabled) {
                await restartServerAndReloadQr();
            }
        } catch (error) {
            console.error('Failed to save relay URL:', error);
            toast.error(t('Failed to save relay URL'));
        }
    };

    const handleMachineIdEnabledChange = async (enabled: boolean) => {
        try {
            setMachineIdEnabled(enabled);
            await invoke('set_smartmic_machine_id_enabled', { enabled });
            if (enabled && machineId.length === 0) {
                setMachineId(machineHostname);
                await invoke('set_smartmic_machine_id', { id: machineHostname || null });
            }
            if (smartMicEnabled) {
                await loadQrCode();
            }
        } catch (error) {
            console.error('Failed to toggle machine ID:', error);
            toast.error(t('Failed to toggle machine ID'));
        }
    };

    const handleMachineIdBlur = async () => {
        try {
            const value = machineId.trim();
            await invoke('set_smartmic_machine_id', { id: value || null });
            if (smartMicEnabled) {
                await loadQrCode();
            }
        } catch (error) {
            console.error('Failed to save machine ID:', error);
            toast.error(t('Failed to save machine ID'));
        }
    };

    const handleSetBindAddress = async (value: string | null) => {
        try {
            setBindAddressState(value);
            await invoke('set_smartmic_bind_address', { address: value });

            if (smartMicEnabled) {
                await restartServerAndReloadQr();
            }
        } catch (error) {
            console.error('Failed to set Smart Mic bind address:', error);
            toast.error(t('Failed to save Smart Mic bind address'));
        }
    };

    const handleTokenTtlChange = async (value: number = 0) => {
        setTokenTtlHours(value);
        try {
            await invoke('set_smartmic_token_ttl_hours', { hours: value > 0 ? value : null });
        } catch (error) {
            console.error('Failed to save token TTL:', error);
            toast.error(t('Failed to save token expiration'));
        }
    };

    const handleRemovePairedDevice = async (token: string) => {
        try {
            await invoke('remove_paired_device', { token });
            await loadPairedDevices();
        } catch (error) {
            console.error('Failed to remove paired device:', error);
            toast.error(t('Failed to remove device'));
        }
    };

    const handleResetTokens = async () => {
        try {
            await invoke('reset_smartmic_tokens');
            await loadQrCode();
            await loadPairedDevices();
        } catch (error) {
            console.error('Failed to reset SmartMic tokens:', error);
            toast.error(t('Failed to reset QR code'));
        }
    };

    const toggleAdvanced = () => setIsAdvancedOpen((prev) => !prev);

    return {
        smartMicEnabled,
        smartMicPort,
        qrCodeDataUri,
        pairedDevices,
        relayUrl,
        setRelayUrl,
        machineId,
        setMachineId,
        machineIdEnabled,
        setMachineIdEnabled: handleMachineIdEnabledChange,
        machineHostname,
        tokenTtlHours,
        bindAddress,
        availableInterfaces,
        setBindAddress: handleSetBindAddress,
        isAdvancedOpen,
        toggleAdvanced,
        setSmartMicEnabled: handleSetSmartMicEnabled,
        setSmartMicPort: handleSetSmartMicPort,
        handleRelayUrlBlur,
        handleMachineIdBlur,
        handleTokenTtlChange,
        removePairedDevice: handleRemovePairedDevice,
        resetTokens: handleResetTokens,
    };
};

export type UseSmartMicStateReturn = ReturnType<typeof useSmartMicState>;
