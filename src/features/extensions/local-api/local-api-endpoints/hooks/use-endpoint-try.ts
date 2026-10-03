import { useState } from 'react';
import type { EndpointMethod, EndpointTryState } from '../../local-api.types';

interface EndpointTryRequest {
    url: string;
    method: EndpointMethod;
    body?: FormData;
}

export const useEndpointTry = () => {
    const [state, setState] = useState<EndpointTryState>({ phase: 'idle' });

    const execute = async (request: EndpointTryRequest) => {
        setState({ phase: 'running' });
        try {
            const response = await fetch(request.url, { method: request.method, body: request.body });
            const body = await response.text();
            setState({ phase: 'done', status: response.status, statusText: response.statusText, body });
        } catch {
            setState({ phase: 'failed' });
        }
    };

    return { state, execute };
};
