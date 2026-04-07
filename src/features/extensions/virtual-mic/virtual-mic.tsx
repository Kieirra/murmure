import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { Switch } from '@/components/switch';
import { useVirtualMicState } from '@/features/settings/system/virtual-mic-settings/hooks/use-virtual-mic-state';
import { VirtualMicSettings } from '@/features/settings/system/virtual-mic-settings/virtual-mic-settings';
import { VirtualMicCta } from './virtual-mic-cta/virtual-mic-cta';
import { useTranslation } from '@/i18n';
import { Smartphone } from 'lucide-react';
import clsx from 'clsx';

export const VirtualMic = () => {
    const { t } = useTranslation();
    const { virtualMicEnabled, setVirtualMicEnabled } = useVirtualMicState();

    return (
        <main>
            <div className="space-y-4">
                <Page.Header>
                    <Typography.MainTitle data-testid="virtual-mic-title">
                        {t('Virtual Mic')}
                        <span className="ml-2 align-middle text-xs font-medium px-2 py-0.5 rounded-full bg-sky-500/15 text-sky-400 border border-sky-500/30">
                            {t('Beta')}
                        </span>
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-muted-foreground">
                        {t('Use your smartphone as a wireless microphone and touchpad. You can also tap the transcription to copy it into any app.')}
                    </Typography.Paragraph>
                </Page.Header>

                <section>
                    <SettingsUI.Container
                        className={clsx(
                            virtualMicEnabled
                                ? 'border-emerald-400/60 bg-linear-to-r from-cyan-800/40 to-emerald-700/50'
                                : 'border-sky-400/60 bg-linear-to-r from-sky-800/50 to-indigo-800/40'
                        )}
                    >
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title className="flex items-center gap-2">
                                    <Smartphone className="w-4 h-4 text-muted-foreground" />
                                    {t('Virtual Mic Remote')}
                                </Typography.Title>
                                <Typography.Paragraph>
                                    {t(
                                        'Use your smartphone as a wireless microphone and touchpad via your local network.'
                                    )}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch data-testid="virtual-mic-toggle" checked={virtualMicEnabled} onCheckedChange={setVirtualMicEnabled} />
                        </SettingsUI.Item>
                    </SettingsUI.Container>
                </section>

                {virtualMicEnabled ? (
                    <section>
                        <Typography.Title className="p-2 font-semibold text-sky-400!">
                            {t('Connection')}
                        </Typography.Title>
                        <SettingsUI.Container>
                            <VirtualMicSettings />
                        </SettingsUI.Container>
                    </section>
                ) : (
                    <VirtualMicCta />
                )}
            </div>
        </main>
    );
};
