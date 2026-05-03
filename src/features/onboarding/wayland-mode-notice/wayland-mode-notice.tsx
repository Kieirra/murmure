import { AlertTriangle, Info, X } from 'lucide-react';
import { Link } from '@tanstack/react-router';
import { Kbd } from '@/components/kbd';
import { useTranslation } from '@/i18n';
import { useWaylandPortalState } from '@/features/settings/system/wayland-portal-settings/hooks/use-wayland-portal-state';
import { useCopyToClipboardState } from '@/features/settings/system/copy-to-clipboard-settings/hooks/use-copy-to-clipboard-state';
import { useWaylandNoticeState } from './hooks/use-wayland-notice-state';
import { NoticeRow } from './notice-row/notice-row';

export const WaylandModeNotice = () => {
    const { t } = useTranslation();
    const { useWaylandPortal } = useWaylandPortalState();
    const { copyToClipboard } = useCopyToClipboardState();
    const { dismissed, dismiss } = useWaylandNoticeState();

    if (dismissed) {
        return null;
    }

    const title = useWaylandPortal ? t('Wayland is experimental') : t('XWayland mode');

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
            <NoticeRow icon={AlertTriangle} title={title}>
                {useWaylandPortal ? (
                    t('Global shortcuts work, but they can be inconsistent on GNOME.')
                ) : (
                    <>
                        {t(
                            'Global shortcuts have known limitations under XWayland. Shortcuts only work when Murmure is the active window. To record while Murmure stays in the background, use the'
                        )}{' '}
                        <Link
                            to="/extensions/voice-mode"
                            className="font-semibold text-yellow-300 underline underline-offset-2 hover:text-yellow-200"
                        >
                            {t('Voice Mode')}
                        </Link>
                        {'.'}
                    </>
                )}
            </NoticeRow>

            {copyToClipboard && (
                <NoticeRow icon={Info} title={t('Tip')}>
                    {t('Your transcription is auto-copied. Press')} <Kbd>Ctrl</Kbd>+<Kbd>V</Kbd> {t('to paste it.')}
                </NoticeRow>
            )}
        </div>
    );
};
