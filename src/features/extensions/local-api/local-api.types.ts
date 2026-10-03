export type LocalApiServerState = 'stopped' | 'running' | 'port_in_use' | 'failed';

export interface LocalApiStatus {
    state: LocalApiServerState;
    port: number;
    message: string | null;
}

export type EndpointMethod = 'GET' | 'POST';

export type EndpointInputKind = 'file' | 'instruction' | 'model' | 'provider' | 'prompt-name';

export interface EndpointField {
    name: string;
    kind: EndpointInputKind;
    description: string;
}

export interface EndpointDefinition {
    method: EndpointMethod;
    path: string;
    title: string;
    description: string;
    requiresPromptMode: boolean;
    fields: EndpointField[];
    responseExample: string;
}

export type EndpointTryState =
    | { phase: 'idle' }
    | { phase: 'running' }
    | { phase: 'done'; status: number; statusText: string; body: string }
    | { phase: 'failed' };

export type StatusTone = 'success' | 'warning' | 'error' | 'neutral';
