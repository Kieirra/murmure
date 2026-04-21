import { NumberInput } from '@/components/number-input';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';

interface TokenTtlSettingsProps {
    tokenTtlHours: number;
    setTokenTtlHours: (value: number) => void;
}

export const TokenTtlSettings = ({ tokenTtlHours, setTokenTtlHours }: TokenTtlSettingsProps) => {
    const { t } = useTranslation();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title>{t('Token expiration (hours)')}</Typography.Title>
                <Typography.Paragraph>{t('Set to 0 for no expiration (default)')}</Typography.Paragraph>
            </SettingsUI.Description>
            <NumberInput min={0} value={tokenTtlHours} onValueChange={(value) => setTokenTtlHours(value ?? 0)} />
        </SettingsUI.Item>
    );
};
