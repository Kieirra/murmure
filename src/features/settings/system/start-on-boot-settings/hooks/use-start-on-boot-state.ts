import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useStartOnBootState = () => {
    const [isAutostartEnabled, setIsAutostartEnabled] = useState(false);
    const { t } = useTranslation();

    const loadAutostartStatus = async () => {
        try {
            const enabled = await isEnabled();
            setIsAutostartEnabled(enabled);
        } catch (error) {
            console.error('Failed to load autostart status:', error);
        }
    };

    useEffect(() => {
        loadAutostartStatus();
    }, []);

    const setStartOnBoot = async (checked: boolean) => {
        try {
            if (checked) {
                await enable();
            } else {
                await disable();
            }
            setIsAutostartEnabled(checked);
        } catch (error) {
            toast.error(t('Failed to set "Start on boot"'));
            console.error(error);
        }
    };

    return {
        startOnBoot: isAutostartEnabled,
        setStartOnBoot: setStartOnBoot,
    };
};
