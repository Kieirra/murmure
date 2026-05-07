import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { useWaylandPortalState } from '@/features/settings/system/wayland-portal-settings/hooks/use-wayland-portal-state';
import { ShortcutsCli } from './shortcuts-cli/shortcuts-cli';
import { ShortcutsRegular } from './shortcuts-regular/shortcuts-regular';

export const Shortcuts = () => {
    const isWayland = useIsWayland();
    const { useWaylandPortal } = useWaylandPortalState();
    const isCliMode = isWayland && !useWaylandPortal;

    return isCliMode ? <ShortcutsCli /> : <ShortcutsRegular />;
};
