import type { LocalApiStatus, StatusTone } from '../local-api.types';

interface StatusView {
    label: string;
    hint?: string;
    tone: StatusTone;
    canRetry: boolean;
}

export const getStatusView = (status: LocalApiStatus | null, isStarting: boolean): StatusView => {
    if (isStarting || status === null) {
        return { label: 'Starting...', tone: 'neutral', canRetry: false };
    }
    switch (status.state) {
        case 'running':
            return { label: 'Running on http://127.0.0.1:{{port}}', tone: 'success', canRetry: false };
        case 'port_in_use':
            return {
                label: 'Port {{port}} is already used by another app.',
                hint: 'Choose another port below. The server restarts on its own.',
                tone: 'error',
                canRetry: true,
            };
        case 'failed':
            return {
                label: 'The server could not start. Check the logs for details.',
                tone: 'error',
                canRetry: true,
            };
        case 'stopped':
            return { label: 'The server is not running.', tone: 'neutral', canRetry: true };
    }
};
