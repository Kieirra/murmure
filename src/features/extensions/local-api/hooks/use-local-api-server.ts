import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import type { LocalApiStatus } from '../local-api.types';
import { DEFAULT_API_PORT, MAX_API_PORT, MIN_API_PORT } from '../local-api.helpers';

export const useLocalApiServer = () => {
    const [enabled, setEnabled] = useState<boolean | null>(null);
    const [port, setPort] = useState<number>(DEFAULT_API_PORT);
    const [status, setStatus] = useState<LocalApiStatus | null>(null);
    const [isStarting, setIsStarting] = useState(false);
    const { t } = useTranslation();

    const loadStatus = async () => {
        try {
            setStatus(await invoke<LocalApiStatus>('get_http_api_status'));
        } catch (error) {
            console.error('Failed to load HTTP API status:', error);
        }
    };

    useEffect(() => {
        const load = async () => {
            try {
                const enabledValue = await invoke<boolean>('get_api_enabled');
                const portValue = await invoke<number>('get_api_port');
                setEnabled(enabledValue);
                setPort(portValue);
            } catch (error) {
                console.error('Failed to load API state:', error);
            }
            await loadStatus();
        };
        load();
    }, []);

    const runStartAction = async (action: () => Promise<void>) => {
        setIsStarting(true);
        try {
            await action();
        } catch (error) {
            console.error('Failed to start HTTP API server:', error);
        }
        await loadStatus();
        setIsStarting(false);
    };

    const start = () => runStartAction(() => invoke('start_http_api_server'));

    const restart = () =>
        runStartAction(async () => {
            await invoke('stop_http_api_server');
            await invoke('start_http_api_server');
        });

    const stop = async () => {
        try {
            await invoke('stop_http_api_server');
        } catch (error) {
            console.error('Failed to stop HTTP API server:', error);
        }
        await loadStatus();
    };

    const saveEnabled = async (value: boolean) => {
        try {
            await invoke('set_api_enabled', { enabled: value });
        } catch (error) {
            console.error('Failed to set API enabled:', error);
            toast.error(t('Failed to toggle API'));
            return;
        }
        setEnabled(value);
        if (value) {
            await start();
        } else {
            await stop();
        }
    };

    const savePort = async (value: number) => {
        if (value < MIN_API_PORT || value > MAX_API_PORT) {
            return;
        }
        setPort(value);
        try {
            await invoke('set_api_port', { port: value });
        } catch (error) {
            console.error('Failed to set API port:', error);
            toast.error(t('Failed to save API port'));
            return;
        }
        if (enabled === true) {
            await restart();
        }
    };

    return { enabled, port, status, isStarting, saveEnabled, savePort, retry: start };
};
