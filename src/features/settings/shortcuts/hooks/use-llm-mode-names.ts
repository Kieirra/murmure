import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import type { LLMConnectSettings } from '@/features/extensions/llm-connect/hooks/use-llm-connect';

export const useLlmModeNames = () => {
    const [names, setNames] = useState<string[]>([]);

    useEffect(() => {
        invoke<LLMConnectSettings>('get_llm_connect_settings')
            .then((settings) => setNames(settings.modes.map((mode) => mode.name)))
            .catch(() => setNames([]));

        const unlisten = listen<LLMConnectSettings>('llm-settings-updated', (event) => {
            setNames(event.payload.modes.map((mode) => mode.name));
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    return names;
};
