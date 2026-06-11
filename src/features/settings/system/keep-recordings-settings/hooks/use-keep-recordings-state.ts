import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useKeepRecordingsState = () => {
    const [keepRecordings, setKeepRecordings] = useState<boolean>(false);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            setKeepRecordings(settings.keep_recordings);
        });
    }, []);

    const handleSetKeepRecordings = async (enabled: boolean) => {
        try {
            setKeepRecordings(enabled);
            await invoke('set_keep_recordings', { enabled });
        } catch {
            toast.error(t('Failed to save recordings setting'));
            setKeepRecordings(!enabled);
        }
    };

    return {
        keepRecordings,
        setKeepRecordings: handleSetKeepRecordings,
    };
};
