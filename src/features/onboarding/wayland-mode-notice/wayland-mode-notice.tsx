import { AlertTriangle, ArrowRight, Terminal, X } from 'lucide-react';
import { Link } from '@tanstack/react-router';
import { useTranslation } from '@/i18n';
import { useWaylandPortalState } from '@/features/settings/system/wayland-portal-settings/hooks/use-wayland-portal-state';
import { useWaylandNoticeState } from './hooks/use-wayland-notice-state';
import { NoticeRow } from './notice-row/notice-row';

export const WaylandModeNotice = () => {
    const { t } = useTranslation();
    const { useWaylandPortal } = useWaylandPortalState();
    const { dismissed, dismiss } = useWaylandNoticeState();

    if (dismissed) {
        return null;
    }

    return (
        <div className="w-full bg-yellow-300/10 border border-yellow-300/20 rounded-lg p-4 space-y-4 relative">
            <button
                type="button"
                onClick={dismiss}
                aria-label={t('Close')}
                className="absolute top-2 right-2 text-muted-foreground hover:text-foreground p-1"
            >
                <X className="w-4 h-4 cursor-pointer" />
            </button>
            <NoticeRow icon={AlertTriangle} title={t('Wayland is experimental')}>
                {t('Wayland support is still experimental in Murmure. If you hit any issue, please open a GitHub issue.')}
            </NoticeRow>

            {!useWaylandPortal && (
                <>
                    <hr className="border-yellow-300/20" />
                    <NoticeRow icon={Terminal} title={t('Shortcuts are managed by your system')}>
                        {t(
                            'Murmure is running in CLI mode. Shortcuts must be configured at the system level (GNOME Settings, KDE shortcuts, ...).'
                        )}{' '}
                        <Link
                            to="/settings/shortcuts"
                            className="inline-flex items-center gap-1 font-semibold text-yellow-300 underline underline-offset-2 hover:text-yellow-200"
                        >
                            {t('See available commands')}
                            <ArrowRight className="w-3 h-3" />
                        </Link>
                    </NoticeRow>
                </>
            )}
        </div>
    );
};
