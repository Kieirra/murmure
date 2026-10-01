import { AlertTriangle, Zap } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { ExtensionActiveCard } from '@/components/extension-active-card';
import { useShortcut, SHORTCUT_CONFIGS } from '@/features/settings/shortcuts/hooks/use-shortcut';
import { useLLMConnectPage } from '@/features/extensions/llm-connect/hooks/use-llm-connect-page';
import { LLMSetup } from '@/features/extensions/llm-connect/llm-setup/llm-setup';
import { LLMServerSettings } from '@/features/extensions/llm-connect/llm-server-settings/llm-server-settings';
import { ProviderModelPicker } from '@/features/extensions/llm-connect/provider-model-picker/provider-model-picker';
import { CommandModeCta } from './command-mode-cta/command-mode-cta';
import { CommandTriggers } from './command-triggers/command-triggers';
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

    const header = (
        <Page.Header>
            <Typography.MainTitle className="flex items-center gap-2" data-testid="command-mode-title">
                <Zap className="w-6 h-6 text-sky-400" />
                {t('Command Mode')}
            </Typography.MainTitle>
            <Typography.Paragraph>
                {t('Speak an instruction, the model applies it or answers you.')}
            </Typography.Paragraph>
        </Page.Header>
    );

    if (!page.isEnabled) {
        return (
            <main>
                <div className="space-y-6">
                    {header}
                    <CommandModeCta onEnable={() => void page.handleEnable()} />
                </div>
            </main>
        );
    }

    return (
        <main>
            <div className="space-y-6">
                {header}

                <ExtensionActiveCard
                    icon={Zap}
                    label={t('Command Mode is active')}
                    checked={page.isEnabled}
                    onCheckedChange={(checked) => void page.setEnabled(checked)}
                    testId="command-mode-toggle"
                />

                <CommandTriggers shortcut={shortcut} />

                <OverlayPreview shortcut={shortcut} />

                <SettingsUI.Container>
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
                </SettingsUI.Container>

                <LLMServerSettings page={page} />
            </div>
        </main>
    );
};
