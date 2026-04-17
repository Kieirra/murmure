import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { ExtensionActiveCard } from '@/components/extension-active-card';
import { useSmartMicState } from './hooks/use-smart-mic-state';
import { SmartMicSettings } from './smart-mic-settings';
import { SmartMicQrHero } from './smart-mic-qr-hero/smart-mic-qr-hero';
import { SmartMicCta } from './smart-mic-cta/smart-mic-cta';
import { useTranslation } from '@/i18n';
import { Smartphone } from 'lucide-react';

export const SmartMic = () => {
    const { t } = useTranslation();
    const { smartMicEnabled, setSmartMicEnabled } = useSmartMicState();

    return (
        <main>
            <div className="space-y-4">
                <Page.Header>
                    <Typography.MainTitle data-testid="smart-mic-title">
                        {t('Smart Mic')}
                        <span className="ml-2 align-middle text-xs font-medium px-2 py-0.5 rounded-full bg-sky-500/15 text-sky-400 border border-sky-500/30">
                            {t('Beta')}
                        </span>
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-muted-foreground">
                        {t('Your phone, part of Murmure.')}
                    </Typography.Paragraph>
                </Page.Header>

                {smartMicEnabled ? (
                    <>
                        <ExtensionActiveCard
                            icon={Smartphone}
                            label={t('Smart Mic is active')}
                            checked={smartMicEnabled}
                            onCheckedChange={setSmartMicEnabled}
                            testId="smart-mic-toggle"
                        />

                        <section>
                            <SmartMicQrHero />
                            <SettingsUI.Container>
                                <SmartMicSettings />
                            </SettingsUI.Container>
                        </section>
                    </>
                ) : (
                    <SmartMicCta onEnable={() => setSmartMicEnabled(true)} />
                )}
            </div>
        </main>
    );
};
