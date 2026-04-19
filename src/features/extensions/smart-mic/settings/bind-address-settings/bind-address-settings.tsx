import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useTranslation } from '@/i18n';
import type { NetworkInterface } from '../../hooks/use-bind-address';

const BIND_ADDRESS_AUTO = 'auto';

interface BindAddressSettingsProps {
    bindAddress: string | null;
    availableInterfaces: NetworkInterface[];
    setBindAddress: (value: string | null) => void;
}

export const BindAddressSettings = ({ bindAddress, availableInterfaces, setBindAddress }: BindAddressSettingsProps) => {
    const { t } = useTranslation();
    const selectedValue = bindAddress == null ? BIND_ADDRESS_AUTO : bindAddress;

    const handleChange = (value: string) => {
        if (value === BIND_ADDRESS_AUTO) {
            setBindAddress(null);
            return;
        }
        setBindAddress(value);
    };

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title>{t('Bind address')}</Typography.Title>
                <Typography.Paragraph>
                    {t(
                        'Choose which network interface the smart mic server should listen on. Use Auto unless you need to force a specific VPN or LAN interface.'
                    )}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={selectedValue} onValueChange={handleChange}>
                <SelectTrigger className="w-72">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent className="max-h-96">
                    <SelectItem value={BIND_ADDRESS_AUTO}>{t('Auto (recommended)')}</SelectItem>
                    {availableInterfaces.map((iface) => (
                        <SelectItem key={iface.ip} value={iface.ip}>
                            {iface.ip} ({iface.name})
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
