import { useState, useEffect, useCallback } from 'react';
import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Button } from '@/components/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { RefreshCw, Wrench, Monitor, Cloud, AlertTriangle } from 'lucide-react';
import { HighlightedPromptEditor } from './highlighted-prompt-editor';
import { ModelCombobox } from './model-combobox';
import clsx from 'clsx';
import { RenderKeys } from '@/components/render-keys';
import { toast } from 'react-toastify';
import { LLMConnectSettings, LLMMode, LLMProvider, OllamaModel } from '../hooks/use-llm-connect';

interface ModeContentProps {
    activeMode: LLMMode;
    activeModeIndex: number;
    modes: LLMMode[];
    models: OllamaModel[];
    isLoading: boolean;
    updateSettings: (updates: Partial<LLMConnectSettings>) => Promise<void>;
    onRefreshModels: () => void;
    remoteModels: OllamaModel[];
    isRemoteConfigured: boolean;
    isLocalConfigured: boolean;
    onRefreshRemoteModels: () => void;
}

export const ModeContent = ({
    activeMode,
    activeModeIndex,
    modes,
    models,
    isLoading,
    updateSettings,
    onRefreshModels,
    remoteModels,
    isRemoteConfigured,
    isLocalConfigured,
    onRefreshRemoteModels,
}: ModeContentProps) => {
    const { t } = useTranslation();
    const [promptDraft, setPromptDraft] = useState(activeMode.prompt);
    const [showRemoteUnavailableMessage, setShowRemoteUnavailableMessage] = useState(false);

    const activeProvider = activeMode.provider ?? 'local';
    const isRemote = activeProvider === 'remote';
    const currentModels = isRemote ? remoteModels : models;
    const promptMaxLength = isRemote ? undefined : 4000;

    // Sync local draft when active mode changes
    useEffect(() => {
        setPromptDraft(activeMode.prompt);
    }, [activeMode.prompt, activeModeIndex]);

    const updateActiveMode = useCallback(
        (updates: Partial<LLMMode>) => {
            const newModes = [...modes];
            newModes[activeModeIndex] = { ...activeMode, ...updates };
            updateSettings({ modes: newModes });
        },
        [activeMode, activeModeIndex, modes, updateSettings]
    );

    // Autosave Prompt Debounce
    useEffect(() => {
        const timer = setTimeout(() => {
            if (promptDraft !== activeMode.prompt) {
                updateActiveMode({ prompt: promptDraft });
            }
        }, 1000);
        return () => clearTimeout(timer);
    }, [promptDraft, activeMode.prompt, activeModeIndex, updateActiveMode]);

    const handleModelChange = useCallback(
        (modelName: string) => {
            updateActiveMode({ model: modelName });
        },
        [updateActiveMode]
    );

    const handleProviderChange = useCallback(
        (value: string) => {
            const provider = value as LLMProvider;
            if (provider === activeProvider) {
                return;
            }
            if (provider === 'remote' && !isRemoteConfigured) {
                setShowRemoteUnavailableMessage(true);
                toast.info(t('Configure your remote server in Advanced configuration first.'), { autoClose: 3000 });
                return;
            }
            if (provider === 'local' && !isLocalConfigured) {
                toast.info(t('Configure your local Ollama server in Advanced configuration first.'), {
                    autoClose: 3000,
                });
                return;
            }
            setShowRemoteUnavailableMessage(false);
            updateActiveMode({ provider, model: '' });
            if (provider === 'remote') {
                onRefreshRemoteModels();
            } else {
                onRefreshModels();
            }
        },
        [
            activeProvider,
            updateActiveMode,
            isRemoteConfigured,
            isLocalConfigured,
            t,
            onRefreshRemoteModels,
            onRefreshModels,
        ]
    );

    const handleRefresh = isRemote ? onRefreshRemoteModels : onRefreshModels;

    const promptExceedsLocalLimit = activeProvider === 'local' && promptDraft.length > 4000;
    const shouldShowRemoteUnavailableMessage = showRemoteUnavailableMessage;

    return (
        <div className="flex flex-col gap-6">
            <SettingsUI.Container>
                {/* Model */}
                <SettingsUI.Item>
                    <SettingsUI.Description>
                        <Typography.Title className="flex items-center gap-2">
                            <Wrench className="w-4 h-4 text-muted-foreground" />
                            {t('Model')}
                        </Typography.Title>
                    </SettingsUI.Description>

                    <div className="flex gap-2 items-center">
                        <Select value={activeProvider} onValueChange={handleProviderChange}>
                            <SelectTrigger className="w-[140px]">
                                <SelectValue />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="local">
                                    <div
                                        className={clsx('flex items-center gap-2', !isLocalConfigured && 'opacity-40')}
                                    >
                                        <Monitor className="w-3.5 h-3.5 text-emerald-400" />
                                        {t('Local')}
                                    </div>
                                </SelectItem>
                                <SelectItem value="remote">
                                    <div
                                        className={clsx('flex items-center gap-2', !isRemoteConfigured && 'opacity-40')}
                                    >
                                        <Cloud className="w-3.5 h-3.5 text-sky-400" />
                                        {t('Remote')}
                                    </div>
                                </SelectItem>
                            </SelectContent>
                        </Select>

                        <ModelCombobox
                            models={currentModels}
                            value={activeMode.model}
                            onValueChange={handleModelChange}
                            disabled={currentModels.length === 0}
                            placeholder={t('Select a model')}
                        />
                        <Button
                            onClick={handleRefresh}
                            variant="ghost"
                            size="sm"
                            className="p-2"
                            title={t('Refresh Models')}
                        >
                            <RefreshCw className={clsx('w-4 h-4', isLoading && 'animate-spin')} />
                        </Button>
                    </div>
                    {shouldShowRemoteUnavailableMessage && (
                        <div className="mt-2 flex items-center gap-1.5 text-xs text-amber-500">
                            <AlertTriangle className="w-3 h-3 flex-shrink-0" />
                            {t('Configure your remote server in Advanced configuration first.')}
                        </div>
                    )}
                </SettingsUI.Item>

                {isRemote && (
                    <div className="flex justify-center -mt-2 pb-2">
                        <span className="flex items-center gap-1.5 text-xs text-amber-500">
                            <AlertTriangle className="w-3 h-3 flex-shrink-0" />
                            {t(
                                'Your transcriptions are sent to a third-party server and are not protected by local privacy.'
                            )}
                        </span>
                    </div>
                )}

                <SettingsUI.Separator />

                {/* Prompt Editor */}
                <SettingsUI.Item className="flex-col! items-start gap-4">
                    <div className="flex w-full items-start">
                        <SettingsUI.Description className="flex-1">
                            <Typography.Title>{t('Prompt')}</Typography.Title>
                            <Typography.Paragraph>
                                {t(
                                    'Use {{TRANSCRIPT}} as the captured text and {{DICTIONARY}} as the word set defined in Personalize > Dictionary.'
                                )}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <div className="text-xs text-muted-foreground bg-background/50 px-2 rounded w-34">
                            <RenderKeys keyString={activeMode.shortcut} />
                        </div>
                    </div>

                    <div className="relative w-full">
                        <HighlightedPromptEditor
                            value={promptDraft}
                            onChange={(value) => setPromptDraft(value)}
                            maxLength={promptMaxLength}
                            placeholder={t('Enter your prompt here...')}
                            className="w-full h-[600px]"
                        />
                        <div className="absolute bottom-3 right-3 flex flex-col gap-1 items-end pointer-events-none z-20">
                            <span
                                className={clsx(
                                    'text-[10px] mb-1',
                                    promptExceedsLocalLimit ? 'text-red-400' : 'text-muted-foreground'
                                )}
                            >
                                {isRemote ? promptDraft.length : `${promptDraft.length} / 4000`}
                            </span>
                        </div>
                    </div>

                    {promptExceedsLocalLimit && (
                        <div className="flex items-center gap-2 text-xs text-amber-500">
                            <AlertTriangle className="w-3.5 h-3.5" />
                            {t(
                                'Prompt exceeds the recommended limit for local models. This may cause context overflow errors.'
                            )}
                        </div>
                    )}
                </SettingsUI.Item>
            </SettingsUI.Container>
        </div>
    );
};
