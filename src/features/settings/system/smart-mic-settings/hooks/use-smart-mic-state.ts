import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

interface PairedDevice {
    token: string;
    name: string;
    last_connected: string;
}

export const useSmartMicState = () => {
    const [smartMicEnabled, setSmartMicEnabled] = useState<boolean>(false);
    const [smartMicPort, setSmartMicPort] = useState<number>(4801);
    const [qrCodeDataUri, setQrCodeDataUri] = useState<string>('');
    const [pairedDevices, setPairedDevices] = useState<PairedDevice[]>([]);
    const { t } = useTranslation();

    const loadSmartMicState = async () => {
        try {
            const enabled = await invoke<boolean>('get_smartmic_enabled');
            const port = await invoke<number>('get_smartmic_port');
            setSmartMicEnabled(enabled);
            setSmartMicPort(port);

            if (enabled) {
                await loadQrCode();
                await loadPairedDevices();
            }
        } catch (error) {
            console.error('Failed to load Smart Mic state:', error);
        }
    };

    const loadQrCode = async () => {
        try {
            const dataUri = await invoke<string>('get_smartmic_qr_code');
            setQrCodeDataUri(dataUri);
        } catch (error) {
            console.error('Failed to load Smart Mic QR code:', error);
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
        loadSmartMicState();
    }, []);

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
                    try {
                        await invoke('stop_smartmic_server');
                        await invoke('start_smartmic_server');
                        await loadQrCode();
                    } catch (error) {
                        console.error('Failed to restart Smart Mic server with new port:', error);
                        toast.error(t('Failed to restart Smart Mic server'));
                    }
                }
            } catch (error) {
                console.error('Failed to set Smart Mic port:', error);
                toast.error(t('Failed to save Smart Mic port'));
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

    return {
        smartMicEnabled,
        smartMicPort,
        qrCodeDataUri,
        pairedDevices,
        setSmartMicEnabled: handleSetSmartMicEnabled,
        setSmartMicPort: handleSetSmartMicPort,
        removePairedDevice: handleRemovePairedDevice,
        resetTokens: handleResetTokens,
    };
};
