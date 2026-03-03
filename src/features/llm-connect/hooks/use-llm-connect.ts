import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect, useCallback, useRef } from 'react';
import { useTranslation } from '@/i18n';
import { toast } from 'react-toastify';

export type LLMProvider = 'local' | 'remote';

export interface LLMMode {
    name: string;
    prompt: string;
    model: string;
    shortcut: string;
    provider: LLMProvider;
}

export interface LLMConnectSettings {
    url: string;
    model: string;
    prompt: string;
    modes: LLMMode[];
    active_mode_index: number;
    onboarding_completed: boolean;
    remote_url: string;
    remote_privacy_acknowledged: boolean;
}

export interface OllamaModel {
    name: string;
}

export type ConnectionStatus =
    | 'disconnected'
    | 'connected'
    | 'testing'
    | 'error';

export const useLLMConnect = () => {
    const { t } = useTranslation();
    const [settings, setSettings] = useState<LLMConnectSettings>({
        url: 'http://localhost:11434/api',
        model: '',
        prompt: '',
        modes: [],
        active_mode_index: 0,
        onboarding_completed: false,
        remote_url: '',
        remote_privacy_acknowledged: false,
    });
    const [models, setModels] = useState<OllamaModel[]>([]);
    const [connectionStatus, setConnectionStatus] =
        useState<ConnectionStatus>('disconnected');
    const [remoteModels, setRemoteModels] = useState<OllamaModel[]>([]);
    const [remoteConnectionStatus, setRemoteConnectionStatus] =
        useState<ConnectionStatus>('disconnected');
    const [isLoading, setIsLoading] = useState(false);
    const [isSettingsLoaded, setIsSettingsLoaded] = useState(false);

    // Load settings on mount
    useEffect(() => {
        loadSettings();
    }, []);

    // Listen for LLM errors from backend
    useEffect(() => {
        const unlisten = listen<string>('llm-error', (event) => {
            toast.error(t('LLM processing failed') + ' : ' + event.payload, {
                autoClose: 5000,
            });
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    useEffect(() => {
        const unlisten = listen<LLMConnectSettings>(
            'llm-settings-updated',
            (event) => {
                setSettings(event.payload);
            }
        );

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    const loadSettings = async () => {
        try {
            const loadedSettings = await invoke<LLMConnectSettings>(
                'get_llm_connect_settings'
            );
            setSettings(loadedSettings);
            setIsSettingsLoaded(true);

            // Test connection and fetch models if url is present
            if (loadedSettings.url) {
                const connected = await testConnection(loadedSettings.url);
                if (connected) {
                    await fetchModels(loadedSettings.url);
                }
            }

            // Test remote connection and fetch remote models if remote_url is present
            if (loadedSettings.remote_url) {
                const remoteConnected = await testRemoteConnection(
                    loadedSettings.remote_url
                );
                if (remoteConnected) {
                    await fetchRemoteModels(loadedSettings.remote_url);
                }
            }
        } catch (error) {
            console.error('Failed to load LLM Connect settings:', error);
            setIsSettingsLoaded(true);
        }
    };

    const saveSettings = useCallback(async (newSettings: LLMConnectSettings) => {
        try {
            await invoke('set_llm_connect_settings', { settings: newSettings });
            setSettings(newSettings);
        } catch (error) {
            console.error('Failed to save LLM Connect settings:', error);
            throw error;
        }
    }, []);

    const testConnection = useCallback(
        async (url?: string) => {
            const testUrl = url || settings.url;
            setConnectionStatus('testing');

            try {
                const result = await invoke<boolean>('test_llm_connection', {
                    url: testUrl,
                });
                setConnectionStatus(result ? 'connected' : 'error');
                return result;
            } catch (error) {
                console.error('Connection test failed:', error);
                setConnectionStatus('error');
                return false;
            }
        },
        [settings.url]
    );

    const testRemoteConnection = useCallback(
        async (url?: string): Promise<number> => {
            const testUrl = url || settings.remote_url;
            setRemoteConnectionStatus('testing');

            try {
                const modelCount = await invoke<number>(
                    'test_remote_connection',
                    { url: testUrl }
                );
                setRemoteConnectionStatus('connected');
                return modelCount;
            } catch (error) {
                console.error('Remote connection test failed:', error);
                setRemoteConnectionStatus('error');
                throw error;
            }
        },
        [settings.remote_url]
    );

    const fetchRemoteModels = useCallback(
        async (url?: string): Promise<OllamaModel[]> => {
            const fetchUrl = url || settings.remote_url;
            setIsLoading(true);

            try {
                const fetchedModels = await invoke<OllamaModel[]>(
                    'fetch_remote_models',
                    { url: fetchUrl }
                );
                setRemoteModels(fetchedModels);
                setRemoteConnectionStatus('connected');
                return fetchedModels;
            } catch (error) {
                console.error('Failed to fetch remote models:', error);
                setRemoteConnectionStatus('error');
                setRemoteModels([]);
                throw error;
            } finally {
                setIsLoading(false);
            }
        },
        [settings.remote_url]
    );

    const storeRemoteApiKey = useCallback(async (apiKey: string) => {
        try {
            await invoke('store_remote_api_key', { apiKey });
        } catch (error) {
            console.error('Failed to store remote API key:', error);
            throw error;
        }
    }, []);

    const fetchModels = useCallback(
        async (url?: string) => {
            const fetchUrl = url || settings.url;
            setIsLoading(true);

            try {
                const fetchedModels = await invoke<OllamaModel[]>(
                    'fetch_ollama_models',
                    { url: fetchUrl }
                );
                setModels(fetchedModels);
                setConnectionStatus('connected');
                return fetchedModels;
            } catch (error) {
                console.error('Failed to fetch models:', error);
                setConnectionStatus('error');
                setModels([]);
                throw error;
            } finally {
                setIsLoading(false);
            }
        },
        [settings.url]
    );

    const pullModel = useCallback(
        async (model: string) => {
            try {
                await invoke('pull_ollama_model', {
                    url: settings.url,
                    model,
                });
            } catch (error) {
                console.error('Failed to pull model:', error);
                throw error;
            }
        },
        [settings.url]
    );

    const completeOnboarding = async () => {
        await updateSettings({ onboarding_completed: true });
    };

    const settingsRef = useRef(settings);
    settingsRef.current = settings;

    const updateSettings = useCallback(async (updates: Partial<LLMConnectSettings>) => {
        const newSettings = { ...settingsRef.current, ...updates };
        await saveSettings(newSettings);
    }, [saveSettings]);

    return {
        settings,
        models,
        connectionStatus,
        remoteModels,
        remoteConnectionStatus,
        isLoading,
        isSettingsLoaded,
        loadSettings,
        saveSettings,
        updateSettings,
        testConnection,
        testRemoteConnection,
        fetchModels,
        fetchRemoteModels,
        storeRemoteApiKey,
        pullModel,
        completeOnboarding,
    };
};
