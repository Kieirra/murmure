import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useRemoveHesitations = () => {
    const [removeHesitations, setRemoveHesitations] = useState<boolean>(false);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            setRemoveHesitations(settings.remove_hesitations);
        });
    }, []);

    const handleSetRemoveHesitations = async (enabled: boolean) => {
        try {
            setRemoveHesitations(enabled);
            await invoke('set_remove_hesitations', { enabled });
        } catch {
            toast.error(t('Failed to save hesitation setting'));
            setRemoveHesitations(!enabled);
        }
    };

    return {
        removeHesitations,
        setRemoveHesitations: handleSetRemoveHesitations,
    };
};
