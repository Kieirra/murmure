import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { useHistoryPersistenceState } from './hooks/use-history-persistence-state';
import { Shield } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const HistorySettings = () => {
    const { persistHistory, setPersistHistory } = useHistoryPersistenceState();
    const { t } = useTranslation();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Shield className="w-4 h-4 text-zinc-400" />
                    {t('History persistence')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t(
                        'Store the last five transcriptions on disk. Disable to keep history in memory only.'
                    )}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch
                checked={persistHistory}
                onCheckedChange={setPersistHistory}
            />
        </SettingsUI.Item>
    );
};
