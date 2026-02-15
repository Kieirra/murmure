import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { APISettings } from './api-settings/api-settings';
import { OverlaySettings } from './overlay-settings/overlay-settings';
import { StartOnBootSettings } from './start-on-boot-settings/start-on-boot-settings';
import { CopyToClipboardSettings } from './copy-to-clipboard-settings/copy-to-clipboard-settings';
import { PasteMethodSettings } from './paste-method-settings/paste-method-settings';
import { AutoSendEnterSettings } from './auto-send-enter-settings/auto-send-enter-settings';
import { HistorySettings } from './history-settings/history-settings';
import { LanguageSettings } from './language-settings/language-settings';
import { SoundSettings } from './sound-settings/sound-settings';
import { MicSettings } from './mic-settings/mic-settings';
import { useTranslation } from '@/i18n';
import { RecordModeSettings } from '@/features/settings/system/record-mode-settings/record-mode-settings.tsx';

import { LogLevelSettings } from './log-level-settings/log-level-settings';

export const System = () => {
    const { t } = useTranslation();
    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle data-testid="system-title">
                        {t('System')}
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        {t(
                            "Adjust system preferences to control Murmure's behavior at startup and more."
                        )}
                    </Typography.Paragraph>
                </Page.Header>

                <div className="flex justify-center mb-8">
                    <SettingsUI.Container>
                        <LanguageSettings />
                        <SettingsUI.Separator />
                        <MicSettings />
                        <SettingsUI.Separator />
                        <RecordModeSettings />
                        <SettingsUI.Separator />
                        <PasteMethodSettings />
                        <SettingsUI.Separator />
                        <AutoSendEnterSettings />
                        <SettingsUI.Separator />
                        <StartOnBootSettings />
                        <SettingsUI.Separator />
                        <HistorySettings />
                        <SettingsUI.Separator />
                        <SoundSettings />
                        <SettingsUI.Separator />
                        <OverlaySettings />
                        <SettingsUI.Separator />
                        <APISettings />
                        <SettingsUI.Separator />
                        <LogLevelSettings />
                        <SettingsUI.Separator />
                        <CopyToClipboardSettings />
                    </SettingsUI.Container>
                </div>
            </div>
        </main>
    );
};
