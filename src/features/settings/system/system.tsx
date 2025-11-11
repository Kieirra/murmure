import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { APISettings } from './api-settings/api-settings';
import { OverlaySettings } from './overlay-settings/overlay-settings';
import { StartOnBootSettings } from './start-on-boot-settings/start-on-boot-settings';
import { CopyToClipboardSettings } from './copy-to-clipboard-settings/copy-to-clipboard-settings';
import { HistorySettings } from './history-settings/history-settings';
import { LanguageSettings } from './language-settings/language-settings';
import { useTranslation } from '@/i18n';

export const System = () => {
    const { t } = useTranslation('settings');
    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle>
                        {t('system.title')}
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        {t('system.description')}
                    </Typography.Paragraph>
                </Page.Header>

                <div className="flex justify-center mb-8">
                    <SettingsUI.Container>
                        <LanguageSettings />
                        <SettingsUI.Separator />
                        <StartOnBootSettings />
                        <SettingsUI.Separator />
                        <CopyToClipboardSettings />
                        <SettingsUI.Separator />
                        <HistorySettings />
                        <SettingsUI.Separator />
                        <OverlaySettings />
                        <SettingsUI.Separator />
                        <APISettings />
                    </SettingsUI.Container>
                </div>
            </div>
        </main>
    );
};
