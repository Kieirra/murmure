import { ExtensionCtaLayout } from '@/features/extensions/extension-cta-layout/extension-cta-layout';
import { ExtensionHeader } from '@/features/extensions/extension-header/extension-header';
import { useSmartMicServer } from './hooks/use-smart-mic-server';
import { usePairedDevices } from './hooks/use-paired-devices';
import { useRelayConfig } from './hooks/use-relay-config';
import { useBindAddress } from './hooks/use-bind-address';
import { useTokenTtl } from './hooks/use-token-ttl';
import { SmartMicSettings } from './smart-mic-settings';
import { SmartMicQrHero } from './smart-mic-qr-hero';
import { SmartMicCta } from './smart-mic-cta/smart-mic-cta';
import { SmartMicWaylandNotice } from './smart-mic-wayland-notice/smart-mic-wayland-notice';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { useTranslation } from '@/i18n';
import { Smartphone } from 'lucide-react';

export const SmartMic = () => {
    const { t } = useTranslation();
    const isWayland = useIsWayland();

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

    if (!isWayland && server.enabled === true) {
        return (
            <ExtensionCtaLayout colorScheme="standard" haloIntensity={12}>
                <ExtensionHeader
                    icon={Smartphone}
                    title={t('Smart Mic')}
                    colorScheme="standard"
                    titleTestId="smart-mic-title"
                    badge={t('Beta')}
                    toggle={{
                        checked: server.enabled,
                        onCheckedChange: server.saveEnabled,
                        ariaLabel: t('Enable Smart Mic'),
                        testId: 'smart-mic-toggle',
                    }}
                />

                <SmartMicQrHero qrCodeDataUri={server.qrCodeDataUri} resetTokens={handleResetTokens} />
                <SmartMicSettings
                    pairedDevices={{
                        devices: pairedDevices.devices,
                        remove: pairedDevices.remove,
                    }}
                    server={{
                        port: server.port,
                        setPort: server.savePort,
                    }}
                    bindAddress={{
                        bindAddress: bindAddress.bindAddress,
                        availableInterfaces: bindAddress.availableInterfaces,
                        setBindAddress: bindAddress.saveBindAddress,
                    }}
                    tokenTtl={{
                        tokenTtlHours: tokenTtl.tokenTtlHours,
                        setTokenTtlHours: tokenTtl.saveTokenTtlHours,
                    }}
                    relay={{
                        relayEnabled: relay.relayEnabled,
                        relayUrl: relay.relayUrl,
                        machineIdEnabled: relay.machineIdEnabled,
                        machineId: relay.machineId,
                        setRelayEnabled: relay.saveRelayEnabled,
                        setRelayUrl: relay.setRelayUrl,
                        setMachineIdEnabled: relay.saveMachineIdEnabled,
                        setMachineId: relay.setMachineId,
                        handleRelayUrlBlur: relay.handleRelayUrlBlur,
                        handleMachineIdBlur: relay.handleMachineIdBlur,
                    }}
                />
            </ExtensionCtaLayout>
        );
    }

    return (
        <ExtensionCtaLayout colorScheme="standard">
            <ExtensionHeader
                icon={Smartphone}
                title={t('Smart Mic')}
                colorScheme="standard"
                titleTestId="smart-mic-title"
                subtitle={t('Dictate, translate and control your PC from your phone.')}
                badge={t('Beta')}
            />

            {isWayland && (
                <>
                    <SmartMicWaylandNotice />
                    <SmartMicCta onEnable={() => {}} disabled />
                </>
            )}

            {!isWayland && server.enabled === false && <SmartMicCta onEnable={() => server.saveEnabled(true)} />}
        </ExtensionCtaLayout>
    );
};
