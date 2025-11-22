import { AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';
import { ConnectionStatus } from './hooks/use-llm-connect';
import { MEDICAL_PROMPT_EN, MEDICAL_PROMPT_FR } from './llm-connect.constants';

export const getStatusIcon = (connectionStatus: ConnectionStatus) => {
    switch (connectionStatus) {
        case 'connected':
            return <CheckCircle2 className="w-4 h-4 text-green-500" />;
        case 'testing':
            return <Loader2 className="w-4 h-4 text-blue-500 animate-spin" />;
        case 'error':
            return <AlertCircle className="w-4 h-4 text-red-500" />;
        default:
            return <AlertCircle className="w-4 h-4 text-zinc-500" />;
    }
};

export const getStatusText = (connectionStatus: ConnectionStatus, t: (key: string) => string) => {
    switch (connectionStatus) {
        case 'connected':
            return t('Connected');
        case 'testing':
            return t('Testing...');
        case 'error':
            return t('Connection error');
        default:
            return t('Disconnected');
    }
};

export const getDefaultMedicalPrompt = (language: string) => {
    return language.startsWith('fr') ? MEDICAL_PROMPT_FR : MEDICAL_PROMPT_EN;
}