import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect, useCallback } from 'react';

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

    const loadSettings = async () => {
        try {
            const loadedSettings = await invoke<LLMConnectSettings>('get_llm_connect_settings');
            setSettings(loadedSettings);
            
            // If enabled, test connection
            if (loadedSettings.enabled && loadedSettings.url) {
                testConnection(loadedSettings.url);
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
