import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useOverlayState = () => {
    const [overlayMode, setOverlayMode] = useState<'hidden' | 'recording' | 'always'>('recording');
    const [overlayPosition, setOverlayPosition] = useState<'top' | 'bottom'>('bottom');
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            const m = settings.overlay_mode;
            if (m === 'hidden' || m === 'recording' || m === 'always') setOverlayMode(m);
            const p = settings.overlay_position;
            if (p === 'top' || p === 'bottom') setOverlayPosition(p);
        });
    }, []);

    return {
        setOverlayMode: (m: 'hidden' | 'recording' | 'always') => {
            setOverlayMode(m);
            invoke('set_overlay_mode', { mode: m }).catch(() => {
                toast.error(t('Failed to save overlay settings'));
            });
        },
        setOverlayPosition: (p: 'top' | 'bottom') => {
            setOverlayPosition(p);
            invoke('set_overlay_position', { position: p }).catch(() => {
                toast.error(t('Failed to save overlay settings'));
            });
        },
        overlayMode,
        overlayPosition,
    };
};
