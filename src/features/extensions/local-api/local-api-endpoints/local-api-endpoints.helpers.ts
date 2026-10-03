import type { LLMMode } from '@/features/extensions/llm-connect/hooks/use-llm-connect';
import { AUDIO_FIELD, TRANSCRIBE_ENDPOINT } from '../local-api.helpers';
import type { EndpointDefinition, EndpointField, StatusTone } from '../local-api.types';

const PROMPT_MODE_RESPONSE = JSON.stringify({ text: '...', transcription: '...' }, null, 2);

export const ENDPOINTS: EndpointDefinition[] = [
    TRANSCRIBE_ENDPOINT,
    {
        method: 'POST',
        path: '/api/prompt-mode/custom',
        title: 'Transcribe and apply your own prompt',
        description: 'Send the audio, your prompt and the model to use.',
        requiresPromptMode: true,
        fields: [
            AUDIO_FIELD,
            { name: 'instruction', kind: 'instruction', description: 'Your prompt, up to 4000 characters.' },
            { name: 'provider', kind: 'provider', description: 'local or remote.' },
            { name: 'model', kind: 'model', description: 'Model name on this provider.' },
        ],
        responseExample: PROMPT_MODE_RESPONSE,
    },
    {
        method: 'POST',
        path: '/api/prompt-mode',
        title: 'Transcribe and apply a saved prompt',
        description: 'Runs one of your Prompt Mode prompts.',
        requiresPromptMode: true,
        fields: [
            AUDIO_FIELD,
            { name: 'prompt', kind: 'prompt-name', description: 'Exact name of a Prompt Mode prompt.' },
        ],
        responseExample: PROMPT_MODE_RESPONSE,
    },
    {
        method: 'GET',
        path: '/api/prompt-mode/prompts',
        title: 'List saved prompts',
        description: 'Returns the names of your Prompt Mode prompts.',
        requiresPromptMode: false,
        fields: [],
        responseExample: JSON.stringify({ prompts: ['General', 'Email'] }, null, 2),
    },
];

export const formatResponseBody = (raw: string) => {
    try {
        return JSON.stringify(JSON.parse(raw), null, 2);
    } catch {
        return raw;
    }
};

export const getStatusTone = (status: number): StatusTone => {
    if (status >= 200 && status < 300) {
        return 'success';
    }
    if (status >= 400 && status < 500) {
        return 'warning';
    }
    if (status >= 500) {
        return 'error';
    }
    return 'neutral';
};

export const getPromptNames = (prompts: LLMMode[]) =>
    prompts.map((prompt) => prompt.name).filter((name) => name.trim().length > 0);

export const getInitialValues = (fields: EndpointField[], prompts: LLMMode[]) =>
    Object.fromEntries(
        fields.filter((field) => field.kind !== 'file').map((field) => [field.name, getInitialValue(field, prompts)])
    );

export const getTryBlockedReason = (
    fields: EndpointField[],
    values: Record<string, string>,
    file: File | null,
    isServerRunning: boolean
) => {
    const isMissing = (field: EndpointField) => (values[field.name] ?? '').trim().length === 0;

    if (!isServerRunning) {
        return 'Start the server to try this endpoint.';
    }
    if (fields.some((field) => field.kind === 'file') && file === null) {
        return 'Choose a WAV file first.';
    }
    if (fields.some((field) => (field.kind === 'instruction' || field.kind === 'model') && isMissing(field))) {
        return 'Fill in the instruction and the model.';
    }
    if (fields.some((field) => field.kind === 'prompt-name' && isMissing(field))) {
        return 'Create a prompt first.';
    }
    return null;
};

export const buildFormData = (fields: EndpointField[], values: Record<string, string>, file: File | null) => {
    if (fields.length === 0) {
        return undefined;
    }
    const formData = new FormData();
    fields.forEach((field) => {
        if (field.kind === 'file') {
            if (file !== null) {
                formData.append(field.name, file);
            }
            return;
        }
        formData.append(field.name, values[field.name] ?? '');
    });
    return formData;
};

const getInitialValue = (field: EndpointField, prompts: LLMMode[]) => {
    switch (field.kind) {
        case 'instruction':
            return 'Summarize this in three bullet points.';
        case 'model':
            return prompts.length > 0 && prompts[0].model.length > 0 ? prompts[0].model : 'qwen3:8b';
        case 'provider':
            return prompts[0]?.provider ?? 'local';
        case 'prompt-name':
            return getPromptNames(prompts)[0] ?? '';
        case 'file':
            return '';
    }
};
