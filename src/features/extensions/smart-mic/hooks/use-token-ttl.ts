import { invoke } from '@tauri-apps/api/core';
import { useState, useCallback, useEffect } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useTokenTtl = () => {
    const [tokenTtlHours, setTokenTtlHoursState] = useState<number>(0);
    const { t } = useTranslation();

    const load = useCallback(async () => {
        try {
            const ttl = await invoke<number | null>('get_smartmic_token_ttl_hours');
            setTokenTtlHoursState(ttl ?? 0);
        } catch (error) {
            console.error('Failed to load token TTL:', error);
        }
    }, []);

    useEffect(() => {
        load();
    }, [load]);

    const setTokenTtlHours = useCallback(
        async (value: number = 0) => {
            setTokenTtlHoursState(value);
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
        setTokenTtlHours,
        load,
    };
};
