import { Input } from '@/components/input';
import { NumberInput } from '@/components/number-input';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useSmartMicState } from './hooks/use-smart-mic-state';
import { Switch } from '@/components/switch';
import { ExternalLink } from '@/components/external-link';
import { Smartphone, Trash2, RefreshCw, Settings2, ChevronDown, ChevronUp, AlertTriangle, FileCode2 } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const SmartMicSettings = () => {
    const {
        smartMicPort,
        setSmartMicPort,
        qrCodeDataUri,
        pairedDevices,
        removePairedDevice,
        resetTokens,
        relayUrl,
        setRelayUrl,
        machineId,
        setMachineId,
        machineIdEnabled,
        setMachineIdEnabled,
        machineHostname,
        tokenTtlHours,
        isAdvancedOpen,
        toggleAdvanced,
        handleRelayUrlBlur,
        handleMachineIdBlur,
        handleTokenTtlChange,
    } = useSmartMicState();
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
                                    <span className="text-xs font-medium text-cyan-400">
                                        {t('Tip')}
                                    </span>
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
            <SettingsUI.Separator />
            <div className="p-4">
                <button
                    type="button"
                    onClick={toggleAdvanced}
                    className="flex items-center gap-2 text-sm font-medium text-foreground hover:text-foreground transition-colors w-fit cursor-pointer"
                >
                    <Settings2 className="w-4 h-4" />
                    {t('Advanced Settings')}
                    {isAdvancedOpen ? (
                        <ChevronUp className="w-4 h-4 text-muted-foreground" />
                    ) : (
                        <ChevronDown className="w-4 h-4 text-muted-foreground" />
                    )}
                </button>
                {isAdvancedOpen && (
                    <div className="mt-4 flex flex-col">
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
                                placeholder="https://smartmic.hospital.com"
                                className="w-72"
                            />
                        </SettingsUI.Item>
                        {relayUrl.length > 0 && (
                            <div className="flex items-center gap-1.5 text-xs text-amber-500/80 px-4 pb-3">
                                <AlertTriangle className="w-3 h-3 shrink-0" />
                                {t('When using an external relay, audio data transits through the relay server. For sensitive data, use a self-hosted relay.')}
                            </div>
                        )}
                        <SettingsUI.Separator />
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title>{t('Machine ID')}</Typography.Title>
                                <Typography.Paragraph>
                                    {t('Include a machine identifier in the relay URL (for multi-computer setups)')}
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
                                        value={machineId.length > 0 ? machineId : machineHostname}
                                        onChange={(e) => setMachineId(e.target.value)}
                                        onBlur={handleMachineIdBlur}
                                    />
                                </SettingsUI.Item>
                            </>
                        )}
                        <SettingsUI.Separator />
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title>{t('Token expiration (hours)')}</Typography.Title>
                                <Typography.Paragraph>
                                    {t('Set to 0 for no expiration (default)')}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <NumberInput
                                min={0}
                                value={tokenTtlHours}
                                onValueChange={handleTokenTtlChange}
                            />
                        </SettingsUI.Item>
                        <div className="text-xs flex items-center gap-1 mt-4">
                            <FileCode2 className="w-4 h-4 text-muted-foreground inline-block" />
                            {t('View')}{' '}
                            <ExternalLink href="https://docs.murmure.app/features/smart-speech-mic/#remote-access">
                                {t('remote access documentation')}
                            </ExternalLink>
                        </div>
                    </div>
                )}
            </div>
        </>
    );
};
