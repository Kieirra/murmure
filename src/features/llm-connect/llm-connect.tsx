import { useTranslation } from '@/i18n';
import { useState, useEffect, useRef } from 'react';
import { useLLMConnect, LLMMode } from './hooks/use-llm-connect';
import { toast } from 'react-toastify';
import { getPresetLabel, getPromptByPreset } from './llm-connect.helpers';
import { LLMConnectOnboarding } from './onboarding/llm-connect-onboarding';
import { LLMHeader } from './llm-header/llm-header';
import { ModeTabs } from './mode-tabs/mode-tabs';
import { ModeContent } from './mode-content/mode-content';
import { LLMAdvancedSettings } from './llm-advanced-settings/llm-advanced-settings';

export const LLMConnect = () => {
    const { t, i18n } = useTranslation();
    const {
        settings,
        models,
        connectionStatus,
        remoteModels,
        remoteConnectionStatus,
        isLoading,
        isSettingsLoaded,
        updateSettings,
        testConnection,
        testRemoteConnection,
        fetchModels,
        fetchRemoteModels,
        storeRemoteApiKey,
        pullModel,
    } = useLLMConnect();

    const [showModelSelector, setShowModelSelector] = useState(false);

    const activeModeIndex = settings.active_mode_index;
    const activeMode = settings.modes[activeModeIndex];

    const isLocalConfigured = connectionStatus === 'connected';
    const isRemoteConfigured = settings.remote_url.length > 0 && remoteConnectionStatus === 'connected';

    const showInstallModel = settings.modes.some((m) => (m.provider ?? 'local') === 'local');

    const handleTestConnection = async (url: string) => {
        const result = await testConnection(url);
        if (result) {
            await fetchModels(url);
        }
    };

    const handleTestRemoteConnection = async (url: string): Promise<number> => {
        const modelCount = await testRemoteConnection(url);
        try {
            await fetchRemoteModels(url);
        } catch (error) {
            console.error('Failed to fetch remote models:', error);
        }
        return modelCount;
    };

    const handleRefreshRemoteModels = async () => {
        try {
            await fetchRemoteModels();
        } catch {
            toast.error(t('Failed to fetch remote models'), {
                autoClose: 5000,
            });
        }
    };

    const buildDefaultMode = (modelName: string): LLMMode => {
        const name = t(getPresetLabel('general'));
        return {
            name,
            prompt: getPromptByPreset('general', i18n.language),
            model: modelName,
            shortcut: 'Ctrl + Shift + 1',
            provider: 'local',
            wake_word: `alix ${name.toLowerCase()}`,
        };
    };

    const handleResetOnboarding = async () => {
        try {
            await updateSettings({
                onboarding_completed: false,
                model: '',
                prompt: '',
                modes: [buildDefaultMode('')],
                active_mode_index: 0,
            });
        } catch {
            toast.error(t('Failed to reset onboarding'));
        }
    };

    const initializedRef = useRef(false);

    useEffect(() => {
        if (initializedRef.current) return;
        if (isSettingsLoaded && !settings.onboarding_completed && !showModelSelector && settings.model === '') {
            const defaultMode = buildDefaultMode('');
            const hasOneMode = settings.modes.length === 1;
            const isDefaultMode =
                hasOneMode &&
                settings.active_mode_index === 0 &&
                settings.modes[0]?.name === defaultMode.name &&
                settings.modes[0]?.prompt === defaultMode.prompt &&
                settings.modes[0]?.model === '' &&
                settings.modes[0]?.shortcut === defaultMode.shortcut;

            if (!isDefaultMode) {
                initializedRef.current = true;
                updateSettings({
                    model: '',
                    prompt: '',
                    modes: [defaultMode],
                    active_mode_index: 0,
                });
            }
        }
    }, [
        isSettingsLoaded,
        settings.onboarding_completed,
        settings.model,
        settings.modes,
        settings.active_mode_index,
        showModelSelector,
        i18n.language,
        updateSettings,
        t,
    ]);

    if (!isSettingsLoaded || !settings.modes || settings.modes.length === 0) {
        return null;
    }

    // Install another model flow (preserves existing configuration)
    if (showModelSelector) {
        return (
            <main>
                <LLMConnectOnboarding
                    settings={settings}
                    testConnection={testConnection}
                    pullModel={pullModel}
                    updateSettings={updateSettings}
                    models={models}
                    fetchModels={fetchModels}
                    isInstallOnly={true}
                    completeOnboarding={async () => {
                        await fetchModels();
                        setShowModelSelector(false);
                    }}
                    remoteModels={remoteModels}
                    testRemoteConnection={testRemoteConnection}
                    fetchRemoteModels={fetchRemoteModels}
                    storeRemoteApiKey={storeRemoteApiKey}
                />
            </main>
        );
    }

    // First-time setup onboarding flow
    if (!settings.onboarding_completed) {
        return (
            <main>
                <LLMConnectOnboarding
                    settings={settings}
                    testConnection={testConnection}
                    pullModel={pullModel}
                    updateSettings={updateSettings}
                    models={models}
                    fetchModels={fetchModels}
                    completeOnboarding={async () => {
                        await updateSettings({ onboarding_completed: true });
                        void fetchModels().catch(() => {});
                    }}
                    remoteModels={remoteModels}
                    testRemoteConnection={testRemoteConnection}
                    fetchRemoteModels={fetchRemoteModels}
                    storeRemoteApiKey={storeRemoteApiKey}
                />
            </main>
        );
    }

    return (
        <main>
            <div className="space-y-6">
                <LLMHeader />

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
                                void handleTestConnection(settings.url);
                            }}
                            remoteModels={remoteModels}
                            isRemoteConfigured={isRemoteConfigured}
                            isLocalConfigured={isLocalConfigured}
                            onRefreshRemoteModels={handleRefreshRemoteModels}
                        />

                        <LLMAdvancedSettings
                            url={settings.url}
                            onUrlChange={(url) => updateSettings({ url })}
                            onTestConnection={handleTestConnection}
                            localConnectionStatus={connectionStatus}
                            onInstallModel={() => setShowModelSelector(true)}
                            onResetOnboarding={handleResetOnboarding}
                            remoteUrl={settings.remote_url}
                            onRemoteUrlChange={(remote_url) => updateSettings({ remote_url })}
                            onTestRemoteConnection={handleTestRemoteConnection}
                            remoteConnectionStatus={remoteConnectionStatus}
                            onApiKeyChange={storeRemoteApiKey}
                            showInstallModel={showInstallModel}
                        />
                    </>
                )}
            </div>
        </main>
    );
};
