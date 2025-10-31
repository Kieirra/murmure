import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { APISettings } from './api-settings/api-settings';
import { OverlaySettings } from './overlay-settings/overlay-settings';
import { StartOnBootSettings } from './start-on-boot-settings/start-on-boot-settings';
import { CopyToClipboardSettings } from './copy-to-clipboard-settings/copy-to-clipboard-settings';

export const System = () => {
    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle>System</Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        Adjust system preferences to control Murmure's behavior
                        at startup and more.
                    </Typography.Paragraph>
                </Page.Header>

                <div className="flex justify-center">
                    <SettingsUI.Container>
                        <StartOnBootSettings />
                        <SettingsUI.Separator />
                        <CopyToClipboardSettings />
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
