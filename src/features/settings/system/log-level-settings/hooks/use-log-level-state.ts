import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useLogLevelState = () => {
    const [logLevel, setLogLevelState] = useState<string>('info');
    const { t } = useTranslation();

    useEffect(() => {
        const loadLogLevel = async () => {
            try {
                const savedLevel = await invoke<string>('get_log_level');
                if (savedLevel) {
                    setLogLevelState(savedLevel);
                }
            } catch (error) {
                console.error('Failed to load log level:', error);
                // Silent fail or toast? default is info anyway.
            }
        };
        loadLogLevel();
    }, []);

    const setLogLevel = async (level: string) => {
        try {
            await invoke('set_log_level', { level });
            setLogLevelState(level);
            toast.success(t('Log level updated'));
        } catch (error) {
            console.error('Failed to save log level:', error);
            toast.error(t('Failed to save log level'));
        }
    };

    return {
        logLevel,
        setLogLevel,
    };
};
