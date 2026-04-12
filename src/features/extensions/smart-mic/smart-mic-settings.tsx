import { NumberInput } from '@/components/number-input';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useSmartMicState } from './hooks/use-smart-mic-state';
import { Smartphone, Trash2, RefreshCw } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const SmartMicSettings = () => {
    const { smartMicPort, setSmartMicPort, qrCodeDataUri, pairedDevices, removePairedDevice, resetTokens } =
        useSmartMicState();
    const { t } = useTranslation();

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{t('Smart Mic Port')}</Typography.Title>
                    <Typography.Paragraph>
                        {t('Set the port number for the Smart Mic HTTPS server (1024-65535)')}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <NumberInput
                    min={1024}
                    max={65535}
                    value={smartMicPort}
                    onValueChange={(value) => setSmartMicPort(value ?? 4801)}
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
                            <div className="mt-2 flex items-start gap-2 rounded-lg bg-linear-to-r from-cyan-900/30 to-emerald-900/30 border border-cyan-500/20 p-2.5 text-sm">
                                <Smartphone className="w-4 h-4 mt-0.5 shrink-0 text-cyan-400" />
                                <div>
                                    <span className="text-xs font-medium text-cyan-400">{t('Tip')}</span>
                                    <p className="mt-0.5 text-muted-foreground">
                                        {t(
                                            'After scanning, use your browser\'s "Add to Home Screen" option. Smart Mic will then be available as an app — no need to scan again.'
                                        )}
                                    </p>
                                </div>
                            </div>
                        </SettingsUI.Description>
                        <div className="flex flex-col items-center gap-2">
                            <img
                                src={qrCodeDataUri}
                                alt="Smart Mic QR Code"
                                className="w-[200px] h-[200px] rounded-lg border border-border"
                            />
                            <button
                                onClick={resetTokens}
                                className="flex items-center gap-1.5 px-3 py-1.5 text-xs rounded-md border border-border text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
                                title={t('Reset QR code and revoke all paired devices')}
                            >
                                <RefreshCw className="w-3 h-3" />
                                {t('Reset QR Code')}
                            </button>
                        </div>
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
                                {t('Devices that have been paired with Smart Mic')}
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
                                                    {t('Last connected')}:{' '}
                                                    {new Date(device.last_connected).toLocaleString()}
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
