import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Power } from 'lucide-react';
import { Switch } from '@/components/switch';
import { useStartOnBootState } from './hooks/use-start-on-boot-state';
import { useTranslation } from '@/i18n';

export const StartOnBootSettings = () => {
    const { startOnBoot, setStartOnBoot } = useStartOnBootState();
    const { t } = useTranslation('settings');

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Power className="w-4 h-4 text-zinc-400" />
                    {t('system.startOnBoot.title')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('system.startOnBoot.description')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch checked={startOnBoot} onCheckedChange={setStartOnBoot} />
        </SettingsUI.Item>
    );
};
