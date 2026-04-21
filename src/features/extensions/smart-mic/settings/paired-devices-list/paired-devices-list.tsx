import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Trash2 } from 'lucide-react';
import { useTranslation } from '@/i18n';
import type { PairedDevice } from '../../hooks/use-paired-devices';

interface PairedDevicesListProps {
    devices: PairedDevice[];
    remove: (token: string) => void;
}

export const PairedDevicesList = ({ devices, remove }: PairedDevicesListProps) => {
    const { t } = useTranslation();

    if (devices.length === 0) {
        return null;
    }

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{t('Paired Devices')}</Typography.Title>
                    <Typography.Paragraph>{t('Devices that have been paired with Smart Mic')}</Typography.Paragraph>
                    <div className="mt-2 space-y-2">
                        {devices.map((device) => (
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
                                    onClick={() => remove(device.token)}
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
            <SettingsUI.Separator />
        </>
    );
};
