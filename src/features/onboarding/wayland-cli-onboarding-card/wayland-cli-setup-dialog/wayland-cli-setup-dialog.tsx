import { ReactNode } from 'react';
import { Terminal } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useLinuxDistroInfo } from '@/components/hooks/use-linux-session-type';
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/dialog';
import { Page } from '@/components/page';
import { Typography } from '@/components/typography';
import { InternalLink } from '@/components/internal-link';
import { CommandPill } from '../command-pill/command-pill';
import {
    DEFAULT_SHORTCUT_PATH,
    DESKTOP_ENV_SHORTCUT_PATH,
    WAYLAND_CLI_SETUP_COMMAND,
} from '../wayland-cli-onboarding-card.helpers';

interface SetupStepProps {
    number: number;
    children: ReactNode;
}

const SetupStep = ({ number, children }: SetupStepProps) => (
    <li className="flex items-start gap-3">
        <span className="size-6 bg-sky-500/15 rounded-full flex items-center justify-center flex-shrink-0 text-xs font-semibold text-sky-400">
            {number}
        </span>
        <div className="space-y-2 flex-1 min-w-0 pt-0.5">{children}</div>
    </li>
);

interface WaylandCliSetupDialogProps {
    open: boolean;
    onClose: () => void;
}

export const WaylandCliSetupDialog = ({ open, onClose }: WaylandCliSetupDialogProps) => {
    const { t } = useTranslation();
    const distroInfo = useLinuxDistroInfo();

    const osName = distroInfo?.os_name ?? null;
    const shortcutPath =
        (distroInfo ? DESKTOP_ENV_SHORTCUT_PATH[distroInfo.desktop_env] : null) ?? DEFAULT_SHORTCUT_PATH;
    const instruction =
        osName != null
            ? t('Create a keyboard shortcut on {{osName}} that runs it', { osName })
            : t('Create a keyboard shortcut on your system that runs it');

    const handleOpenChange = (isOpen: boolean) => {
        if (!isOpen) {
            onClose();
        }
    };

    return (
        <Dialog open={open} onOpenChange={handleOpenChange}>
            <DialogContent
                className="max-w-xl"
                overlayClassName="backdrop-blur-sm"
                data-testid="wayland-cli-setup-dialog"
            >
                <DialogHeader className="space-y-3">
                    <div className="flex items-center gap-3">
                        <div className="w-8 h-8 bg-sky-500/15 rounded-full flex items-center justify-center flex-shrink-0">
                            <Terminal className="w-4 h-4 text-sky-400" />
                        </div>
                        <DialogTitle>{t('Set up Murmure on Linux')}</DialogTitle>
                    </div>
                    <DialogDescription>
                        {t('On Wayland, Murmure starts from a system shortcut. It takes one minute.')}
                    </DialogDescription>
                </DialogHeader>

                <ol className="space-y-4">
                    <SetupStep number={1}>
                        <Typography.Paragraph className="text-foreground">
                            {t('Copy this command', { context: 'step' })}
                        </Typography.Paragraph>
                        <CommandPill command={WAYLAND_CLI_SETUP_COMMAND} />
                    </SetupStep>
                    <SetupStep number={2}>
                        <Typography.Paragraph className="text-foreground">{instruction}</Typography.Paragraph>
                        <Typography.Paragraph className="text-xs">
                            {t('In {{path}}', { path: t(shortcutPath) })}
                        </Typography.Paragraph>
                    </SetupStep>
                    <SetupStep number={3}>
                        <Typography.Paragraph className="text-foreground">
                            {t('Test it: press your shortcut and speak.')}
                        </Typography.Paragraph>
                    </SetupStep>
                </ol>

                <DialogFooter className="items-center sm:justify-between">
                    <InternalLink to="/settings/shortcuts" onClick={onClose} className="text-sm">
                        {t('See other commands →')}
                    </InternalLink>
                    <DialogClose asChild>
                        <Page.PrimaryButton data-testid="wayland-cli-setup-dialog-confirm">
                            {t('Got it')}
                        </Page.PrimaryButton>
                    </DialogClose>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    );
};
