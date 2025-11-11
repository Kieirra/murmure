import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { useHistoryPersistenceState } from './hooks/use-history-persistence-state';
import { Shield } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const HistorySettings = () => {
    const { persistHistory, setPersistHistory } = useHistoryPersistenceState();
    const { t } = useTranslation('settings');

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Shield className="w-4 h-4 text-zinc-400" />
                    {t('system.historyPersistence.title')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('system.historyPersistence.description')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch
                checked={persistHistory}
                onCheckedChange={setPersistHistory}
            />
        </SettingsUI.Item>
    );
};
