import { AlertTriangle } from 'lucide-react';
import { Link } from '@tanstack/react-router';
import { Kbd } from '@/components/kbd';
import { useTranslation } from '@/i18n';
import { useWaylandPortalState } from '@/features/settings/system/wayland-portal-settings/hooks/use-wayland-portal-state';

export const WaylandExperimentalNotice = () => {
    const { t } = useTranslation();
    const { useWaylandPortal } = useWaylandPortalState();

    const title = useWaylandPortal ? t('Wayland is experimental') : t('XWayland mode: limited shortcuts');

    return (
        <div className="rounded-lg border border-yellow-300/20 bg-yellow-300/10 p-4 space-y-4">
            <div className="flex items-center gap-2">
                <AlertTriangle className="w-4 h-4 flex-shrink-0 text-yellow-300" />
                <span className="text-yellow-300 text-sm font-semibold">{title}</span>
            </div>
            {useWaylandPortal ? (
                <p className="text-xs text-yellow-300 pl-6">
                    {t('Global shortcuts work, but they can be inconsistent on GNOME.')}
                </p>
            ) : (
                <p className="text-xs text-yellow-300 pl-6">
                    {t(
                        'Shortcuts only work when Murmure is the active window. To record while Murmure stays in the background, use the'
                    )}{' '}
                    <Link
                        to="/extensions/voice-mode"
                        className="font-semibold text-yellow-300 underline underline-offset-2 hover:text-yellow-200"
                    >
                        {t('Voice Mode')}
                    </Link>
                    {t(' (Extensions > Voice Mode in the sidebar).')}
                </p>
            )}
            <p className="text-xs text-muted-foreground pl-6">
                <span className="font-semibold">{t('Tip:')}</span> {t('your transcription is auto-copied. Press')}{' '}
                <Kbd>Ctrl</Kbd>+<Kbd>V</Kbd> {t('to paste it.')}
            </p>
        </div>
    );
};
