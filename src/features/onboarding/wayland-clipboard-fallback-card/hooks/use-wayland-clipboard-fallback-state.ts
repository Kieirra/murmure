import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { AppSettings } from '@/features/settings/settings.types';
import { useHistoryState } from '@/features/home/history/hooks/use-history-state';

interface UseWaylandClipboardFallbackStateResult {
    shouldDisplay: boolean;
    dismiss: () => Promise<void>;
}

export const useWaylandClipboardFallbackState = (isWayland: boolean): UseWaylandClipboardFallbackStateResult => {
    const [dismissed, setDismissed] = useState<boolean>(true);
    const { history } = useHistoryState();
    const hasAtLeastOneTranscript = history.length > 0;

    useEffect(() => {
        if (!isWayland) return;
        invoke<AppSettings>('get_all_settings')
            .then((settings) => setDismissed(settings.wayland_clipboard_fallback_dismissed))
            .catch((error) => {
                console.error('Failed to load Wayland clipboard fallback dismissed flag:', error);
            });
    }, [isWayland]);

    const dismiss = async () => {
        setDismissed(true);
        try {
            await invoke('dismiss_wayland_clipboard_fallback');
        } catch (error) {
            console.error('Failed to persist Wayland clipboard fallback dismissed flag:', error);
            setDismissed(false);
        }
    };

    return {
        shouldDisplay: isWayland && hasAtLeastOneTranscript && !dismissed,
        dismiss,
    };
};
