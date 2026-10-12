import { Info } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { WaylandCliSetupDialog } from './wayland-cli-setup-dialog/wayland-cli-setup-dialog';
import { useWaylandCliOnboardingState } from './hooks/use-wayland-cli-onboarding-state';

export const WaylandCliOnboardingCard = () => {
    const { t } = useTranslation();
    const isWayland = useIsWayland();
    const { isDialogOpen, isReminderVisible, openDialog, closeDialogForSession } =
        useWaylandCliOnboardingState(isWayland);

    return (
        <>
            {isReminderVisible && (
                <div
                    className="w-full flex items-center gap-3 rounded-r-md border border-l-2 border-input border-l-sky-500 bg-surface-header py-1.5 pl-3 pr-1.5"
                    data-testid="wayland-cli-reminder"
                >
                    <Info className="size-4 shrink-0 text-sky-400" />
                    <Typography.Paragraph className="flex-1 min-w-0 text-foreground text-xs">
                        {t('Shortcut set up? This message goes away after your first transcription.')}
                    </Typography.Paragraph>
                    <Page.SecondaryButton size="sm" onClick={openDialog} data-testid="wayland-cli-reminder-open">
                        {t('See the steps')}
                    </Page.SecondaryButton>
                </div>
            )}
            <WaylandCliSetupDialog open={isDialogOpen} onClose={closeDialogForSession} />
        </>
    );
};
