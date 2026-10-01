import { Sparkles } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { ExtensionActiveCard } from '@/components/extension-active-card';
import { useLLMConnectPage } from './hooks/use-llm-connect-page';
import { LLMSetup } from './llm-setup/llm-setup';
import { LLMHeader } from './llm-header/llm-header';
import { LLMConnectCta } from './llm-connect-cta/llm-connect-cta';
import { ModeTabs } from './mode-tabs/mode-tabs';
import { ModeContent } from './mode-content/mode-content';
import { LLMServerSettings } from './llm-server-settings/llm-server-settings';

export const LLMConnect = () => {
    const { t } = useTranslation();
    const page = useLLMConnectPage('llm-connect');
    const { settings, models, remoteModels, isLoading, isSettingsLoaded, updateSettings } = page;

    const activeModeIndex = settings.active_mode_index;
    const activeMode = settings.modes[activeModeIndex];

    if (!isSettingsLoaded || settings.modes.length === 0) {
        return null;
    }

    if (page.showModelSelector || page.isSetupRequested) {
        return (
            <main>
                <LLMSetup page={page} />
            </main>
        );
    }

    if (!page.isEnabled) {
        return (
            <main>
                <div className="space-y-6">
                    <LLMHeader />
                    <LLMConnectCta
                        onEnable={() => void page.handleEnable()}
                        needsSetup={!settings.onboarding_completed}
                    />
                </div>
            </main>
        );
    }

    return (
        <main>
            <div className="space-y-6">
                <LLMHeader />

                <ExtensionActiveCard
                    icon={Sparkles}
                    label={t('Prompt Mode is active')}
                    checked={page.isEnabled}
                    onCheckedChange={(checked) => void page.setEnabled(checked)}
                    testId="llm-connect-toggle"
                />

                <ModeTabs
                    modes={settings.modes}
                    activeModeIndex={activeModeIndex}
                    models={models}
                    updateSettings={updateSettings}
                />

                {activeMode && (
                    <>
                        <ModeContent
                            activeMode={activeMode}
                            activeModeIndex={activeModeIndex}
                            modes={settings.modes}
                            models={models}
                            isLoading={isLoading}
                            updateSettings={updateSettings}
                            onRefreshModels={() => {
                                void page.handleTestConnection(settings.url);
                            }}
                            remoteModels={remoteModels}
                            isRemoteConfigured={page.isRemoteConfigured}
                            isLocalConfigured={page.isLocalConfigured}
                            onRefreshRemoteModels={page.handleRefreshRemoteModels}
                        />

                        <LLMServerSettings page={page} />
                    </>
                )}
            </div>
        </main>
    );
};
