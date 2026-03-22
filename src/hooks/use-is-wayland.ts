import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export function useIsWayland(): boolean {
    const [isWayland, setIsWayland] = useState(false);

    useEffect(() => {
        invoke<boolean>('get_is_wayland')
            .then(setIsWayland)
            .catch(() => setIsWayland(false));
    }, []);

    return isWayland;
}
