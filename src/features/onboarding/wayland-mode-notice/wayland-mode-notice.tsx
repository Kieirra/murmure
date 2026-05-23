import { AlertTriangle, Clipboard, Terminal, X } from 'lucide-react';
import { Trans } from 'react-i18next';
import { InternalLink } from '@/components/internal-link';
import { RenderKeys } from '@/components/render-keys';
import { useTranslation } from '@/i18n';
import { useCopyToClipboardState } from '@/features/settings/system/copy-to-clipboard-settings/hooks/use-copy-to-clipboard-state';
import { useWaylandNoticeState } from './hooks/use-wayland-notice-state';
import { NoticeRow } from './notice-row/notice-row';

const SMALL_KBD = '[&_kbd]:h-[18px] [&_kbd]:min-w-[18px] [&_kbd]:px-1.5 [&_kbd]:py-0 [&_kbd]:text-[11px]';

export const WaylandModeNotice = () => {
    const { t } = useTranslation();
    const { copyToClipboard } = useCopyToClipboardState();
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
                {t('Wayland support is still experimental in Murmure.')}
            </NoticeRow>

            <hr className="border-yellow-300/20" />
            <NoticeRow icon={Terminal} title={t('Shortcuts are managed by your system')}>
                {t('Murmure is running in CLI mode. Shortcuts must be configured at the system level.')}
                <InternalLink to="/settings/shortcuts" className="block mt-1">
                    {t('See available commands')}
                </InternalLink>
            </NoticeRow>

            {copyToClipboard && (
                <>
                    <hr className="border-yellow-300/20" />
                    <NoticeRow icon={Clipboard} title={t('Paste manually if needed')}>
                        <Trans
                            i18nKey="If the transcription doesn't appear, press <kv/> (or <ksv/> in a terminal) to paste it from your clipboard."
                            components={{
                                kv: <RenderKeys keyString="Ctrl+V" className={SMALL_KBD} />,
                                ksv: <RenderKeys keyString="Ctrl+Shift+V" className={SMALL_KBD} />,
                            }}
                        />
                    </NoticeRow>
                </>
            )}
        </div>
    );
};
