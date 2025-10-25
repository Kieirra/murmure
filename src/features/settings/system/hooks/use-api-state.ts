import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';

export const useApiState = () => {
    const [apiEnabled, setApiEnabled] = useState<boolean>(false);
    const [apiPort, setApiPort] = useState<number>(4800);

    useEffect(() => {
        invoke<boolean>('get_api_enabled').then((enabled) => {
            setApiEnabled(enabled);
        });

        invoke<number>('get_api_port').then((port) => {
            setApiPort(port);
        });
    }, []);

    return {
        setApiEnabled: async (enabled: boolean) => {
            setApiEnabled(enabled);
            await invoke('set_api_enabled', { enabled });

            // Start the HTTP API server immediately when enabled
            if (enabled) {
                try {
                    await invoke('start_http_api_server');
                } catch (error) {
                    console.error('Failed to start HTTP API server:', error);
                }
            }
        },
        setApiPort: (port: number) => {
            if (port >= 1024 && port <= 65535) {
                setApiPort(port);
                invoke('set_api_port', { port });
            }
        },
        apiEnabled,
        apiPort,
    };
};
