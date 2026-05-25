import { Info, X } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useIsWayland, useLinuxDistroInfo } from '@/components/hooks/use-linux-session-type';
import { Typography } from '@/components/typography';
import { InternalLink } from '@/components/internal-link';
import { CliCommandRow } from '@/features/settings/shortcuts/shortcuts-cli/cli-commands-panel/cli-command-row/cli-command-row';
import { useWaylandCliOnboardingState } from './hooks/use-wayland-cli-onboarding-state';
import {
    DEFAULT_SHORTCUT_PATH,
    DESKTOP_ENV_SHORTCUT_PATH,
    WAYLAND_CLI_SETUP_COMMAND,
} from './wayland-cli-onboarding-card.helpers';

export const WaylandCliOnboardingCard = () => {
    const { t } = useTranslation();
    const isWayland = useIsWayland();
    const distroInfo = useLinuxDistroInfo();
    const { shouldDisplay, dismiss } = useWaylandCliOnboardingState(isWayland);

    if (!shouldDisplay) {
        return null;
    }

    const osName = distroInfo?.os_name ?? null;
    const shortcutPath =
        (distroInfo ? DESKTOP_ENV_SHORTCUT_PATH[distroInfo.desktop_env] : null) ?? DEFAULT_SHORTCUT_PATH;
    const instruction =
        osName != null
            ? t('Create a keyboard shortcut on {{osName}} that runs:', { osName })
            : t('Create a keyboard shortcut on your system that runs:');

    return (
        <div className="relative w-full bg-cyan-300/10 border border-cyan-300/20 rounded-lg p-4">
            <button
                type="button"
                onClick={dismiss}
                aria-label={t('Close')}
                className="absolute top-2 right-2 text-muted-foreground hover:text-foreground px-2 p-0.5"
            >
                <X className="w-4 h-4 cursor-pointer" />
            </button>

            <div className="flex items-start gap-3">
                <div className="w-8 h-8 bg-cyan-300/20 rounded-full flex items-center justify-center flex-shrink-0">
                    <Info className="w-4 h-4 text-cyan-300" />
                </div>
                <div className="space-y-2 flex-1 min-w-0 pr-6">
                    <Typography.Title className="text-cyan-300 font-semibold text-sm">
                        {t('Set up Murmure on Linux')}
                    </Typography.Title>
                    <Typography.Paragraph className="text-foreground text-xs">{instruction}</Typography.Paragraph>
                    <div className="border border-border rounded-md bg-background/40">
                        <CliCommandRow label="" command={WAYLAND_CLI_SETUP_COMMAND} />
                    </div>
                    <Typography.Paragraph className="text-muted-foreground text-xs">
                        {t('In {{path}}', { path: t(shortcutPath) })}
                    </Typography.Paragraph>
                    <InternalLink to="/settings/shortcuts" className="text-xs">
                        {t('See other commands →')}
                    </InternalLink>
                </div>
            </div>
        </div>
    );
};
