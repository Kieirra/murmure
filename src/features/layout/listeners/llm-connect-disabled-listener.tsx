import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const LlmConnectDisabledListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlisten = listen('llm-connect-disabled', () => {
            toast.info(t('Prompt Mode is off. Turn it on in Prompt Mode.'), { autoClose: 5000 });
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    return null;
};
