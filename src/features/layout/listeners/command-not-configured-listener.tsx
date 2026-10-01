import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const CommandNotConfiguredListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlisten = listen('command-not-configured', () => {
            toast.info(t('The command has no model. Choose one in Command Mode.'), { autoClose: 5000 });
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    return null;
};
