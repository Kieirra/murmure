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
        const hideOverlay = () => {
            invoke('hide_overlay_if_idle').catch(() => {});
        };

        const showFlash = (newText: string) => {
            if (fadeTimerRef.current != null) clearTimeout(fadeTimerRef.current);
            if (hideTimerRef.current != null) clearTimeout(hideTimerRef.current);

            setText(newText);
            setIsFadingOut(false);

            fadeTimerRef.current = setTimeout(() => setIsFadingOut(true), FLASH_HOLD_MS);
            hideTimerRef.current = setTimeout(hideOverlay, FLASH_HOLD_MS + FADE_OUT_MS);
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
            if (fadeTimerRef.current != null) clearTimeout(fadeTimerRef.current);
            if (hideTimerRef.current != null) clearTimeout(hideTimerRef.current);
            unlistenPromise.then((u) => u()).catch(() => {});
        };
    }, []);

    return { text, isFadingOut };
};
