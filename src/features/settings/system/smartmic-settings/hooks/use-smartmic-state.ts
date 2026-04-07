import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

interface PairedDevice {
    token: string;
    name: string;
    last_connected: string;
}

export const useSmartmicState = () => {
    const [smartmicEnabled, setSmartmicEnabled] = useState<boolean>(false);
    const [smartmicPort, setSmartmicPort] = useState<number>(4801);
    const [qrCodeDataUri, setQrCodeDataUri] = useState<string>('');
    const [pairedDevices, setPairedDevices] = useState<PairedDevice[]>([]);
    const { t } = useTranslation();

    const loadSmartmicState = async () => {
        try {
            const enabled = await invoke<boolean>('get_smartmic_enabled');
            const port = await invoke<number>('get_smartmic_port');
            setSmartmicEnabled(enabled);
            setSmartmicPort(port);

            if (enabled) {
                await loadQrCode();
                await loadPairedDevices();
            }
        } catch (error) {
            console.error('Failed to load SmartMic state:', error);
        }
    };

    const loadQrCode = async () => {
        try {
            const dataUri = await invoke<string>('get_smartmic_qr_code');
            setQrCodeDataUri(dataUri);
        } catch (error) {
            console.error('Failed to load SmartMic QR code:', error);
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
        loadSmartmicState();
    }, []);

    const handleSetSmartmicEnabled = async (enabled: boolean) => {
        try {
            await invoke('set_smartmic_enabled', { enabled });
            setSmartmicEnabled(enabled);

            if (enabled) {
                await invoke('start_smartmic_server');
                await loadQrCode();
                await loadPairedDevices();
            } else {
                await invoke('stop_smartmic_server');
                setQrCodeDataUri('');
            }
        } catch (error) {
            console.error('Failed to toggle SmartMic:', error);
            toast.error(t('Failed to toggle SmartMic'));
            // Reload actual state from backend
            loadSmartmicState();
        }
    };

    const handleSetSmartmicPort = async (port: number) => {
        if (port >= 1024 && port <= 65535) {
            try {
                setSmartmicPort(port);
                await invoke('set_smartmic_port', { port });

                if (smartmicEnabled) {
                    try {
                        await invoke('stop_smartmic_server');
                        await invoke('start_smartmic_server');
                        await loadQrCode();
                    } catch (error) {
                        console.error('Failed to restart SmartMic server with new port:', error);
                        toast.error(t('Failed to restart SmartMic server'));
                    }
                }
            } catch (error) {
                console.error('Failed to set SmartMic port:', error);
                toast.error(t('Failed to save SmartMic port'));
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
        smartmicEnabled,
        smartmicPort,
        qrCodeDataUri,
        pairedDevices,
        setSmartmicEnabled: handleSetSmartmicEnabled,
        setSmartmicPort: handleSetSmartmicPort,
        removePairedDevice: handleRemovePairedDevice,
    };
};
