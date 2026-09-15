import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef, useState } from 'react';
import { clampResultPanelDurationSecs } from '../result-panel.helpers';

export interface FinalResultPayload {
    text: string;
    promptName?: string | null;
}

interface ActiveResult {
    payload: FinalResultPayload;
    durationSecs: number;
    showId: number;
}

const COPIED_HIDE_MS = 1000;

export const useResultPanel = (durationSecs: number, isConfigLoaded: boolean) => {
    const [active, setActive] = useState<ActiveResult | null>(null);
    const [isPaused, setIsPaused] = useState(false);
    const [isCopied, setIsCopied] = useState(false);
    const hideTimerRef = useRef<number | null>(null);
    const hideDeadlineRef = useRef(0);
    const remainingMsRef = useRef(0);
    const durationRef = useRef(durationSecs);
    const showIdRef = useRef(0);

    useEffect(() => {
        durationRef.current = durationSecs;
    }, [durationSecs]);

    const clearHideTimer = () => {
        if (hideTimerRef.current != null) {
            clearTimeout(hideTimerRef.current);
            hideTimerRef.current = null;
        }
    };

    const clearPanel = () => {
        clearHideTimer();
        setActive(null);
        setIsPaused(false);
        setIsCopied(false);
    };

    const dismiss = () => {
        clearPanel();
        invoke('hide_overlay_if_idle').catch(() => {});
    };

    const scheduleHide = (delayMs: number) => {
        clearHideTimer();
        hideDeadlineRef.current = Date.now() + delayMs;
        hideTimerRef.current = setTimeout(dismiss, delayMs);
    };

    useEffect(() => {
        if (!isConfigLoaded) return;

        const show = (payload: FinalResultPayload) => {
            const shownDurationSecs = clampResultPanelDurationSecs(durationRef.current);
            showIdRef.current += 1;
            setActive({ payload, durationSecs: shownDurationSecs, showId: showIdRef.current });
            setIsPaused(false);
            setIsCopied(false);
            scheduleHide(shownDurationSecs * 1000);
            invoke('ack_result_panel_shown').catch(() => {});
        };

        invoke<FinalResultPayload | null>('consume_pending_result')
            .then((pending) => {
                if (pending != null) show(pending);
            })
            .catch(() => {});

        const unlistenResult = listen<FinalResultPayload>('final-result', (event) => show(event.payload));
        const unlistenRecording = listen('recording-mode', clearPanel);

        return () => {
            clearHideTimer();
            unlistenResult.then((u) => u()).catch(() => {});
            unlistenRecording.then((u) => u()).catch(() => {});
        };
    }, [isConfigLoaded]);

    const pause = () => {
        if (active == null || isCopied) return;
        remainingMsRef.current = Math.max(0, hideDeadlineRef.current - Date.now());
        clearHideTimer();
        setIsPaused(true);
    };

    const resume = () => {
        if (active == null || isCopied || !isPaused) return;
        setIsPaused(false);
        scheduleHide(remainingMsRef.current);
    };

    const copy = () => {
        if (active == null || isCopied) return;
        invoke('copy_result_text', { text: active.payload.text }).catch(() => {});
        setIsCopied(true);
        scheduleHide(COPIED_HIDE_MS);
    };

    return {
        result: active?.payload ?? null,
        shownDurationSecs: active?.durationSecs ?? 0,
        showId: active?.showId ?? 0,
        isPaused,
        isCopied,
        pause,
        resume,
        copy,
        close: dismiss,
    };
};
