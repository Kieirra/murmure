import { useTranslation } from '@/i18n';
import { Page } from '@/components/page';
import { ChevronDown, ChevronUp, Settings2 } from 'lucide-react';
import { ConnectionStatus } from '../hooks/use-llm-connect';
import { useAdvancedSettings } from './hooks/use-advanced-settings';
import { LocalServerSection } from './local-server-section/local-server-section';
import { RemoteServerSection } from './remote-server-section/remote-server-section';

interface LLMAdvancedSettingsProps {
    url: string;
    onUrlChange: (url: string) => void;
    onTestConnection: (url: string) => Promise<void>;
    localConnectionStatus: ConnectionStatus;
    onInstallModel: () => void;
    onResetOnboarding: () => void;
    remoteUrl: string;
    onRemoteUrlChange: (url: string) => void;
    onTestRemoteConnection: (url: string) => Promise<number>;
    remoteConnectionStatus: ConnectionStatus;
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

    const {
        isOpen,
        toggleOpen,
        showApiKey,
        toggleShowApiKey,
        apiKeyValue,
        isTesting,
        isTestingLocal,
        remoteError,
        remoteModelCount,
        localUrl,
        setLocalUrl,
        localRemoteUrl,
        setLocalRemoteUrl,
        handleApiKeyChange,
        handleApiKeyBlur,
        handleLocalUrlBlur,
        handleRemoteUrlBlur,
        handleTestLocal,
        handleTestRemote,
    } = useAdvancedSettings({
        url,
        remoteUrl,
        onUrlChange,
        onRemoteUrlChange,
        onTestConnection,
        onTestRemoteConnection,
        onApiKeyChange,
    });

    return (
        <div className="mb-6 flex flex-col gap-2">
            <button
                type="button"
                onClick={toggleOpen}
                className="flex items-center gap-2 text-sm font-medium text-foreground hover:text-foreground transition-colors w-fit cursor-pointer"
            >
                <Settings2 className="w-4 h-4" />
                {t('Advanced configuration')}
                {isOpen ? (
                    <ChevronUp className="w-4 h-4 text-muted-foreground" />
                ) : (
                    <ChevronDown className="w-4 h-4 text-muted-foreground" />
                )}
            </button>

            {isOpen && (
                <>
                    <LocalServerSection
                        localUrl={localUrl}
                        onLocalUrlChange={setLocalUrl}
                        onLocalUrlBlur={handleLocalUrlBlur}
                        isTestingLocal={isTestingLocal}
                        localConnectionStatus={localConnectionStatus}
                        onTestLocal={handleTestLocal}
                        showInstallModel={showInstallModel}
                        onInstallModel={onInstallModel}
                    />
                    <RemoteServerSection
                        localRemoteUrl={localRemoteUrl}
                        onRemoteUrlChange={setLocalRemoteUrl}
                        onRemoteUrlBlur={handleRemoteUrlBlur}
                        isTesting={isTesting}
                        remoteConnectionStatus={remoteConnectionStatus}
                        onTestRemote={handleTestRemote}
                        remoteModelCount={remoteModelCount}
                        remoteError={remoteError}
                        apiKeyValue={apiKeyValue}
                        onApiKeyChange={handleApiKeyChange}
                        onApiKeyBlur={handleApiKeyBlur}
                        showApiKey={showApiKey}
                        onToggleShowApiKey={toggleShowApiKey}
                    />
                </>
            )}

            <div className="flex justify-center">
                <Page.SecondaryButton
                    onClick={onResetOnboarding}
                    size="sm"
                    variant="ghost"
                    className="text-muted-foreground hover:text-foreground"
                >
                    {t('Reset Tutorial')}
                </Page.SecondaryButton>
            </div>
        </div>
    );
};
