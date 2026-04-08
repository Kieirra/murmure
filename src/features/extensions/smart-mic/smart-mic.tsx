import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { Switch } from '@/components/switch';
import { useSmartMicState } from '@/features/settings/system/smart-mic-settings/hooks/use-smart-mic-state';
import { SmartMicSettings } from '@/features/settings/system/smart-mic-settings/smart-mic-settings';
import { SmartMicCta } from './smart-mic-cta/smart-mic-cta';
import { useTranslation } from '@/i18n';
import { Smartphone } from 'lucide-react';
import clsx from 'clsx';

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
                </Page.Header>

                <section>
                    <SettingsUI.Container
                        className={clsx(
                            smartMicEnabled
                                ? 'border-emerald-400/60 bg-linear-to-r from-cyan-800/40 to-emerald-700/50'
                                : 'border-sky-400/60 bg-linear-to-r from-sky-800/50 to-indigo-800/40'
                        )}
                    >
                        <SettingsUI.Item>
                            <SettingsUI.Description className="w-auto flex-1">
                                <Typography.Title className="flex items-center gap-2">
                                    <Smartphone className="w-4 h-4 text-muted-foreground" />
                                    {t('Smart Mic Remote')}
                                </Typography.Title>
                                <Typography.Paragraph>
                                    {t(
                                        'Use your smartphone as a wireless microphone and touchpad. You can also tap the transcription to copy it into any app.'
                                    )}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch data-testid="smart-mic-toggle" checked={smartMicEnabled} onCheckedChange={setSmartMicEnabled} />
                        </SettingsUI.Item>
                    </SettingsUI.Container>
                </section>

                {smartMicEnabled ? (
                    <section>
                        <Typography.Title className="p-2 font-semibold text-sky-400!">
                            {t('Connection')}
                        </Typography.Title>
                        <SettingsUI.Container>
                            <SmartMicSettings />
                        </SettingsUI.Container>
                    </section>
                ) : (
                    <SmartMicCta />
                )}
            </div>
        </main>
    );
};
