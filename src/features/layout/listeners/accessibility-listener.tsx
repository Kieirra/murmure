import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

/**
 * Listens for macOS accessibility permission events and shows a toast notification
 * when the permission is missing.
 */
export const AccessibilityListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlisten = listen('accessibility-permission-missing', () => {
            toast.warning(
                <div className="flex flex-col gap-2">
                    <span>
                        {t('Accessibility permission required for shortcuts')}
                    </span>
                    <button
                        className="text-sm underline text-left hover:text-blue-400"
                        onClick={() => invoke('open_accessibility_settings')}
                    >
                        {t('Open Settings')}
                    </button>
                </div>,
                { autoClose: false }
            );
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    return null;
};
