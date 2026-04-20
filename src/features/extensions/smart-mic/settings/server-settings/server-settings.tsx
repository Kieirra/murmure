import { NumberInput } from '@/components/number-input';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';

interface ServerSettingsProps {
    port: number;
    setPort: (value: number) => void;
}

export const ServerSettings = ({ port, setPort }: ServerSettingsProps) => {
    const { t } = useTranslation();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title>{t('Smart Mic Port')}</Typography.Title>
                <Typography.Paragraph>
                    {t('Set the port number for the Smart Mic HTTPS server (1024-65535)')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <NumberInput min={1024} max={65535} value={port} onValueChange={(value) => setPort(value ?? 4801)} />
        </SettingsUI.Item>
    );
};
