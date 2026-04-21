import { Input } from '@/components/input';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { ExternalLink } from '@/components/external-link';
import { AlertTriangle, FileCode2 } from 'lucide-react';
import { useTranslation } from '@/i18n';

interface RelaySettingsProps {
    relayEnabled: boolean;
    relayUrl: string;
    machineIdEnabled: boolean;
    machineId: string;
    setRelayEnabled: (value: boolean) => void;
    setRelayUrl: (value: string) => void;
    setMachineIdEnabled: (value: boolean) => void;
    setMachineId: (value: string) => void;
    handleRelayUrlBlur: () => void;
    handleMachineIdBlur: () => void;
}

export const RelaySettings = ({
    relayEnabled,
    relayUrl,
    machineIdEnabled,
    machineId,
    setRelayEnabled,
    setRelayUrl,
    setMachineIdEnabled,
    setMachineId,
    handleRelayUrlBlur,
    handleMachineIdBlur,
}: RelaySettingsProps) => {
    const { t } = useTranslation();

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{t('Enable relay')}</Typography.Title>
                    <Typography.Paragraph>
                        {t('Expose Smart Mic through a reverse proxy for external access')}
                    </Typography.Paragraph>
                    <div className="flex items-center gap-1 text-xs">
                        <FileCode2 className="w-4 h-4 text-muted-foreground" />
                        {t('View')}{' '}
                        <ExternalLink href="https://docs.murmure.app/features/smart-speech-mic/#remote-access">
                            {t('remote access documentation')}
                        </ExternalLink>
                    </div>
                    {relayEnabled && (
                        <div className="flex items-center gap-1.5 text-xs text-yellow-300/90">
                            <AlertTriangle className="w-3 h-3 shrink-0" />
                            {t(
                                'When using an external relay, audio data transits through the relay server. For sensitive data, use a self-hosted relay.'
                            )}
                        </div>
                    )}
                </SettingsUI.Description>
                <Switch checked={relayEnabled} onCheckedChange={setRelayEnabled} />
            </SettingsUI.Item>
            {relayEnabled && (
                <>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>{t('Relay URL')}</Typography.Title>
                            <Typography.Paragraph>
                                {t('Reverse proxy URL for accessing Smart Mic from an external network')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <Input
                            value={relayUrl}
                            onChange={(e) => setRelayUrl(e.target.value)}
                            onBlur={handleRelayUrlBlur}
                            placeholder="https://myrelay.com"
                            className="w-72"
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>{t('Use machine name')}</Typography.Title>
                            <Typography.Paragraph>
                                {t(
                                    'Include a machine identifier in the relay URL (for multi-computer setups behind a reverse proxy that strips this segment)'
                                )}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <Switch checked={machineIdEnabled} onCheckedChange={setMachineIdEnabled} />
                    </SettingsUI.Item>
                    {machineIdEnabled && (
                        <>
                            <SettingsUI.Separator />
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>{t('Machine name')}</Typography.Title>
                                </SettingsUI.Description>
                                <Input
                                    value={machineId}
                                    onChange={(e) => setMachineId(e.target.value)}
                                    onBlur={handleMachineIdBlur}
                                    className="w-72"
                                />
                            </SettingsUI.Item>
                        </>
                    )}
                </>
            )}
        </>
    );
};
