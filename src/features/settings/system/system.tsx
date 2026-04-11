import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { APISettings } from './api-settings/api-settings';
import { OverlaySettings } from './overlay-settings/overlay-settings';
import { StartOnBootSettings } from './start-on-boot-settings/start-on-boot-settings';
import { DockSettings } from './dock-settings/dock-settings';
import { isMac } from '@/utils/os';
import { CopyToClipboardSettings } from './copy-to-clipboard-settings/copy-to-clipboard-settings';
import { PasteMethodSettings } from './paste-method-settings/paste-method-settings';
import { HistorySettings } from './history-settings/history-settings';
import { LanguageSettings } from './language-settings/language-settings';
import { SoundSettings } from './sound-settings/sound-settings';
import { MicSettings } from './mic-settings/mic-settings';
import { useTranslation } from '@/i18n';
import { RecordModeSettings } from '@/features/settings/system/record-mode-settings/record-mode-settings.tsx';
import { LogLevelSettings } from './log-level-settings/log-level-settings';
import { Settings, Zap, Wrench, Monitor } from 'lucide-react';

export const System = () => {
    const { t } = useTranslation();
    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle data-testid="system-title">{t('System')}</Typography.MainTitle>
                    <Typography.Paragraph className="text-muted-foreground">
                        {t("Adjust system preferences to control Murmure's behavior at startup and more.")}
                    </Typography.Paragraph>
                </Page.Header>

                <div className="flex justify-center">
                    <div className="w-full space-y-6">
                        <SettingsUI.Section title={t('General')} icon={Settings}>
                            <LanguageSettings />
                            <SettingsUI.Separator />
                            <MicSettings />
                            <SettingsUI.Separator />
                            <RecordModeSettings />
                            <SettingsUI.Separator />
                            <PasteMethodSettings />
                            <SettingsUI.Separator />
                            <StartOnBootSettings />
                            {isMac && (
                                <>
                                    <SettingsUI.Separator />
                                    <DockSettings />
                                </>
                            )}
                            <SettingsUI.Separator />
                            <HistorySettings />
                            <SettingsUI.Separator />
                            <SoundSettings />
                        </SettingsUI.Section>

                        <SettingsUI.Section title={t('Overlay')} icon={Monitor}>
                            <OverlaySettings />
                        </SettingsUI.Section>

                        <SettingsUI.Section
                            title={t('Local API')}
                            icon={Zap}
                            badge={<SettingsUI.BadgeExperimental label={t('Experimental')} />}
                        >
                            <APISettings />
                        </SettingsUI.Section>

                        <SettingsUI.Section title={t('Advanced')} icon={Wrench}>
                            <LogLevelSettings />
                            <SettingsUI.Separator />
                            <CopyToClipboardSettings />
                        </SettingsUI.Section>
                    </div>
                </div>
            </div>
        </main>
    );
};
