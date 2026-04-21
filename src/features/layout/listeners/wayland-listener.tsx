import { useEffect } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

const unsubscribeAll = (promises: Promise<UnlistenFn>[]) => {
    promises.forEach((p) => {
        p.then((fn) => fn());
    });
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
            listen('wayland-clipboard-selection-unavailable', () => {
                toast.warning(
                    t(
                        'Command mode could not read the selected text on Wayland. Using the raw transcription instead.'
                    ),
                    {
                        toastId: 'wayland-clipboard-selection-unavailable',
                    }
                );
            }),
            listen('wayland-clipboard-direct-unavailable', () => {
                toast.warning(
                    t(
                        'Direct paste mode may not reach native Wayland apps. Consider switching to Standard (Ctrl+V) paste.'
                    ),
                    {
                        toastId: 'wayland-clipboard-direct-unavailable',
                    }
                );
            }),
        ];

        return () => unsubscribeAll(unlistens);
    }, [t]);

    return null;
};
