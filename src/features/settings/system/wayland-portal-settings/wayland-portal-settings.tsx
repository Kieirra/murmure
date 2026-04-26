import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { KeyRound } from 'lucide-react';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { useTranslation } from '@/i18n';
import { useWaylandPortalState } from './hooks/use-wayland-portal-state';

export const WaylandPortalSettings = () => {
    const isWayland = useIsWayland();
    const { useWaylandPortal, setUseWaylandPortal } = useWaylandPortalState();
    const { t } = useTranslation();

    if (!isWayland) {
        return null;
    }

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <KeyRound className="w-4 h-4 text-muted-foreground" />
                    {t('Use Wayland portal for global shortcuts')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Recommended for KDE Plasma. May be unstable on GNOME.')}
                    <br />
                    {t('Disable to use XWayland fallback (limited to focused window).')}
                    <br />
                    <span className="text-xs text-muted-foreground">
                        {t('Note: requires restart to take effect.')}
                    </span>
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch checked={useWaylandPortal} onCheckedChange={setUseWaylandPortal} />
        </SettingsUI.Item>
    );
};
