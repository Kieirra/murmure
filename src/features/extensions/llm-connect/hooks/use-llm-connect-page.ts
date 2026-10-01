import { useTranslation } from '@/i18n';
import { useState, useEffect, useRef } from 'react';
import { toast } from 'react-toastify';
import { useLLMConnect, LLMMode } from './use-llm-connect';
import { getPresetLabel, getPromptByPreset } from '../llm-connect.helpers';

export type LLMConnectPage = ReturnType<typeof useLLMConnectPage>;

type LLMExtension = 'llm-connect' | 'command-mode';

export const useLLMConnectPage = (extension: LLMExtension) => {
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
        getLatestSettings,
        testConnection,
        testRemoteConnection,
        fetchModels,
        fetchRemoteModels,
        storeRemoteApiKey,
        pullModel,
    } = useLLMConnect();

    const [showModelSelector, setShowModelSelector] = useState(false);
    const [isSetupRequested, setIsSetupRequested] = useState(false);

    const isExtensionEnabled = extension === 'command-mode' ? settings.command.enabled : settings.enabled;
    const isEnabled = settings.onboarding_completed && isExtensionEnabled;

    const isLocalConfigured = connectionStatus === 'connected';
    const isRemoteConfigured = settings.remote_url.length > 0;

    const showInstallModel =
        settings.modes.some((m) => (m.provider ?? 'local') === 'local') || settings.command.provider === 'local';

    const handleTestConnection = async (url: string) => {
        const result = await testConnection(url);
        if (result) {
            await fetchModels(url);
        }
    };

    const handleTestRemoteConnection = async (url: string) => {
        const modelCount = await testRemoteConnection(url);
        await fetchRemoteModels(url).catch((error) => {
            console.error('Failed to fetch remote models:', error);
        });
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

    const buildDefaultMode = (): LLMMode => {
        const name = t(getPresetLabel('general'));
        return {
            name,
            prompt: getPromptByPreset('general', i18n.language),
            model: '',
            provider: 'local',
            wake_word: `alix ${name.toLowerCase()}`,
        };
    };

    const buildEnabledUpdate = (enabled: boolean) =>
        extension === 'command-mode' ? { command: { ...getLatestSettings().command, enabled } } : { enabled };

    const setEnabled = async (enabled: boolean) => {
        await updateSettings(buildEnabledUpdate(enabled));
    };

    const handleEnable = async () => {
        if (!settings.onboarding_completed) {
            setIsSetupRequested(true);
            return;
        }
        await setEnabled(true);
    };

    const completeSetup = async () => {
        await updateSettings({ onboarding_completed: true, ...buildEnabledUpdate(true) });
        setIsSetupRequested(false);
        void fetchModels().catch(() => {});
    };

    const initializedRef = useRef(false);

    useEffect(() => {
        if (initializedRef.current) return;
        if (isSettingsLoaded && !settings.onboarding_completed && !showModelSelector && settings.model === '') {
            const defaultMode = buildDefaultMode();
            const hasOneMode = settings.modes.length === 1;
            const isDefaultMode =
                hasOneMode &&
                settings.active_mode_index === 0 &&
                settings.modes[0]?.name === defaultMode.name &&
                settings.modes[0]?.prompt === defaultMode.prompt &&
                settings.modes[0]?.model === '';

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

    return {
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
        showModelSelector,
        setShowModelSelector,
        isSetupRequested,
        isEnabled,
        setEnabled,
        handleEnable,
        completeSetup,
        isLocalConfigured,
        isRemoteConfigured,
        showInstallModel,
        handleTestConnection,
        handleTestRemoteConnection,
        handleRefreshRemoteModels,
    };
};
