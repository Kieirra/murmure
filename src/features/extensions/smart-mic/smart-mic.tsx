import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { Switch } from '@/components/switch';
import { useSmartMicState } from './hooks/use-smart-mic-state';
import { SmartMicSettings } from './smart-mic-settings';
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
                        {t('Your phone, part of Murmure. Over local WiFi only.')}
                    </Typography.Paragraph>
                </Page.Header>

                {smartMicEnabled ? (
                    <>
                        <section>
                            <SettingsUI.Container className="border-emerald-400/40 bg-linear-to-r from-emerald-900/20 to-transparent">
                                <SettingsUI.Item>
                                    <SettingsUI.Description>
                                        <Typography.Title className="flex items-center gap-2">
                                            <Smartphone className="w-4 h-4 text-emerald-400" />
                                            {t('Smart Mic is active')}
                                        </Typography.Title>
                                    </SettingsUI.Description>
                                    <Switch
                                        checked={smartMicEnabled}
                                        onCheckedChange={setSmartMicEnabled}
                                        data-testid="smart-mic-toggle"
                                    />
                                </SettingsUI.Item>
                            </SettingsUI.Container>
                        </section>

                        <section>
                            <Typography.Title className="p-2 font-semibold text-sky-400!">
                                {t('Connection')}
                            </Typography.Title>
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
