import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export function useIsWayland(): boolean {
    const [isWayland, setIsWayland] = useState(false);

    useEffect(() => {
        invoke<boolean>('is_wayland_session')
            .then(setIsWayland)
            .catch(() => setIsWayland(false));
    }, []);

    return isWayland;
}
