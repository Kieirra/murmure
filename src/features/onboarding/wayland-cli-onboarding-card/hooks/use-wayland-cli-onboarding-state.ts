import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { AppSettings } from '@/features/settings/settings.types';
import { useHistoryState } from '@/features/home/history/hooks/use-history-state';

interface UseWaylandCliOnboardingStateResult {
    shouldDisplay: boolean;
    dismiss: () => Promise<void>;
}

export const useWaylandCliOnboardingState = (isWayland: boolean): UseWaylandCliOnboardingStateResult => {
    const [dismissed, setDismissed] = useState<boolean>(true);
    const { history } = useHistoryState();
    const hasAtLeastOneTranscript = history.length > 0;

    useEffect(() => {
        if (!isWayland) return;
        invoke<AppSettings>('get_all_settings')
            .then((settings) => setDismissed(settings.wayland_notice_dismissed))
            .catch((error) => {
                console.error('Failed to load Wayland notice dismissed flag:', error);
            });
    }, [isWayland]);

    useEffect(() => {
        if (!isWayland) return;
        if (dismissed) return;
        if (!hasAtLeastOneTranscript) return;
        setDismissed(true);
        invoke('dismiss_wayland_notice').catch((error) => {
            console.error('Failed to persist Wayland notice dismissed flag after first transcription:', error);
        });
    }, [isWayland, dismissed, hasAtLeastOneTranscript]);

    const dismiss = async () => {
        setDismissed(true);
        try {
            await invoke('dismiss_wayland_notice');
        } catch (error) {
            console.error('Failed to persist Wayland notice dismissed flag:', error);
            setDismissed(false);
        }
    };

    return {
        shouldDisplay: isWayland && !dismissed && !hasAtLeastOneTranscript,
        dismiss,
    };
};
