import { AlertTriangle, Brain, Zap } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { SettingsUI } from '@/components/settings-ui';
import { useShortcut, SHORTCUT_CONFIGS } from '@/features/settings/shortcuts/hooks/use-shortcut';
import { useLLMConnectPage } from '@/features/extensions/llm-connect/hooks/use-llm-connect-page';
import { LLMSetup } from '@/features/extensions/llm-connect/llm-setup/llm-setup';
import { LLMServerSettings } from '@/features/extensions/llm-connect/llm-server-settings/llm-server-settings';
import { ProviderModelPicker } from '@/features/extensions/llm-connect/provider-model-picker/provider-model-picker';
import { ExtensionCtaLayout } from '@/features/extensions/extension-cta-layout/extension-cta-layout';
import { ExtensionHeader } from '@/features/extensions/extension-header/extension-header';
import { CommandModeCta } from './command-mode-cta/command-mode-cta';
import { OverlayPreview } from './overlay-preview/overlay-preview';

export const CommandMode = () => {
    const { t } = useTranslation();
    const page = useLLMConnectPage('command-mode');
    const { shortcut } = useShortcut(SHORTCUT_CONFIGS.command);

    if (!page.isSettingsLoaded) {
        return null;
    }

    if (page.showModelSelector || page.isSetupRequested) {
        return (
            <main>
                <LLMSetup page={page} />
            </main>
        );
    }

    const { command } = page.settings;

    if (!page.isEnabled) {
        return (
            <ExtensionCtaLayout colorScheme="command">
                <ExtensionHeader
                    icon={Zap}
                    title={t('Command Mode')}
                    colorScheme="command"
                    titleTestId="command-mode-title"
                    subtitle={t('Select text, then say what to do with it.')}
                />
                <CommandModeCta
                    onEnable={() => void page.handleEnable()}
                    needsSetup={!page.settings.onboarding_completed}
                />
            </ExtensionCtaLayout>
        );
    }

    return (
        <ExtensionCtaLayout colorScheme="command" haloIntensity={12}>
            <ExtensionHeader
                icon={Zap}
                title={t('Command Mode')}
                colorScheme="command"
                titleTestId="command-mode-title"
                toggle={{
                    checked: page.isEnabled,
                    onCheckedChange: (checked) => void page.setEnabled(checked),
                    ariaLabel: t('Enable Command Mode'),
                    testId: 'command-mode-toggle',
                }}
            />

            <OverlayPreview shortcut={shortcut} />

            <SettingsUI.Section title={t('Model')} icon={Brain}>
                <ProviderModelPicker
                    provider={command.provider}
                    model={command.model}
                    onChange={(value) => page.updateSettings({ command: { ...command, ...value } })}
                    models={page.models}
                    remoteModels={page.remoteModels}
                    isLoading={page.isLoading}
                    isLocalConfigured={page.isLocalConfigured}
                    isRemoteConfigured={page.isRemoteConfigured}
                    onRefreshModels={() => void page.handleTestConnection(page.settings.url)}
                    onRefreshRemoteModels={page.handleRefreshRemoteModels}
                />
                {command.model.trim().length === 0 && (
                    <div className="flex items-center gap-1.5 px-4 pb-4 text-xs text-yellow-300">
                        <AlertTriangle className="w-3 h-3 flex-shrink-0" />
                        {t('Choose a model to use the command.')}
                    </div>
                )}
            </SettingsUI.Section>

            <LLMServerSettings page={page} />
        </ExtensionCtaLayout>
    );
};
