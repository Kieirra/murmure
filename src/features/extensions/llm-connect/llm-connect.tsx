import clsx from 'clsx';
import { Sparkles } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { TAB_PANEL_CLASS } from '@/components/tabs.helpers';
import { ExtensionCtaLayout } from '@/features/extensions/extension-cta-layout/extension-cta-layout';
import { ExtensionHeader } from '@/features/extensions/extension-header/extension-header';
import { useLLMConnectPage } from './hooks/use-llm-connect-page';
import { LLMSetup } from './llm-setup/llm-setup';
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
            <ExtensionCtaLayout colorScheme="llm">
                <ExtensionHeader
                    icon={Sparkles}
                    title={t('Prompt Mode')}
                    colorScheme="llm"
                    titleTestId="llm-connect-title"
                    subtitle={t('Dictate naturally, get clean or rewritten text.')}
                />
                <LLMConnectCta onEnable={() => void page.handleEnable()} needsSetup={!settings.onboarding_completed} />
            </ExtensionCtaLayout>
        );
    }

    return (
        <ExtensionCtaLayout colorScheme="llm" haloIntensity={12}>
            <ExtensionHeader
                icon={Sparkles}
                title={t('Prompt Mode')}
                colorScheme="llm"
                titleTestId="llm-connect-title"
                toggle={{
                    checked: page.isEnabled,
                    onCheckedChange: (checked) => void page.setEnabled(checked),
                    ariaLabel: t('Enable Prompt Mode'),
                    testId: 'llm-connect-toggle',
                }}
            />

            <div>
                <ModeTabs
                    modes={settings.modes}
                    activeModeIndex={activeModeIndex}
                    models={models}
                    updateSettings={updateSettings}
                />

                {activeMode && (
                    <div className={clsx(TAB_PANEL_CLASS, activeModeIndex === 0 && 'rounded-tl-none')}>
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
                    </div>
                )}
            </div>

            {activeMode && <LLMServerSettings page={page} />}
        </ExtensionCtaLayout>
    );
};
