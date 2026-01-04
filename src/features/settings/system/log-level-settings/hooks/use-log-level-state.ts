import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const useLogLevelState = () => {
    const [logLevel, setLogLevel] = useState<string>('info');
    const { t } = useTranslation();

    useEffect(() => {
        const loadLogLevel = async () => {
            try {
                const savedLevel = await invoke<string>('get_log_level');
                if (savedLevel) {
                    setLogLevel(savedLevel);
                }
            } catch (error) {
                console.error('Failed to load log level:', error);
                // Silent fail or toast? default is info anyway.
            }
        };
        loadLogLevel();
    }, []);

    const saveLogLevel = async (level: string) => {
        try {
            await invoke('set_log_level', { level });
            setLogLevel(level);
            toast.success(t('Log level updated'));
        } catch (error) {
            console.error('Failed to save log level:', error);
            toast.error(t('Failed to save log level'));
        }
    };

    return {
        logLevel,
        setLogLevel: saveLogLevel,
    };
};
