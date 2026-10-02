import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const CommandDisabledListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlisten = listen('command-disabled', () => {
            toast.info(t('Command Mode is off. Turn it on in Command Mode.'), { autoClose: 5000 });
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    return null;
};
