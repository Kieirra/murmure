import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef, useState } from 'react';

const PROMPT_NAME_HOLD_MS = 1800;

export const useLlmPromptFlash = () => {
    const [promptName, setPromptName] = useState<string | null>(null);
    const hideTimerRef = useRef<number | null>(null);

    useEffect(() => {
        const showPromptName = () => {
            invoke<string | null>('get_active_llm_prompt_name')
                .then((name) => {
                    if (name == null) return;
                    if (hideTimerRef.current != null) clearTimeout(hideTimerRef.current);
                    setPromptName(name);
                    hideTimerRef.current = setTimeout(() => setPromptName(null), PROMPT_NAME_HOLD_MS);
                })
                .catch(() => {});
        };

        invoke<string>('get_recording_mode')
            .then((mode) => {
                if (mode === 'llm') showPromptName();
            })
            .catch(() => {});

        const unlistenPromise = listen<string>('recording-mode', (event) => {
            if (event.payload === 'llm') showPromptName();
        });

        return () => {
            if (hideTimerRef.current != null) clearTimeout(hideTimerRef.current);
            unlistenPromise.then((u) => u()).catch(() => {});
        };
    }, []);

    return { promptName };
};
