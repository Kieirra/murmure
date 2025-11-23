import { AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';
import { ConnectionStatus } from './hooks/use-llm-connect';
import { PROMPT_PRESETS, PromptPresetType } from './llm-connect.constants';

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

export const getDefaultPrompt = (language: string) => {
    const isFrench = language.startsWith('fr');
    return PROMPT_PRESETS.general.prompts[isFrench ? 'fr' : 'en'];
}

export const getPromptByPreset = (preset: PromptPresetType, language: string): string => {
    const isFrench = language.startsWith('fr');
    return PROMPT_PRESETS[preset].prompts[isFrench ? 'fr' : 'en'];
};

export const getPresetTypes = (): PromptPresetType[] => {
    return Object.keys(PROMPT_PRESETS) as PromptPresetType[];
};

export const getPresetLabel = (preset: PromptPresetType): string => {
    return PROMPT_PRESETS[preset].label;
};