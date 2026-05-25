import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export type LinuxSessionType = 'wayland' | 'x11' | 'unknown';

export type DesktopEnvironment = 'gnome' | 'kde' | 'cinnamon' | 'xfce' | 'mate' | 'hyprland' | 'sway' | 'i3' | 'other';

export interface LinuxDistroInfo {
    os_name: string | null;
    desktop_env: DesktopEnvironment;
}

const DESKTOP_ENVIRONMENT_VALUES: readonly DesktopEnvironment[] = [
    'gnome',
    'kde',
    'cinnamon',
    'xfce',
    'mate',
    'hyprland',
    'sway',
    'i3',
    'other',
];

const isDesktopEnvironment = (value: unknown): value is DesktopEnvironment =>
    typeof value === 'string' && (DESKTOP_ENVIRONMENT_VALUES as readonly string[]).includes(value);

/**
 * Returns the current Linux session type. `null` on non-Linux platforms
 * or while the value is still being fetched.
 */
export const useLinuxSessionType = () => {
    const [sessionType, setSessionType] = useState<LinuxSessionType | null>(null);

    useEffect(() => {
        invoke<string | null>('get_linux_session_type')
            .then((value) => {
                if (value === 'wayland' || value === 'x11' || value === 'unknown') {
                    setSessionType(value);
                }
            })
            .catch((err) => console.error('Failed to get Linux session type:', err));
    }, []);

    return sessionType;
};

/**
 * Returns the Linux distro info (OS name + desktop environment). `null` on
 * non-Linux platforms or while the value is still being fetched.
 */
export const useLinuxDistroInfo = () => {
    const [info, setInfo] = useState<LinuxDistroInfo | null>(null);

    useEffect(() => {
        invoke<LinuxDistroInfo | null>('get_linux_distro_info')
            .then((value) => {
                if (value === null) {
                    return;
                }
                const desktop_env = isDesktopEnvironment(value.desktop_env) ? value.desktop_env : 'other';
                const os_name = typeof value.os_name === 'string' && value.os_name.length > 0 ? value.os_name : null;
                setInfo({ os_name, desktop_env });
            })
            .catch((err) => console.error('Failed to get Linux distro info:', err));
    }, []);

    return info;
};

export const useIsWayland = () => useLinuxSessionType() === 'wayland';
