import { AlertTriangle, Info } from 'lucide-react';
import { Link } from '@tanstack/react-router';
import { Kbd } from '@/components/kbd';
import { useTranslation } from '@/i18n';
import { useWaylandPortalState } from '@/features/settings/system/wayland-portal-settings/hooks/use-wayland-portal-state';
import { useCopyToClipboardState } from '@/features/settings/system/copy-to-clipboard-settings/hooks/use-copy-to-clipboard-state';

export const WaylandExperimentalNotice = () => {
    const { t } = useTranslation();
    const { useWaylandPortal } = useWaylandPortalState();
    const { copyToClipboard } = useCopyToClipboardState();

    const title = useWaylandPortal ? t('Wayland is experimental') : t('XWayland mode: limited shortcuts');

    return (
        <div className="w-full bg-yellow-300/10 border border-yellow-300/20 rounded-lg p-4 space-y-4">
            <div className="flex items-start gap-3">
                <div className="w-8 h-8 bg-yellow-300/20 rounded-full flex items-center justify-center flex-shrink-0">
                    <AlertTriangle className="w-4 h-4 text-yellow-300" />
                </div>
                <div className="space-y-1">
                    <p className="text-yellow-300 font-semibold text-sm">{title}</p>
                    {useWaylandPortal ? (
                        <p className="text-foreground text-xs">
                            {t('Global shortcuts work, but they can be inconsistent on GNOME.')}
                        </p>
                    ) : (
                        <p className="text-foreground text-xs">
                            {t(
                                'Shortcuts only work when Murmure is the active window. To record while Murmure stays in the background, use the'
                            )}{' '}
                            <Link
                                to="/extensions/voice-mode"
                                className="font-semibold text-yellow-300 underline underline-offset-2 hover:text-yellow-200"
                            >
                                {t('Voice Mode')}
                            </Link>
                            {'.'}
                        </p>
                    )}
                </div>
            </div>

            {copyToClipboard && (
                <div className="flex items-start gap-3">
                    <div className="w-8 h-8 bg-yellow-300/20 rounded-full flex items-center justify-center flex-shrink-0">
                        <Info className="w-4 h-4 text-yellow-300" />
                    </div>
                    <div className="space-y-1">
                        <p className="text-yellow-300 font-semibold text-sm">{t('Tip')}</p>
                        <p className="text-foreground text-xs">
                            {t('Your transcription is auto-copied. Press')} <Kbd>Ctrl</Kbd>+<Kbd>V</Kbd>{' '}
                            {t('to paste it.')}
                        </p>
                    </div>
                </div>
            )}
        </div>
    );
};
