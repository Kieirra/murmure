import { useState, useEffect } from 'react';
import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Input } from '@/components/input';
import { Page } from '@/components/page';
import { invoke } from '@tauri-apps/api/core';
import {
    AlertTriangle,
    Eye,
    EyeOff,
    CheckCircle2,
    RefreshCw,
    AlertCircle,
    ChevronDown,
    ChevronUp,
    Settings2,
    Monitor,
    Cloud,
} from 'lucide-react';
import { ConnectionStatus } from '../hooks/use-llm-connect';
import { DEFAULT_REMOTE_URL_PLACEHOLDER } from '../llm-connect.constants';
import { isInsecureRemoteUrl } from '../llm-connect.helpers';
import clsx from 'clsx';

interface LLMAdvancedSettingsProps {
    url: string;
    onUrlChange: (url: string) => void;
    onTestConnection: () => void;
    localConnectionStatus: ConnectionStatus;
    onInstallModel: () => void;
    onResetOnboarding: () => void;
    remoteUrl: string;
    onRemoteUrlChange: (url: string) => void;
    onTestRemoteConnection: () => Promise<void>;
    remoteConnectionStatus: ConnectionStatus;
    hasApiKey: boolean;
    onApiKeyChange: (apiKey: string) => Promise<void>;
    showInstallModel: boolean;
}

export const LLMAdvancedSettings = ({
    url,
    onUrlChange,
    onTestConnection,
    localConnectionStatus,
    onInstallModel,
    onResetOnboarding,
    remoteUrl,
    onRemoteUrlChange,
    onTestRemoteConnection,
    remoteConnectionStatus,
    onApiKeyChange,
    showInstallModel,
}: LLMAdvancedSettingsProps) => {
    const { t } = useTranslation();
    const [isOpen, setIsOpen] = useState(false);
    const [showApiKey, setShowApiKey] = useState(false);
    const [apiKeyValue, setApiKeyValue] = useState('');
    const [isTesting, setIsTesting] = useState(false);
    const [isTestingLocal, setIsTestingLocal] = useState(false);
    const [remoteError, setRemoteError] = useState<string | null>(null);

    useEffect(() => {
        const loadApiKey = async () => {
            try {
                const key = await invoke<string>('get_remote_api_key');
                setApiKeyValue(key);
            } catch {
                // No API key stored
            }
        };
        loadApiKey();
    }, []);

    const handleApiKeyBlur = async () => {
        await onApiKeyChange(apiKeyValue);
    };

    const handleTestLocal = async () => {
        setIsTestingLocal(true);
        try {
            await onTestConnection();
        } finally {
            setIsTestingLocal(false);
        }
    };

    const handleTestRemote = async () => {
        setIsTesting(true);
        setRemoteError(null);
        try {
            await onApiKeyChange(apiKeyValue);
            await onTestRemoteConnection();
        } catch (err: unknown) {
            const errorMessage =
                err instanceof Error ? err.message : String(err);
            setRemoteError(errorMessage);
        } finally {
            setIsTesting(false);
        }
    };

    const renderConnectionButton = (
        isCurrentlyTesting: boolean,
        status: ConnectionStatus,
        onClick: () => void,
        disabled?: boolean
    ) => (
        <Page.SecondaryButton
            onClick={onClick}
            size="sm"
            disabled={disabled || isCurrentlyTesting}
            className={clsx(
                'whitespace-nowrap',
                status === 'connected' &&
                    'text-emerald-500 hover:bg-emerald-400/10 hover:text-emerald-300'
            )}
        >
            {isCurrentlyTesting ? (
                <>
                    <RefreshCw className="w-4 h-4 animate-spin mr-2" />
                    {t('Testing...')}
                </>
            ) : status === 'connected' ? (
                <>
                    <CheckCircle2 className="w-4 h-4 mr-2" />
                    {t('Connected')}
                </>
            ) : (
                t('Test Connection')
            )}
        </Page.SecondaryButton>
    );

    return (
        <div className="mb-6 flex flex-col gap-2">
            {/* Toggle Advanced Configuration */}
            <button
                type="button"
                onClick={() => setIsOpen(!isOpen)}
                className="flex items-center gap-2 text-sm font-medium text-zinc-300 hover:text-zinc-100 transition-colors w-fit cursor-pointer"
            >
                <Settings2 className="w-4 h-4" />
                {t('Advanced configuration')}
                {isOpen ? (
                    <ChevronUp className="w-4 h-4 text-zinc-500" />
                ) : (
                    <ChevronDown className="w-4 h-4 text-zinc-500" />
                )}
            </button>

            {isOpen && (
                <>
                    {/* Local Server */}
                    <section>
                        <Typography.Title className="p-2 font-semibold text-sky-400! flex items-center gap-2">
                            <Monitor className="w-4 h-4" />
                            {t('Local Server (Ollama)')}
                        </Typography.Title>
                        <SettingsUI.Container>
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>
                                        {t('Server URL')}
                                    </Typography.Title>
                                </SettingsUI.Description>
                                <div className="flex items-center gap-3">
                                    <Input
                                        value={url}
                                        onChange={(e) =>
                                            onUrlChange(e.target.value)
                                        }
                                        className="w-[280px]"
                                        placeholder="http://localhost:11434/api"
                                    />
                                    {renderConnectionButton(
                                        isTestingLocal,
                                        localConnectionStatus,
                                        handleTestLocal
                                    )}
                                </div>
                            </SettingsUI.Item>
                            {showInstallModel && (
                                <>
                                    <SettingsUI.Separator />
                                    <SettingsUI.Item>
                                        <SettingsUI.Description>
                                            <Typography.Title>
                                                {t('Models')}
                                            </Typography.Title>
                                        </SettingsUI.Description>
                                        <Page.SecondaryButton
                                            onClick={onInstallModel}
                                            size="sm"
                                        >
                                            {t('Install another model')}
                                        </Page.SecondaryButton>
                                    </SettingsUI.Item>
                                </>
                            )}
                        </SettingsUI.Container>
                    </section>

                    {/* Remote Server */}
                    <section>
                        <Typography.Title className="p-2 font-semibold text-sky-400! flex items-center gap-2">
                            <Cloud className="w-4 h-4" />
                            {t('Remote Server (OpenAI-compatible)')}
                        </Typography.Title>
                        <SettingsUI.Container>
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>
                                        {t('Server URL')}
                                    </Typography.Title>
                                </SettingsUI.Description>
                                <div className="flex items-center gap-3">
                                    <Input
                                        value={remoteUrl}
                                        onChange={(e) =>
                                            onRemoteUrlChange(e.target.value)
                                        }
                                        className="w-[280px]"
                                        placeholder={
                                            DEFAULT_REMOTE_URL_PLACEHOLDER
                                        }
                                    />
                                    {renderConnectionButton(
                                        isTesting,
                                        remoteConnectionStatus,
                                        handleTestRemote,
                                        remoteUrl.length === 0
                                    )}
                                </div>
                            </SettingsUI.Item>
                            <SettingsUI.Separator />
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>
                                        {t('API Key')}
                                    </Typography.Title>
                                    <Typography.Paragraph>
                                        {t(
                                            "Leave empty if your server doesn't require authentication."
                                        )}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <div className="relative w-[280px]">
                                    <Input
                                        type={showApiKey ? 'text' : 'password'}
                                        value={apiKeyValue}
                                        onChange={(e) =>
                                            setApiKeyValue(e.target.value)
                                        }
                                        onBlur={handleApiKeyBlur}
                                        placeholder="sk-..."
                                        className="w-full pr-10"
                                    />
                                    <button
                                        type="button"
                                        onClick={() =>
                                            setShowApiKey(!showApiKey)
                                        }
                                        className="absolute right-3 top-1/2 -translate-y-1/2 text-zinc-400 hover:text-zinc-300 transition-colors cursor-pointer"
                                    >
                                        {showApiKey ? (
                                            <EyeOff className="w-4 h-4" />
                                        ) : (
                                            <Eye className="w-4 h-4" />
                                        )}
                                    </button>
                                </div>
                            </SettingsUI.Item>

                            {(remoteError ||
                                remoteUrl.length > 0 ||
                                isInsecureRemoteUrl(remoteUrl)) && (
                                <div className="px-4 pb-3 flex flex-col gap-1">
                                    {remoteError && (
                                        <div className="flex items-center gap-1.5 text-xs text-red-400">
                                            <AlertCircle className="w-3 h-3 flex-shrink-0" />
                                            {remoteError}
                                        </div>
                                    )}
                                    {remoteUrl.length > 0 && (
                                        <div className="flex items-center gap-1.5 text-xs text-amber-500">
                                            <AlertTriangle className="w-3 h-3 flex-shrink-0" />
                                            {t(
                                                'Your transcriptions are sent to a third-party server and are not protected by local privacy.'
                                            )}
                                        </div>
                                    )}
                                    {isInsecureRemoteUrl(remoteUrl) && (
                                        <div className="flex items-center gap-1.5 text-xs text-amber-500">
                                            <AlertTriangle className="w-3 h-3 flex-shrink-0" />
                                            {t(
                                                'This connection is not encrypted. Your data could be intercepted.'
                                            )}
                                        </div>
                                    )}
                                </div>
                            )}
                        </SettingsUI.Container>
                    </section>
                </>
            )}

            {/* Reset Tutorial */}
            <div className="flex justify-center">
                <Page.SecondaryButton
                    onClick={onResetOnboarding}
                    size="sm"
                    variant="ghost"
                    className="text-zinc-400 hover:text-zinc-300"
                >
                    {t('Reset Tutorial')}
                </Page.SecondaryButton>
            </div>
        </div>
    );
};
