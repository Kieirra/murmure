import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { SmartMicSettings } from '@/features/settings/system/smartmic-settings/smartmic-settings';
import { useTranslation } from '@/i18n';
import { Smartphone } from 'lucide-react';

export const SmartSpeechMic = () => {
    const { t } = useTranslation();
    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle data-testid="smart-speech-mic-title">
                        {t('Smart Speech Mic')}
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-muted-foreground">
                        {t('Use your smartphone as a wireless microphone and touchpad.')}
                    </Typography.Paragraph>
                </Page.Header>

                <div className="flex justify-center">
                    <div className="w-full space-y-6">
                        <SettingsUI.Section
                            title={t('SmartMic Remote')}
                            icon={Smartphone}
                            badge={<SettingsUI.BadgeExperimental label={t('Experimental')} />}
                        >
                            <SmartMicSettings />
                        </SettingsUI.Section>
                    </div>
                </div>
            </div>
        </main>
    );
};
