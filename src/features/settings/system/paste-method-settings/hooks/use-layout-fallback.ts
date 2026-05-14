import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface LayoutFallbackPayload {
    layout: string;
    variant: string | null;
    reason: string;
}

interface LayoutFallbackState {
    isFallback: boolean;
}

// Sticky once set: the initial invoke rehydrates a fallback that fired
// before Settings mounted, the listen picks up later recompiles.
export const useLayoutFallback = (): LayoutFallbackState => {
    const [state, setState] = useState<LayoutFallbackState>({ isFallback: false });

    useEffect(() => {
        invoke<LayoutFallbackPayload | null>('get_layout_fallback_state')
            .then((payload) => {
                if (payload != null) {
                    setState({ isFallback: true });
                }
            })
            .catch(() => {});
    }, []);

    useEffect(() => {
        const unlisten = listen<LayoutFallbackPayload>('wayland-layout-fallback', () => {
            setState({ isFallback: true });
        });
        return () => {
            unlisten.then((u) => u()).catch(() => {});
        };
    }, []);

    return state;
};
