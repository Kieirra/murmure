import { Clipboard, X } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { useWaylandClipboardFallbackState } from './hooks/use-wayland-clipboard-fallback-state';

export const WaylandClipboardFallbackCard = () => {
    const { t } = useTranslation();
    const isWayland = useIsWayland();
    const { shouldDisplay, dismiss } = useWaylandClipboardFallbackState(isWayland);

    if (!shouldDisplay) {
        return null;
    }

    return (
        <div className="w-full flex items-center gap-3 rounded-r-md border border-l-2 border-input border-l-sky-500 bg-surface-header py-2 pl-4 pr-2">
            <Clipboard className="w-4 h-4 shrink-0 text-sky-400" />
            <div className="flex-1 min-w-0">
                <Typography.Paragraph className="text-foreground text-xs">
                    {t("Transcription didn't show up in your app? Press Ctrl+V, it's in your clipboard.")}
                </Typography.Paragraph>
            </div>
            <Button
                type="button"
                variant="subtle"
                size="icon-sm"
                onClick={dismiss}
                aria-label={t('Close')}
                className="size-7"
            >
                <X />
            </Button>
        </div>
    );
};
