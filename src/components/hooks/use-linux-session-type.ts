import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export type LinuxSessionType = 'wayland' | 'x11' | 'unknown';

/**
 * Returns the current Linux session type. `null` on non-Linux platforms
 * or while the value is still being fetched.
 */
export const useLinuxSessionType = () => {
    const [sessionType, setSessionType] = useState<LinuxSessionType | null>(null);

    useEffect(() => {
        let cancelled = false;
        invoke<string | null>('get_linux_session_type')
            .then((value) => {
                if (cancelled) return;
                if (value === 'wayland' || value === 'x11' || value === 'unknown') {
                    setSessionType(value);
                }
            })
            .catch((err) => {
                if (!cancelled) console.error('Failed to get Linux session type:', err);
            });
        return () => {
            cancelled = true;
        };
    }, []);

    return sessionType;
};

export const useIsWayland = () => useLinuxSessionType() === 'wayland';
