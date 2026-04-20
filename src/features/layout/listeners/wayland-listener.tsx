import { useEffect, useState } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { Button } from '@/components/button';

const unsubscribeAll = (promises: Promise<UnlistenFn>[]) => {
    promises.forEach((p) => {
        p.then((fn) => fn());
    });
};

/** Keep in sync with `packaging/linux/60-murmure-uinput.rules`. */
const UINPUT_FIX_COMMAND = `sudo tee /etc/udev/rules.d/60-murmure-uinput.rules > /dev/null <<'EOF'
KERNEL=="uinput", SUBSYSTEM=="misc", OPTIONS+="static_node=uinput", GROUP="input", MODE="0660", TAG+="uaccess"
EOF
sudo udevadm control --reload-rules
sudo udevadm trigger --property-match=DEVNAME=/dev/uinput`;

const InjectUnavailableBody = () => {
    const { t } = useTranslation();
    const [copied, setCopied] = useState(false);

    const handleCopy = async () => {
        try {
            await navigator.clipboard.writeText(UINPUT_FIX_COMMAND);
            setCopied(true);
            setTimeout(() => setCopied(false), 2000);
        } catch (e) {
            console.error('Failed to copy fix command:', e);
        }
    };

    return (
        <div className="flex flex-col gap-2">
            <p className="text-sm">
                {t(
                    'Wayland keystroke injection is unavailable: Murmure could not open /dev/uinput. Until you apply the fix, the transcription is copied to the clipboard — press Ctrl+V to paste.'
                )}
            </p>
            <p className="text-xs text-muted-foreground">
                {t('Run this in a terminal, then log out and back in:')}
            </p>
            <pre className="text-[10px] bg-black/40 rounded p-2 overflow-x-auto whitespace-pre-wrap break-all">
                {UINPUT_FIX_COMMAND}
            </pre>
            <Button
                type="button"
                variant="default"
                size="sm"
                onClick={handleCopy}
                disabled={copied}
                className="self-start"
            >
                {copied ? t('Copied!') : t('Copy command')}
            </Button>
        </div>
    );
};

/**
 * Surfaces Wayland-specific failure events from the backend as toast
 * warnings. Each event uses a stable toastId so repeated emissions do
 * not spam the user — react-toastify dedupes by id.
 */
export const WaylandListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlistens: Promise<UnlistenFn>[] = [
            listen('wayland-shortcuts-unavailable', () => {
                toast.warning(t('Global shortcuts are unavailable on this Wayland session.'), {
                    toastId: 'wayland-shortcuts-unavailable',
                    autoClose: false,
                });
            }),
            listen('wayland-inject-unavailable', () => {
                toast.warning(<InjectUnavailableBody />, {
                    toastId: 'wayland-inject-unavailable',
                    autoClose: false,
                });
            }),
        ];

        return () => unsubscribeAll(unlistens);
    }, [t]);

    return null;
};
