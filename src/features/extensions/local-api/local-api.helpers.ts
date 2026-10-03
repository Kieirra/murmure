import type { EndpointDefinition, EndpointField } from './local-api.types';

export const DEFAULT_API_PORT = 4800;
export const MIN_API_PORT = 1024;
export const MAX_API_PORT = 65535;

export const AUDIO_FIELD: EndpointField = { name: 'audio', kind: 'file', description: 'WAV file, 16 bit PCM.' };

export const TRANSCRIBE_ENDPOINT: EndpointDefinition = {
    method: 'POST',
    path: '/api/transcribe',
    title: 'Transcribe an audio file',
    description: 'Send a WAV file, get the text back as JSON.',
    requiresPromptMode: false,
    fields: [AUDIO_FIELD],
    responseExample: JSON.stringify({ text: '...' }, null, 2),
};

export const buildCurl = (endpoint: EndpointDefinition, port: number, values: Record<string, string>) => {
    const url = buildEndpointUrl(endpoint, port);
    if (endpoint.method === 'GET') {
        return `curl ${url}`;
    }
    const formFields = endpoint.fields.map((field) => {
        const value = field.kind === 'file' ? '@recording.wav' : escapeQuotes(values[field.name] ?? '');
        return `-F "${field.name}=${value}"`;
    });
    return [`curl -X POST ${url}`, ...formFields].join(' ');
};

export const buildEndpointUrl = (endpoint: EndpointDefinition, port: number) => `${buildBaseUrl(port)}${endpoint.path}`;

const buildBaseUrl = (port: number) => `http://127.0.0.1:${port}`;

const escapeQuotes = (value: string) => value.replace(/[\\"$`]/g, (char) => `\\${char}`);
