import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from '@/i18n';

export const TrayMenuI18nListener = () => {
    const { t, i18n } = useTranslation();

    useEffect(() => {
        invoke('set_tray_menu_labels', {
            show: t('Open Murmure'),
            copyLastTranscript: t('Copy last transcript'),
            quit: t('Quit'),
        }).catch((error) => {
            console.error('Failed to update tray menu labels:', error);
        });
    }, [t, i18n.language]);

    return null;
};
