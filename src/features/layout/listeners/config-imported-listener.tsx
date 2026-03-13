import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

/**
 * Listens for the 'config-imported' event emitted by the Rust backend
 * when a configuration is imported via CLI while the app is running (hot-reload).
 * Forces a full page reload to ensure all views reflect the new configuration values.
 */
export const ConfigImportedListener = () => {
    useEffect(() => {
        const unlisten = listen('config-imported', () => {
            window.location.reload();
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    return null;
};
