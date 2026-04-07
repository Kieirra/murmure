import { NumberInput } from '@/components/number-input';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useVirtualMicState } from './hooks/use-virtual-mic-state';
import { Trash2 } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const VirtualMicSettings = () => {
    const { virtualMicPort, setVirtualMicPort, qrCodeDataUri, pairedDevices, removePairedDevice } =
        useVirtualMicState();
    const { t } = useTranslation();

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{t('Virtual Mic Port')}</Typography.Title>
                    <Typography.Paragraph>
                        {t('Set the port number for the Virtual Mic HTTPS server (1024-65535)')}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <NumberInput
                    min={1024}
                    max={65535}
                    value={virtualMicPort}
                    onValueChange={(value) => setVirtualMicPort(value ?? 4801)}
                />
            </SettingsUI.Item>
            {qrCodeDataUri.length > 0 && (
                <>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>{t('QR Code')}</Typography.Title>
                            <Typography.Paragraph>
                                {t('Scan this QR code with your smartphone to connect')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <img
                            src={qrCodeDataUri}
                            alt="Virtual Mic QR Code"
                            className="w-[200px] h-[200px] rounded-lg border border-border"
                        />
                    </SettingsUI.Item>
                </>
            )}
            {pairedDevices.length > 0 && (
                <>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>{t('Paired Devices')}</Typography.Title>
                            <Typography.Paragraph>
                                {t('Devices that have been paired with Virtual Mic')}
                            </Typography.Paragraph>
                            <div className="mt-2 space-y-2">
                                {pairedDevices.map((device) => (
                                    <div
                                        key={device.token}
                                        className="flex items-center justify-between p-2 rounded-md bg-muted/50"
                                    >
                                        <div className="text-sm">
                                            <div className="font-medium">{device.name}</div>
                                            {device.last_connected.length > 0 && (
                                                <div className="text-xs text-muted-foreground">
                                                    {t('Last connected')}: {new Date(device.last_connected).toLocaleString()}
                                                </div>
                                            )}
                                        </div>
                                        <button
                                            onClick={() => removePairedDevice(device.token)}
                                            className="p-1.5 rounded-md hover:bg-destructive/20 text-muted-foreground hover:text-destructive transition-colors"
                                            title={t('Remove device')}
                                        >
                                            <Trash2 className="w-4 h-4" />
                                        </button>
                                    </div>
                                ))}
                            </div>
                        </SettingsUI.Description>
                    </SettingsUI.Item>
                </>
            )}
        </>
    );
};
