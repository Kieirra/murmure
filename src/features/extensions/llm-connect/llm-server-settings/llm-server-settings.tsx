import { LLMAdvancedSettings } from '../llm-advanced-settings/llm-advanced-settings';
import { LLMConnectPage } from '../hooks/use-llm-connect-page';

interface LLMServerSettingsProps {
    page: LLMConnectPage;
}

export const LLMServerSettings = ({ page }: LLMServerSettingsProps) => {
    return (
        <LLMAdvancedSettings
            url={page.settings.url}
            onUrlChange={(url) => page.updateSettings({ url })}
            onTestConnection={page.handleTestConnection}
            localConnectionStatus={page.connectionStatus}
            onInstallModel={() => page.setShowModelSelector(true)}
            remoteUrl={page.settings.remote_url}
            onRemoteUrlChange={(remote_url) => page.updateSettings({ remote_url })}
            onTestRemoteConnection={page.handleTestRemoteConnection}
            remoteConnectionStatus={page.remoteConnectionStatus}
            onApiKeyChange={page.storeRemoteApiKey}
            showInstallModel={page.showInstallModel}
        />
    );
};
