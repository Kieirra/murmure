import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef, useState } from 'react';

const FLASH_HOLD_MS = 1200;
const FADE_OUT_MS = 200;

export const useModeFlash = () => {
    const [text, setText] = useState<string | null>(null);
    const [isFadingOut, setIsFadingOut] = useState(false);
    const fadeTimerRef = useRef<number | null>(null);
    const hideTimerRef = useRef<number | null>(null);

    useEffect(() => {
        const showFlash = (newText: string) => {
            if (fadeTimerRef.current != null) window.clearTimeout(fadeTimerRef.current);
            if (hideTimerRef.current != null) window.clearTimeout(hideTimerRef.current);

            setText(newText);
            setIsFadingOut(false);

            fadeTimerRef.current = window.setTimeout(() => setIsFadingOut(true), FLASH_HOLD_MS);
            hideTimerRef.current = window.setTimeout(() => {
                invoke('hide_overlay_if_idle').catch(() => {});
            }, FLASH_HOLD_MS + FADE_OUT_MS);
        };

        invoke<string | null>('consume_pending_mode_flash')
            .then((pending) => {
                if (pending != null) showFlash(pending);
            })
            .catch(() => {});

        const unlistenPromise = listen<string>('mode-flash', (event) => {
            showFlash(event.payload);
        });

        return () => {
            if (fadeTimerRef.current != null) window.clearTimeout(fadeTimerRef.current);
            if (hideTimerRef.current != null) window.clearTimeout(hideTimerRef.current);
            unlistenPromise.then((u) => u()).catch(() => {});
        };
    }, []);

    return { text, isFadingOut };
};
