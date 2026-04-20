import { invoke } from '@tauri-apps/api/core';
import { useState, useCallback, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useTokenTtl = () => {
    const [tokenTtlHours, setTokenTtlHours] = useState<number>(0);
    const { t } = useTranslation();

    const load = useCallback(async () => {
        try {
            const ttl = await invoke<number | null>('get_smartmic_token_ttl_hours');
            setTokenTtlHours(ttl ?? 0);
        } catch (error) {
            console.error('Failed to load token TTL:', error);
        }
    }, []);

    useEffect(() => {
        load();
    }, [load]);

    const saveTokenTtlHours = useCallback(
        async (value: number = 0) => {
            setTokenTtlHours(value);
            try {
                await invoke('set_smartmic_token_ttl_hours', { hours: value > 0 ? value : null });
            } catch (error) {
                console.error('Failed to save token TTL:', error);
                toast.error(t('Failed to save token expiration'));
            }
        },
        [t]
    );

    return {
        tokenTtlHours,
        saveTokenTtlHours,
        load,
    };
};
