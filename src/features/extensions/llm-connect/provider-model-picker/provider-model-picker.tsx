import { useState } from 'react';
import clsx from 'clsx';
import { toast } from 'react-toastify';
import { RefreshCw, Wrench, Monitor, Cloud, AlertTriangle } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Button } from '@/components/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { ModelCombobox } from '../model-combobox/model-combobox';
import { LLMProvider, OllamaModel } from '../hooks/use-llm-connect';

interface ProviderModelPickerProps {
    provider: LLMProvider;
    model: string;
    onChange: (value: { provider: LLMProvider; model: string }) => void;
    models: OllamaModel[];
    remoteModels: OllamaModel[];
    isLoading: boolean;
    isLocalConfigured: boolean;
    isRemoteConfigured: boolean;
    onRefreshModels: () => void;
    onRefreshRemoteModels: () => void;
}

export const ProviderModelPicker = ({
    provider,
    model,
    onChange,
    models,
    remoteModels,
    isLoading,
    isLocalConfigured,
    isRemoteConfigured,
    onRefreshModels,
    onRefreshRemoteModels,
}: ProviderModelPickerProps) => {
    const { t } = useTranslation();
    const [showRemoteUnavailableMessage, setShowRemoteUnavailableMessage] = useState(false);

    const isRemote = provider === 'remote';
    const currentModels = isRemote ? remoteModels : models;
    const handleRefresh = isRemote ? onRefreshRemoteModels : onRefreshModels;

    const handleProviderChange = (value: string) => {
        if (value !== 'local' && value !== 'remote') {
            return;
        }
        if (value === provider) {
            return;
        }
        if (value === 'remote' && !isRemoteConfigured) {
            setShowRemoteUnavailableMessage(true);
            toast.info(t('Configure your remote server in Advanced configuration first.'), { autoClose: 3000 });
            return;
        }
        if (value === 'local' && !isLocalConfigured) {
            toast.info(t('Configure your local Ollama server in Advanced configuration first.'), {
                autoClose: 3000,
            });
            return;
        }
        setShowRemoteUnavailableMessage(false);
        onChange({ provider: value, model: '' });
        if (value === 'local') {
            onRefreshModels();
        }
    };

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Wrench className="w-4 h-4 text-muted-foreground" />
                    {t('Model')}
                </Typography.Title>
            </SettingsUI.Description>

            <div className="flex gap-2 items-center">
                <Select value={provider} onValueChange={handleProviderChange}>
                    <SelectTrigger className="w-[140px] bg-black/30">
                        <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="local">
                            <div className={clsx('flex items-center gap-2', !isLocalConfigured && 'opacity-40')}>
                                <Monitor className="w-3.5 h-3.5 text-sky-400" />
                                {t('Local')}
                            </div>
                        </SelectItem>
                        <SelectItem value="remote">
                            <div className={clsx('flex items-center gap-2', !isRemoteConfigured && 'opacity-40')}>
                                <Cloud className="w-3.5 h-3.5 text-sky-400" />
                                {t('Remote')}
                            </div>
                        </SelectItem>
                    </SelectContent>
                </Select>

                <ModelCombobox
                    models={currentModels}
                    value={model}
                    onValueChange={(modelName) => onChange({ provider, model: modelName })}
                    placeholder={t('Select or type a model')}
                />
                <Button onClick={handleRefresh} variant="ghost" size="sm" className="p-2" title={t('Refresh Models')}>
                    <RefreshCw className={clsx('w-4 h-4', isLoading && 'animate-spin')} />
                </Button>
            </div>
            {showRemoteUnavailableMessage && (
                <div className="mt-2 flex items-center gap-1.5 text-xs text-yellow-300">
                    <AlertTriangle className="w-3 h-3 flex-shrink-0" />
                    {t('Configure your remote server in Advanced configuration first.')}
                </div>
            )}
        </SettingsUI.Item>
    );
};
