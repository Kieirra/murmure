import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Keyboard, TriangleAlert } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { useTranslation } from '@/i18n';
import { useWaylandPortalState } from './hooks/use-wayland-portal-state';

const OPTIONS = [
    { key: 'portal', label: 'XDG Portal' },
    { key: 'cli', label: 'CLI' },
];

export const WaylandPortalSettings = () => {
    const isWayland = useIsWayland();
    const { useWaylandPortal, setUseWaylandPortal } = useWaylandPortalState();
    const { t } = useTranslation();

    if (!isWayland) {
        return null;
    }

    const value = useWaylandPortal ? 'portal' : 'cli';
    const handleChange = (v: string) => setUseWaylandPortal(v === 'portal');

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Keyboard className="w-4 h-4 text-muted-foreground" />
                    {t('Shortcut handling')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('XDG Portal works on KDE, Hyprland, Sway. Use CLI on GNOME or if your shortcuts misbehave.')}
                </Typography.Paragraph>
                <Typography.Paragraph className="space-x-2 mt-2 inline">
                    <TriangleAlert className="w-4 h-4 shrink-0 text-yellow-400 inline-block" />
                    <span className="text-xs">{t('Restart required.')}</span>
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select value={value} onValueChange={handleChange}>
                <SelectTrigger className="w-[200px]" data-testid="shortcut-handling-select">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {OPTIONS.map((opt) => (
                        <SelectItem key={opt.key} value={opt.key}>
                            {opt.label}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
