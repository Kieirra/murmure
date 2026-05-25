import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { ShortcutsCli } from './shortcuts-cli/shortcuts-cli';
import { ShortcutsRegular } from './shortcuts-regular/shortcuts-regular';

export const Shortcuts = () => {
    const isWayland = useIsWayland();

    return isWayland ? <ShortcutsCli /> : <ShortcutsRegular />;
};
