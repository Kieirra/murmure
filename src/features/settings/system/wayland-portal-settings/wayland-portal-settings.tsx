import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Monitor } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { useTranslation } from '@/i18n';
import { useWaylandPortalState } from './hooks/use-wayland-portal-state';

const OPTIONS = [
    { key: 'wayland', label: 'Wayland (native)' },
    { key: 'xwayland', label: 'XWayland' },
];

export const WaylandPortalSettings = () => {
    const isWayland = useIsWayland();
    const { useWaylandPortal, setUseWaylandPortal } = useWaylandPortalState();
    const { t } = useTranslation();

    if (!isWayland) {
        return null;
    }

    const value = useWaylandPortal ? 'wayland' : 'xwayland';
    const handleChange = (v: string) => setUseWaylandPortal(v === 'wayland');

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Monitor className="w-4 h-4 text-muted-foreground" />
                    {t('Wayland integration')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('KDE works best with native. GNOME may prefer XWayland.')}
                    <br />
                    <span className="text-xs text-muted-foreground">{t('Restart required.')}</span>
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={value} onValueChange={handleChange}>
                <SelectTrigger className="w-[200px]" data-testid="wayland-integration-select">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {OPTIONS.map((opt) => (
                        <SelectItem key={opt.key} value={opt.key}>
                            {t(opt.label)}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
