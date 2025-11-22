import { useTranslation } from '@/i18n';
import { useLLMConnect } from './hooks/use-llm-connect';
import { useLLMPrompt } from './hooks/use-llm-prompt';
import { Switch } from '@/components/switch';
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
import { RefreshCw, Sparkles, Link as LinkIcon, Wrench } from 'lucide-react';
import { toast } from 'react-toastify';
import { getStatusIcon, getStatusText } from './llm-connect.helpers';
import { RenderKeys } from '@/components/render-keys';
import { useLLMShortcutState } from '../settings/shortcuts/hooks/use-llm-shortcut-state';

export const LLMConnect = () => {
    const { t } = useTranslation();
    const {
        settings,
        models,
        connectionStatus,
        isLoading,
        updateSettings,
        testConnection,
        fetchModels,
    } = useLLMConnect();
    const { promptDraft, setPromptDraft } = useLLMPrompt(settings.prompt);
    const { llmShortcut } = useLLMShortcutState();

    const handleToggle = async (enabled: boolean) => {
        try {
            await updateSettings({ enabled });
            if (enabled && settings.url) {
                const connected = await testConnection();
                if (connected) {
                    await fetchModels();
                }
            }
        } catch (error) {
            toast.error(t('Failed to update settings'));
        }
    };

    const handleUrlBlur = async () => {
        if (settings.url !== promptDraft) {
            try {
                await updateSettings({ url: settings.url });
            } catch (error) {
                toast.error(t('Failed to update URL'));
            }
        }
    };

    const handleModelChange = async (model: string) => {
        try {
            await updateSettings({ model });
        } catch (error) {
            toast.error(t('Failed to update model'));
        }
    };

    const handleSavePrompt = async () => {
        try {
            await updateSettings({ prompt: promptDraft });
            toast.success(t('Prompt saved'), { autoClose: 1500 });
        } catch (error) {
            toast.error(t('Failed to update prompt'));
        }
    };

    const handleRefreshModels = async () => {
        try {
            await fetchModels();
            toast.success(t('Models refreshed'), { autoClose: 1500 });
        } catch (error) {
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
        } catch (error) {
            toast.error(t('Connection test failed'));
        }
    };

    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle className="flex items-center gap-2">
                        {t('LLM Connect')}
                        <code className="text-amber-300 text-[10px]">
                            {t('Experimental')}
                        </code>
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        {t('Connect Murmure to a local LLM via Ollama for post-processing and correcting transcriptions.')}
                        {llmShortcut && (
                            <>
                                {' '}
                                {t('Hold')}
                                {' '}
                                <RenderKeys keyString={llmShortcut} />
                                {' '}
                                {t('to use LLM Connect.')}
                            </>
                        )}
                    </Typography.Paragraph>
                </Page.Header>

                <div className="flex justify-center mb-8">
                    <SettingsUI.Container>
                        {/* Enable/Disable Toggle */}
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title className="flex items-center gap-2">
                                    <Sparkles className="w-4 h-4 text-zinc-400" />
                                    {t('Enable LLM Connect')}
                                </Typography.Title>
                                <Typography.Paragraph>
                                    {t('Post-process transcriptions with a local LLM')}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch
                                checked={settings.enabled}
                                onCheckedChange={handleToggle}
                            />
                        </SettingsUI.Item>

                        {settings.enabled && (
                            <>
                                <SettingsUI.Separator />

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
                                        disabled={!settings.url || connectionStatus === 'testing'}
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
                                            {t('The URL of your local Ollama instance')}
                                        </Typography.Paragraph>
                                    </SettingsUI.Description>
                                    <input
                                        type="text"
                                        value={settings.url}
                                        onChange={(e) => updateSettings({ url: e.target.value })}
                                        onBlur={handleUrlBlur}
                                        className="px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 w-[300px]"
                                        placeholder="http://localhost:11434/api"
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
                                    </SettingsUI.Description>
                                    <div className="flex flex-col gap-2">
                                        <div className="flex gap-2">
                                            <Select value={settings.model} onValueChange={handleModelChange} disabled={models.length === 0}>
                                                <SelectTrigger className="w-[200px]">
                                                    <SelectValue placeholder={t('Select a model')} />
                                                </SelectTrigger>
                                                <SelectContent>
                                                    {models.map((model) => (
                                                        <SelectItem key={model.name} value={model.name}>
                                                            {model.name}
                                                        </SelectItem>
                                                    ))}
                                                </SelectContent>
                                            </Select>
                                            <Button
                                                onClick={handleRefreshModels}
                                                variant="outline"
                                                size="sm"
                                                disabled={isLoading || connectionStatus !== 'connected'}
                                            >
                                                <RefreshCw className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
                                            </Button>
                                        </div>
                                        {models.length === 0 && connectionStatus === 'connected' && (
                                            <Typography.Paragraph className="text-amber-400 text-xs">
                                                {t('No models found. Please install a model in Ollama first.')}
                                            </Typography.Paragraph>
                                        )}
                                    </div>
                                </SettingsUI.Item>

                                <SettingsUI.Separator />

                                {/* Prompt Editor */}
                                <SettingsUI.Item className="!flex-col items-start gap-4">
                                    <SettingsUI.Description className="w-full">
                                        <Typography.Title>
                                            {t('Prompt')}
                                        </Typography.Title>
                                        <Typography.Paragraph>
                                            {t('Use {{TRANSCRIPT}} as a placeholder for the transcription text')}
                                        </Typography.Paragraph>
                                    </SettingsUI.Description>
                                    <div className="flex flex-col gap-2 w-full">
                                        <textarea
                                            value={promptDraft}
                                            onChange={(e) => setPromptDraft(e.target.value)}
                                            className="px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 min-h-[150px] font-mono w-full"
                                            placeholder={t('Enter your prompt here...')}
                                        />
                                        <Button
                                            onClick={handleSavePrompt}
                                            variant="default"
                                            size="sm"
                                            disabled={promptDraft === settings.prompt}
                                            className="self-end"
                                        >
                                            {t('Save')}
                                        </Button>
                                    </div>
                                </SettingsUI.Item>
                            </>
                        )}
                    </SettingsUI.Container>
                </div>
            </div>
        </main>
    );
};

