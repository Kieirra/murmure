import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Layout } from 'lucide-react';
import { Switch } from '@/components/switch';
import { useDockSettingsState } from './hooks/use-dock-settings-state';
import { useTranslation } from '@/i18n';

export const DockSettings = () => {
    const { showInDock, setDockVisibility } = useDockSettingsState();
    const { t } = useTranslation();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Layout className="w-4 h-4 text-muted-foreground" />
                    {t('Show in Dock')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Show the application icon in the macOS Dock. (Restart required)')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch checked={showInDock} onCheckedChange={setDockVisibility} />
        </SettingsUI.Item>
    );
};
