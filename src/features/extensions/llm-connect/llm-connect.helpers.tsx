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
            return <AlertCircle className="w-4 h-4 text-muted-foreground" />;
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

export const getStatusColorStyles = (connectionStatus: ConnectionStatus) => {
    switch (connectionStatus) {
        case 'connected':
            return 'bg-emerald-500/10 text-emerald-500 border-emerald-500/20';
        case 'error':
            return 'bg-red-500/10 text-red-500 border-red-500/20';
        default:
            return 'bg-card text-muted-foreground border-border';
    }
};

export const getDefaultPrompt = (language: string) => {
    const isFrench = language.startsWith('fr');
    return PROMPT_PRESETS.general.prompts[isFrench ? 'fr' : 'en'];
};

export const getPromptByPreset = (preset: PromptPresetType, language: string): string => {
    const isFrench = language.startsWith('fr');
    return PROMPT_PRESETS[preset].prompts[isFrench ? 'fr' : 'en'];
};

export const getPresetTypes = (): PromptPresetType[] => {
    return Object.keys(PROMPT_PRESETS);
};

export const getPresetLabel = (preset: PromptPresetType): string => {
    return PROMPT_PRESETS[preset].label;
};

export const getPresetDescription = (preset: PromptPresetType): string => {
    return PROMPT_PRESETS[preset].description;
};

export const isInsecureRemoteUrl = (url: string): boolean => {
    if (!url?.startsWith('http://')) return false;
    try {
        const hostname = new URL(url).hostname;
        if (hostname === 'localhost' || hostname === '127.0.0.1') return false;
        if (hostname.startsWith('192.168.') || hostname.startsWith('10.')) return false;
        if (/^172\.(1[6-9]|2\d|3[01])\./.test(hostname)) return false;
        return true;
    } catch {
        return false;
    }
};
