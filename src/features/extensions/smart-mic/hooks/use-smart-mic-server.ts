import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect, useCallback } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useSmartMicServer = () => {
    const [enabled, setEnabledState] = useState<boolean | null>(null);
    const [port, setPortState] = useState<number>(4801);
    const [qrCodeDataUri, setQrCodeDataUri] = useState<string>('');
    const { t } = useTranslation();

    const loadQrCode = useCallback(async () => {
        try {
            const dataUri = await invoke<string>('get_smartmic_qr_code');
            setQrCodeDataUri(dataUri);
        } catch (error) {
            console.error('Failed to load Smart Mic QR code:', error);
        }
    }, []);

    const load = useCallback(async () => {
        try {
            const enabledValue = await invoke<boolean>('get_smartmic_enabled');
            const portValue = await invoke<number>('get_smartmic_port');
            setEnabledState(enabledValue);
            setPortState(portValue);
            if (enabledValue) {
                await loadQrCode();
            }
            return enabledValue;
        } catch (error) {
            console.error('Failed to load Smart Mic server state:', error);
            return null;
        }
    }, [loadQrCode]);

    useEffect(() => {
        load();
    }, [load]);

    const restart = useCallback(async () => {
        try {
            await invoke('stop_smartmic_server');
            await invoke('start_smartmic_server');
            await loadQrCode();
        } catch (error) {
            console.error('Failed to restart Smart Mic server:', error);
            toast.error(t('Failed to restart Smart Mic server'));
        }
    }, [loadQrCode, t]);

    const start = useCallback(async () => {
        await invoke('start_smartmic_server');
        await loadQrCode();
    }, [loadQrCode]);

    const stop = useCallback(async () => {
        await invoke('stop_smartmic_server');
        setQrCodeDataUri('');
    }, []);

    const setEnabled = useCallback(
        async (value: boolean) => {
            try {
                await invoke('set_smartmic_enabled', { enabled: value });
                setEnabledState(value);

                if (value) {
                    await invoke('start_smartmic_server');
                    await loadQrCode();
                } else {
                    await invoke('stop_smartmic_server');
                    setQrCodeDataUri('');
                }
            } catch (error) {
                console.error('Failed to toggle Smart Mic:', error);
                toast.error(t('Failed to toggle Smart Mic'));
                await invoke('set_smartmic_enabled', { enabled: false });
                setEnabledState(false);
                load();
            }
        },
        [loadQrCode, load, t]
    );

    const setPort = useCallback(
        async (value: number) => {
            if (value < 1024 || value > 65535) {
                return;
            }
            try {
                setPortState(value);
                await invoke('set_smartmic_port', { port: value });

                if (enabled) {
                    await restart();
                }
            } catch (error) {
                console.error('Failed to set Smart Mic port:', error);
                toast.error(t('Failed to save Smart Mic port'));
            }
        },
        [enabled, restart, t]
    );

    return {
        enabled,
        port,
        qrCodeDataUri,
        start,
        stop,
        restart,
        setPort,
        setEnabled,
        loadQrCode,
    };
};
