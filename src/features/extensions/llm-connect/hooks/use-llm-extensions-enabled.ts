import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { LLMConnectSettings } from './use-llm-connect';

interface LlmExtensionsEnabled {
    llmConnectEnabled: boolean;
    commandModeEnabled: boolean;
}

const DISABLED: LlmExtensionsEnabled = { llmConnectEnabled: false, commandModeEnabled: false };

export const useLlmExtensionsEnabled = () => {
    const [extensionsEnabled, setExtensionsEnabled] = useState<LlmExtensionsEnabled>(DISABLED);

    useEffect(() => {
        invoke<LLMConnectSettings>('get_llm_connect_settings')
            .then((settings) => setExtensionsEnabled(toExtensionsEnabled(settings)))
            .catch(() => setExtensionsEnabled(DISABLED));
    }, []);

    return extensionsEnabled;
};

const toExtensionsEnabled = (settings: LLMConnectSettings) => ({
    llmConnectEnabled: settings.onboarding_completed && settings.enabled,
    commandModeEnabled: settings.onboarding_completed && settings.command.enabled,
});
