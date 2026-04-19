import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { ExtensionActiveCard } from '@/components/extension-active-card';
import { useSmartMicServer } from './hooks/use-smart-mic-server';
import { usePairedDevices } from './hooks/use-paired-devices';
import { useRelayConfig } from './hooks/use-relay-config';
import { useBindAddress } from './hooks/use-bind-address';
import { useTokenTtl } from './hooks/use-token-ttl';
import { SmartMicSettings } from './smart-mic-settings';
import { SmartMicQrHero } from './smart-mic-qr-hero/smart-mic-qr-hero';
import { SmartMicCta } from './smart-mic-cta/smart-mic-cta';
import { useTranslation } from '@/i18n';
import { Smartphone } from 'lucide-react';

export const SmartMic = () => {
    const { t } = useTranslation();

    const server = useSmartMicServer();
    const pairedDevices = usePairedDevices({ enabled: server.enabled });
    const relay = useRelayConfig({
        enabled: server.enabled === true,
        onChange: server.restart,
        onMachineIdBlurChange: server.loadQrCode,
    });
    const bindAddress = useBindAddress({
        enabled: server.enabled === true,
        onChange: server.restart,
    });
    const tokenTtl = useTokenTtl();

    const handleResetTokens = async () => {
        await pairedDevices.resetTokens();
        await server.loadQrCode();
    };

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
                        {t('Dictate, translate and control your PC from your phone.')}
                    </Typography.Paragraph>
                </Page.Header>

                {server.enabled === true && (
                    <>
                        <ExtensionActiveCard
                            icon={Smartphone}
                            label={t('Smart Mic is active')}
                            checked={server.enabled}
                            onCheckedChange={server.setEnabled}
                            testId="smart-mic-toggle"
                        />

                        <section>
                            <SmartMicQrHero qrCodeDataUri={server.qrCodeDataUri} resetTokens={handleResetTokens} />
                            <SettingsUI.Container>
                                <SmartMicSettings
                                    pairedDevices={{
                                        devices: pairedDevices.devices,
                                        remove: pairedDevices.remove,
                                    }}
                                    server={{
                                        port: server.port,
                                        setPort: server.setPort,
                                    }}
                                    bindAddress={{
                                        bindAddress: bindAddress.bindAddress,
                                        availableInterfaces: bindAddress.availableInterfaces,
                                        setBindAddress: bindAddress.setBindAddress,
                                    }}
                                    tokenTtl={{
                                        tokenTtlHours: tokenTtl.tokenTtlHours,
                                        setTokenTtlHours: tokenTtl.setTokenTtlHours,
                                    }}
                                    relay={{
                                        relayEnabled: relay.relayEnabled,
                                        relayUrl: relay.relayUrl,
                                        machineIdEnabled: relay.machineIdEnabled,
                                        machineId: relay.machineId,
                                        setRelayEnabled: relay.setRelayEnabled,
                                        setRelayUrl: relay.setRelayUrl,
                                        setMachineIdEnabled: relay.setMachineIdEnabled,
                                        setMachineId: relay.setMachineId,
                                        handleRelayUrlBlur: relay.handleRelayUrlBlur,
                                        handleMachineIdBlur: relay.handleMachineIdBlur,
                                    }}
                                />
                            </SettingsUI.Container>
                        </section>
                    </>
                )}
                {server.enabled === false && <SmartMicCta onEnable={() => server.setEnabled(true)} />}
            </div>
        </main>
    );
};
