import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { RefreshCw } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const XWaylandRefreshButton = () => {
    const { t } = useTranslation();
    const [visible, setVisible] = useState(false);

    useEffect(() => {
        invoke<boolean>('is_xwayland_fallback')
            .then(setVisible)
            .catch((err) => console.error('Failed to detect XWayland fallback:', err));
    }, []);

    if (!visible) return null;

    return (
        <button
            type="button"
            onClick={() => invoke('refresh_main_window')}
            className="text-muted-foreground text-xs hover:text-foreground transition-colors flex items-center gap-2 px-2 w-full text-left"
            data-testid="xwayland-refresh-button"
        >
            <RefreshCw className="w-4 h-4" />
            <span>{t('Refresh window')}</span>
        </button>
    );
};
