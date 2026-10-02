import { LLMConnectOnboarding } from '../onboarding/llm-connect-onboarding';
import { LLMConnectPage } from '../hooks/use-llm-connect-page';

interface LLMSetupProps {
    page: LLMConnectPage;
}

export const LLMSetup = ({ page }: LLMSetupProps) => {
    return (
        <LLMConnectOnboarding
            testConnection={page.testConnection}
            pullModel={page.pullModel}
            updateSettings={page.updateSettings}
            models={page.models}
            fetchModels={page.fetchModels}
            isInstallOnly={page.showModelSelector}
            completeOnboarding={
                page.showModelSelector
                    ? async () => {
                          await page.fetchModels();
                          page.setShowModelSelector(false);
                      }
                    : page.completeSetup
            }
            remoteModels={page.remoteModels}
            testRemoteConnection={page.testRemoteConnection}
            fetchRemoteModels={page.fetchRemoteModels}
            storeRemoteApiKey={page.storeRemoteApiKey}
        />
    );
};
