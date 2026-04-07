import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

interface PairedDevice {
    token: string;
    name: string;
    last_connected: string;
}

export const useVirtualMicState = () => {
    const [virtualMicEnabled, setVirtualMicEnabled] = useState<boolean>(false);
    const [virtualMicPort, setVirtualMicPort] = useState<number>(4801);
    const [qrCodeDataUri, setQrCodeDataUri] = useState<string>('');
    const [pairedDevices, setPairedDevices] = useState<PairedDevice[]>([]);
    const { t } = useTranslation();

    const loadVirtualMicState = async () => {
        try {
            const enabled = await invoke<boolean>('get_smartmic_enabled');
            const port = await invoke<number>('get_smartmic_port');
            setVirtualMicEnabled(enabled);
            setVirtualMicPort(port);

            if (enabled) {
                await loadQrCode();
                await loadPairedDevices();
            }
        } catch (error) {
            console.error('Failed to load Virtual Mic state:', error);
        }
    };

    const loadQrCode = async () => {
        try {
            const dataUri = await invoke<string>('get_smartmic_qr_code');
            setQrCodeDataUri(dataUri);
        } catch (error) {
            console.error('Failed to load Virtual Mic QR code:', error);
        }
    };

    const loadPairedDevices = async () => {
        try {
            const devices = await invoke<PairedDevice[]>('get_paired_devices');
            setPairedDevices(devices);
        } catch (error) {
            console.error('Failed to load paired devices:', error);
        }
    };

    useEffect(() => {
        loadVirtualMicState();
    }, []);

    const handleSetVirtualMicEnabled = async (enabled: boolean) => {
        try {
            await invoke('set_smartmic_enabled', { enabled });
            setVirtualMicEnabled(enabled);

            if (enabled) {
                await invoke('start_smartmic_server');
                await loadQrCode();
                await loadPairedDevices();
            } else {
                await invoke('stop_smartmic_server');
                setQrCodeDataUri('');
            }
        } catch (error) {
            console.error('Failed to toggle Virtual Mic:', error);
            toast.error(t('Failed to toggle Virtual Mic'));
            loadVirtualMicState();
        }
    };

    const handleSetVirtualMicPort = async (port: number) => {
        if (port >= 1024 && port <= 65535) {
            try {
                setVirtualMicPort(port);
                await invoke('set_smartmic_port', { port });

                if (virtualMicEnabled) {
                    try {
                        await invoke('stop_smartmic_server');
                        await invoke('start_smartmic_server');
                        await loadQrCode();
                    } catch (error) {
                        console.error('Failed to restart Virtual Mic server with new port:', error);
                        toast.error(t('Failed to restart Virtual Mic server'));
                    }
                }
            } catch (error) {
                console.error('Failed to set Virtual Mic port:', error);
                toast.error(t('Failed to save Virtual Mic port'));
            }
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

    return {
        virtualMicEnabled,
        virtualMicPort,
        qrCodeDataUri,
        pairedDevices,
        setVirtualMicEnabled: handleSetVirtualMicEnabled,
        setVirtualMicPort: handleSetVirtualMicPort,
        removePairedDevice: handleRemovePairedDevice,
    };
};
