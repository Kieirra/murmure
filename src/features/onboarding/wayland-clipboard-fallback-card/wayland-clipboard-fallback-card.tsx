import { Clipboard, X } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useIsWayland } from '@/components/hooks/use-linux-session-type';
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
                    <Clipboard className="w-4 h-4 text-cyan-300" />
                </div>
                <div className="flex-1 min-w-0 pr-6">
                    <Typography.Paragraph className="text-foreground text-xs">
                        {t("Transcription didn't show up in your app? Press Ctrl+V, it's in your clipboard.")}
                    </Typography.Paragraph>
                </div>
            </div>
        </div>
    );
};
