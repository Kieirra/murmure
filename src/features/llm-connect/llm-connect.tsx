import { useTranslation } from '@/i18n';
import { useState, useEffect } from 'react';
import { useLLMConnect, LLMConnectSettings } from './hooks/use-llm-connect';
import { invoke } from '@tauri-apps/api/core';
import { useLLMPrompt } from './hooks/use-llm-prompt';

import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';
import { RefreshCw, Link as LinkIcon, Wrench } from 'lucide-react';
import { toast } from 'react-toastify';
import {
    getDefaultPrompt,
    getStatusIcon,
    getStatusText,
} from './llm-connect.helpers';
import { DEFAULT_OLLAMA_URL } from './llm-connect.constants';
import { RenderKeys } from '@/components/render-keys';
import {
    useShortcut,
    SHORTCUT_CONFIGS,
} from '../settings/shortcuts/hooks/use-shortcut';
import { PresetSelector } from './preset-selector/preset-selector';
import { LLMConnectOnboarding } from './onboarding/llm-connect-onboarding';
import { Input } from '@/components/input';

export const LLMConnect = () => {
    const { t, i18n } = useTranslation();
    const {
        settings,
        models,
        connectionStatus,
        isLoading,
        updateSettings,
        testConnection,
        fetchModels,
        pullModel,
    } = useLLMConnect();
    const initialPrompt =
        settings.prompt == null || settings.prompt.length === 0
            ? getDefaultPrompt(i18n.language)
            : settings.prompt;
    const { promptDraft, setPromptDraft } = useLLMPrompt(initialPrompt);
    const { shortcut: llmShortcut } = useShortcut(SHORTCUT_CONFIGS.llm);

    const [urlDraft, setUrlDraft] = useState(settings.url);
    const [showModelSelector, setShowModelSelector] = useState(false);

    // Sync url draft with settings when they change externally
    useEffect(() => {
        setUrlDraft(settings.url);
    }, [settings.url]);

    const handleResetOnboarding = async () => {
        try {
            const defaultPrompt = getDefaultPrompt(i18n.language);
            await updateSettings({
                onboarding_completed: false,
                prompt: defaultPrompt,
            });
            setPromptDraft(defaultPrompt);
        } catch {
            toast.error(t('Failed to reset onboarding'));
        }
    };

    const handleUrlBlur = async () => {
        if (settings.url !== urlDraft) {
            try {
                await updateSettings({ url: urlDraft });
            } catch {
                toast.error(t('Failed to update URL'));
            }
        }
    };

    const handleModelChange = async (model: string) => {
        try {
            await updateSettings({ model });
        } catch {
            toast.error(t('Failed to update model'));
        }
    };

    const handleSavePrompt = async () => {
        try {
            await updateSettings({ prompt: promptDraft });
        } catch {
            toast.error(t('Failed to update prompt'));
        }
    };

    const handleRefreshModels = async () => {
        try {
            await fetchModels();
            toast.success(t('Models refreshed'), { autoClose: 1500 });
        } catch {
            toast.error(t('Failed to fetch models'));
        }
    };

    const handleTestConnection = async () => {
        try {
            const result = await testConnection();
            if (result) {
                toast.success(t('Connection successful'), { autoClose: 1500 });
                await fetchModels();
            } else {
                toast.error(t('Connection failed'));
            }
        } catch {
            toast.error(t('Connection test failed'));
        }
    };

    return (
        <main>
            {!settings.onboarding_completed || showModelSelector ? (
                <LLMConnectOnboarding
                    settings={settings}
                    testConnection={testConnection}
                    pullModel={pullModel}
                    updateSettings={updateSettings}
                    initialStep={showModelSelector ? 2 : 0}
                    models={models}
                    fetchModels={fetchModels}
                    completeOnboarding={async () => {
                        await fetchModels();
                        const currentSettings =
                            await invoke<LLMConnectSettings>(
                                'get_llm_connect_settings'
                            );

                        // Only set prompt if this is the first onboarding (not when using "Install another model")
                        if (showModelSelector) {
                            setShowModelSelector(false);
                            if (currentSettings.model !== settings.model) {
                                await updateSettings(currentSettings);
                            }
                        } else {
                            const defaultPrompt = getDefaultPrompt(
                                i18n.language
                            );
                            const newPrompt =
                                currentSettings.prompt || defaultPrompt;
                            await updateSettings({
                                ...currentSettings,
                                onboarding_completed: true,
                                prompt: newPrompt,
                            });
                            setPromptDraft(newPrompt);
                        }
                    }}
                />
            ) : (
                <div className="space-y-8">
                    <Page.Header>
                        <Typography.MainTitle className="flex items-center gap-2">
                            {t('LLM Connect')}
                            <code className="text-amber-300 text-[10px]">
                                {t('Experimental')}
                            </code>
                        </Typography.MainTitle>
                        <Typography.Paragraph className="text-zinc-400">
                            {t(
                                'Connect Murmure to a local LLM via Ollama for post-processing and correcting transcriptions.'
                            )}
                            {llmShortcut && (
                                <>
                                    {' '}
                                    {t('Hold')}{' '}
                                    <RenderKeys keyString={llmShortcut} />{' '}
                                    {t('to use LLM Connect.')}
                                </>
                            )}
                        </Typography.Paragraph>
                    </Page.Header>

                    <div className="flex justify-center mb-8">
                        <SettingsUI.Container>
                            {/* Connection Status */}
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title className="flex items-center gap-2">
                                        {getStatusIcon(connectionStatus)}
                                        {getStatusText(connectionStatus, t)}
                                    </Typography.Title>
                                    <Typography.Paragraph>
                                        {t('Test your connection to Ollama')}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <Button
                                    onClick={handleTestConnection}
                                    variant="outline"
                                    size="sm"
                                    disabled={
                                        !settings.url ||
                                        connectionStatus === 'testing'
                                    }
                                >
                                    {t('Test Connection')}
                                </Button>
                            </SettingsUI.Item>

                            <SettingsUI.Separator />

                            {/* URL Input */}
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title className="flex items-center gap-2">
                                        <LinkIcon className="w-4 h-4 text-zinc-400" />
                                        {t('Ollama API URL')}
                                    </Typography.Title>
                                    <Typography.Paragraph>
                                        {t(
                                            'The URL of your local Ollama instance'
                                        )}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <Input
                                    type="text"
                                    value={urlDraft}
                                    onChange={(e) =>
                                        setUrlDraft(e.target.value)
                                    }
                                    onBlur={handleUrlBlur}
                                    className="w-[250px]"
                                    placeholder={DEFAULT_OLLAMA_URL}
                                    data-testid="llm-connect-url-input"
                                />
                            </SettingsUI.Item>

                            <SettingsUI.Separator />

                            {/* Model Selector */}
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title className="flex items-center gap-2">
                                        <Wrench className="w-4 h-4 text-zinc-400" />
                                        {t('Model')}
                                    </Typography.Title>
                                    <Typography.Paragraph>
                                        {t('Select the Ollama model to use')}
                                    </Typography.Paragraph>
                                    <button
                                        onClick={() =>
                                            setShowModelSelector(true)
                                        }
                                        className="text-sky-400 hover:text-sky-300 transition-colors text-sm cursor-pointer mt-1"
                                    >
                                        {t('Install another model')}
                                    </button>
                                </SettingsUI.Description>
                                <div className="flex flex-col gap-2 min-h-[60px] justify-center">
                                    <div className="flex gap-2 items-center">
                                        <Select
                                            value={settings.model}
                                            onValueChange={handleModelChange}
                                            disabled={models.length === 0}
                                        >
                                            <SelectTrigger
                                                className="w-[200px]"
                                                data-testid="llm-connect-model-select"
                                            >
                                                <SelectValue
                                                    placeholder={t(
                                                        'Select a model'
                                                    )}
                                                />
                                            </SelectTrigger>
                                            <SelectContent>
                                                {models.map((model) => (
                                                    <SelectItem
                                                        key={model.name}
                                                        value={model.name}
                                                    >
                                                        {model.name}
                                                    </SelectItem>
                                                ))}
                                            </SelectContent>
                                        </Select>
                                        <Button
                                            onClick={handleRefreshModels}
                                            variant="outline"
                                            size="sm"
                                            disabled={
                                                isLoading ||
                                                connectionStatus !== 'connected'
                                            }
                                            data-testid="llm-connect-refresh-models-button"
                                        >
                                            <RefreshCw
                                                className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`}
                                            />
                                        </Button>
                                    </div>
                                    {models.length === 0 &&
                                        connectionStatus === 'connected' &&
                                        !isLoading && (
                                            <Typography.Paragraph className="text-amber-400 text-xs">
                                                {t(
                                                    'No models found. Please install a model in Ollama first.'
                                                )}
                                            </Typography.Paragraph>
                                        )}
                                </div>
                            </SettingsUI.Item>

                            <SettingsUI.Separator />

                            {/* Prompt Editor */}
                            <SettingsUI.Item className="flex-col! items-start gap-4">
                                <SettingsUI.Description className="w-full">
                                    <Typography.Title>
                                        {t('Prompt')}
                                    </Typography.Title>
                                    <Typography.Paragraph>
                                        {t(
                                            'Use {{TRANSCRIPT}} for the transcription text and {{DICTIONARY}} for your custom dictionary words'
                                        )}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <div className="flex flex-col gap-2 w-full">
                                    <div className="relative">
                                        <textarea
                                            value={promptDraft}
                                            onChange={(e) =>
                                                setPromptDraft(
                                                    e.target.value.slice(
                                                        0,
                                                        4000
                                                    )
                                                )
                                            }
                                            maxLength={4000}
                                            className="px-3 py-2 bg-zinc-800/25 border border-zinc-700 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 min-h-[380px] font-mono w-full resize-y"
                                            placeholder={t(
                                                'Enter your prompt here...'
                                            )}
                                            data-testid="llm-connect-prompt-textarea"
                                        />
                                        <div className="absolute bottom-2 right-2 text-[10px] text-zinc-500 bg-zinc-800/80 px-2 py-1 rounded-md pointer-events-none">
                                            {promptDraft.length} / 4000
                                        </div>
                                    </div>
                                    <div className="flex gap-2 justify-between w-full items-center">
                                        <div className="flex gap-2 items-center">
                                            <PresetSelector
                                                onSelect={setPromptDraft}
                                            />
                                        </div>
                                        <Button
                                            onClick={handleSavePrompt}
                                            variant="outline"
                                            className="bg-sky-600! hover:bg-sky-700! disabled:bg-zinc-800! text-white"
                                            size="sm"
                                            disabled={
                                                promptDraft === settings.prompt
                                            }
                                            data-testid="llm-connect-save-button"
                                        >
                                            {t('Save')}
                                        </Button>
                                    </div>
                                </div>
                            </SettingsUI.Item>
                            <SettingsUI.Separator />

                            {/* Reset Tutorial Button */}
                            <div className="flex p-1">
                                <Page.SecondaryButton
                                    onClick={handleResetOnboarding}
                                    variant="ghost"
                                    size="sm"
                                    className="text-zinc-500 hover:text-zinc-300"
                                >
                                    {t('Reset Tutorial')}
                                </Page.SecondaryButton>
                            </div>
                        </SettingsUI.Container>
                    </div>
                </div>
            )}
        </main>
    );
};
