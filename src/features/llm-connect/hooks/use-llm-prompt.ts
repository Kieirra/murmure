import { useState, useEffect } from 'react';

export const useLLMPrompt = (initialPrompt: string) => {
    const [promptDraft, setPromptDraft] = useState(initialPrompt);

    // Sync prompt draft with settings
    useEffect(() => {
        setPromptDraft(initialPrompt);
    }, [initialPrompt]);

    return {
        promptDraft,
        setPromptDraft,
    };
};
