import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { LLMConnectSettings, LLMMode } from './use-llm-connect';

export interface LlmExtensionsEnabled {
    isLoaded: boolean;
    llmConnectEnabled: boolean;
    commandModeEnabled: boolean;
    modes: LLMMode[];
}

const DISABLED: LlmExtensionsEnabled = {
    isLoaded: false,
    llmConnectEnabled: false,
    commandModeEnabled: false,
    modes: [],
};

export const useLlmExtensionsEnabled = () => {
    const [extensionsEnabled, setExtensionsEnabled] = useState<LlmExtensionsEnabled>(DISABLED);

    useEffect(() => {
        invoke<LLMConnectSettings>('get_llm_connect_settings')
            .then((settings) => setExtensionsEnabled(toExtensionsEnabled(settings)))
            .catch((error) => {
                console.error('Failed to load Prompt Mode settings:', error);
                setExtensionsEnabled({ ...DISABLED, isLoaded: true });
            });
    }, []);

    return extensionsEnabled;
};

const toExtensionsEnabled = (settings: LLMConnectSettings) => ({
    isLoaded: true,
    llmConnectEnabled: settings.onboarding_completed && settings.enabled,
    commandModeEnabled: settings.onboarding_completed && settings.command.enabled,
    modes: settings.modes,
});
