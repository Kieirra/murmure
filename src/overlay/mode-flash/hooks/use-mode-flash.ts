import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef, useState } from 'react';
import { isModeFlashPayload, ModeFlashPayload, ModeFlashState } from '../mode-flash.helpers';

const FLASH_HOLD_MS = 1200;

export const useModeFlash = () => {
    const [flashState, setFlashState] = useState<ModeFlashState | null>(null);
    const [bootstrapped, setBootstrapped] = useState(false);
    const fadeTimerRef = useRef<number | null>(null);

    useEffect(() => {
        // Le flash reste monté en fadingOut jusqu'au prochain flash ou destruction
        // de l'overlay : remettre à null démasquerait le visualizer dessous.
        const apply = (payload: ModeFlashPayload) => {
            if (fadeTimerRef.current != null) window.clearTimeout(fadeTimerRef.current);
            setFlashState({ text: payload.text, fadingOut: false });
            fadeTimerRef.current = window.setTimeout(() => {
                setFlashState((prev) => (prev != null ? { ...prev, fadingOut: true } : null));
            }, FLASH_HOLD_MS);
        };

        invoke<ModeFlashPayload | null>('consume_pending_mode_flash')
            .then((payload) => {
                if (payload != null && isModeFlashPayload(payload)) apply(payload);
            })
            .catch(() => {})
            .finally(() => setBootstrapped(true));

        const unlistenPromise = listen('mode-flash', (event) => {
            if (isModeFlashPayload(event.payload)) apply(event.payload);
        });

        return () => {
            if (fadeTimerRef.current != null) window.clearTimeout(fadeTimerRef.current);
            unlistenPromise.then((unlisten) => unlisten()).catch(() => {});
        };
    }, []);

    return { flashState, bootstrapped };
};
