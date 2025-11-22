import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import { toast } from 'react-toastify';

export interface LLMConnectSettings {
    enabled: boolean;
    url: string;
    model: string;
    prompt: string;
}

export interface OllamaModel {
    name: string;
}

export type ConnectionStatus = 'disconnected' | 'connected' | 'testing' | 'error';

export const useLLMConnect = () => {
    const { t } = useTranslation();
    const [settings, setSettings] = useState<LLMConnectSettings>({
        enabled: false,
        url: 'http://localhost:11434/api',
        model: '',
        prompt: `You are an ASR (Automatic Speech Recognition) post-processor. Your task is to correct the following transcription by fixing grammar, punctuation, and spelling errors. Return ONLY the corrected text, nothing else.

Transcription: {{TRANSCRIPT}}`,
    });
    const [models, setModels] = useState<OllamaModel[]>([]);
    const [connectionStatus, setConnectionStatus] = useState<ConnectionStatus>('disconnected');
    const [isLoading, setIsLoading] = useState(false);

    // Load settings on mount
    useEffect(() => {
        loadSettings();
    }, []);

    // Listen for LLM errors from backend
    useEffect(() => {
        const unlisten = listen<string>('llm-error', (event) => {
            toast.error(t('LLM processing failed') + ' : ' + event.payload);
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    const loadSettings = async () => {
        try {
            const loadedSettings = await invoke<LLMConnectSettings>('get_llm_connect_settings');
            setSettings(loadedSettings);
            
            // If enabled, test connection and fetch models
            if (loadedSettings.enabled && loadedSettings.url) {
                const connected = await testConnection(loadedSettings.url);
                if (connected) {
                    await fetchModels(loadedSettings.url);
                }
            }
        } catch (error) {
            console.error('Failed to load LLM Connect settings:', error);
        }
    };

    const saveSettings = async (newSettings: LLMConnectSettings) => {
        try {
            await invoke('set_llm_connect_settings', { settings: newSettings });
            setSettings(newSettings);
        } catch (error) {
            console.error('Failed to save LLM Connect settings:', error);
            throw error;
        }
    };

    const testConnection = useCallback(async (url?: string) => {
        const testUrl = url || settings.url;
        setConnectionStatus('testing');
        
        try {
            const result = await invoke<boolean>('test_llm_connection', { url: testUrl });
            setConnectionStatus(result ? 'connected' : 'error');
            return result;
        } catch (error) {
            console.error('Connection test failed:', error);
            setConnectionStatus('error');
            return false;
        }
    }, [settings.url]);

    const fetchModels = useCallback(async (url?: string) => {
        const fetchUrl = url || settings.url;
        setIsLoading(true);
        
        try {
            const fetchedModels = await invoke<OllamaModel[]>('fetch_ollama_models', { url: fetchUrl });
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
    }, [settings.url]);

    const updateSettings = async (updates: Partial<LLMConnectSettings>) => {
        const newSettings = { ...settings, ...updates };
        await saveSettings(newSettings);
    };

    return {
        settings,
        models,
        connectionStatus,
        isLoading,
        loadSettings,
        saveSettings,
        updateSettings,
        testConnection,
        fetchModels,
    };
};
