import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { LLMConnectSettings } from './use-llm-connect';

export const useLlmOnboardingCompleted = () => {
    const [llmOnboardingCompleted, setLlmOnboardingCompleted] = useState(false);

    useEffect(() => {
        invoke<LLMConnectSettings>('get_llm_connect_settings')
            .then((settings) => setLlmOnboardingCompleted(settings.onboarding_completed))
            .catch(() => setLlmOnboardingCompleted(false));
    }, []);

    return llmOnboardingCompleted;
};
