import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useApiState = () => {
    const [apiEnabled, setApiEnabled] = useState<boolean>(false);
    const [apiPort, setApiPort] = useState<number>(4800);
    const { t } = useTranslation();

    const loadApiState = async () => {
        try {
            const enabled = await invoke<boolean>('get_api_enabled');
            const port = await invoke<number>('get_api_port');
            setApiEnabled(enabled);
            setApiPort(port);
        } catch (error) {
            console.error('Failed to load API state:', error);
        }
    };

    useEffect(() => {
        loadApiState();
    }, []);

    const handleSetApiEnabled = async (enabled: boolean) => {
        try {
            setApiEnabled(enabled);
            await invoke('set_api_enabled', { enabled });

            if (enabled) {
                // Start the HTTP API server immediately when enabled
                try {
                    await invoke('start_http_api_server');
                } catch (error) {
                    console.error('Failed to start HTTP API server:', error);
                    toast.error(t('Failed to start API server'));
                    // Revert the state on error
                    setApiEnabled(false);
                }
            } else {
                // Stop the HTTP API server when disabled
                try {
                    await invoke('stop_http_api_server');
                } catch (error) {
                    console.error('Failed to stop HTTP API server:', error);
                    toast.error(t('Failed to stop API server'));
                }
            }
        } catch (error) {
            console.error('Failed to set API enabled:', error);
            toast.error(t('Failed to toggle API'));
            // Revert the state on error
            setApiEnabled(!enabled);
        }
    };

    const handleSetApiPort = async (port: number) => {
        if (port >= 1024 && port <= 65535) {
            try {
                setApiPort(port);
                await invoke('set_api_port', { port });

                if (apiEnabled) {
                    try {
                        await invoke('stop_http_api_server');
                        await new Promise((resolve) => setTimeout(resolve, 100));
                        await invoke('start_http_api_server');
                    } catch (error) {
                        console.error('Failed to restart HTTP API server with new port:', error);
                        toast.error(t('Failed to restart API server'));
                    }
                }
            } catch (error) {
                console.error('Failed to set API port:', error);
                toast.error(t('Failed to save API port'));
            }
        }
    };

    return {
        setApiEnabled: handleSetApiEnabled,
        setApiPort: handleSetApiPort,
        apiEnabled,
        apiPort,
    };
};
