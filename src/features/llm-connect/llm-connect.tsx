import { useTranslation } from '@/i18n';
import { useState, useEffect } from 'react';
import { useLLMConnect, LLMMode } from './hooks/use-llm-connect';
import { toast } from 'react-toastify';
import { getPresetLabel, getPromptByPreset } from './llm-connect.helpers';
import { LLMConnectOnboarding } from './onboarding/llm-connect-onboarding';
import { LLMHeader } from './components/llm-header';
import { ModeTabs } from './components/mode-tabs';
import { ModeContent } from './components/mode-content';
import { LLMAdvancedSettings } from './components/llm-advanced-settings';

export const LLMConnect = () => {
    const { t, i18n } = useTranslation();
    const {
        settings,
        models,
        connectionStatus,
        isLoading,
        isSettingsLoaded,
        updateSettings,
        testConnection,
        fetchModels,
        pullModel,
    } = useLLMConnect();

    const [showModelSelector, setShowModelSelector] = useState(false);

    const activeModeIndex = settings.active_mode_index;
    const activeMode = settings.modes[activeModeIndex];

    const handleTestConnection = async () => {
        const result = await testConnection();
        if (result) {
            toast.success(t('Connection successful'), { autoClose: 1500 });
            await fetchModels();
        } else {
            toast.error(t('Connection failed'));
        }
    };

    const buildDefaultMode = (modelName: string): LLMMode => ({
        name: t(getPresetLabel('general')),
        prompt: getPromptByPreset('general', i18n.language),
        model: modelName,
        shortcut: 'Ctrl + Shift + 1',
    });

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

    useEffect(() => {
        if (
            isSettingsLoaded &&
            !settings.onboarding_completed &&
            !showModelSelector &&
            settings.model === ''
        ) {
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
        return (
            <div className="p-8 text-center text-zinc-500">
                {t('Loading...')}
            </div>
        );
    }

    if (!settings.onboarding_completed || showModelSelector) {
        return (
            <main>
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
                        if (showModelSelector) {
                            setShowModelSelector(false);
                            return;
                        }
                        await updateSettings({ onboarding_completed: true });
                    }}
                />
            </main>
        );
    }

    return (
        <main>
            <div className="space-y-6">
                <LLMHeader connectionStatus={connectionStatus} />

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
                            onRefreshModels={handleTestConnection}
                        />

                        <LLMAdvancedSettings
                            url={settings.url}
                            onUrlChange={(url) => updateSettings({ url })}
                            onTestConnection={handleTestConnection}
                            onInstallModel={() => setShowModelSelector(true)}
                            onResetOnboarding={handleResetOnboarding}
                        />
                    </>
                )}
            </div>
        </main>
    );
};
