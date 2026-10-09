import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const ToastEventListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlistens = [
            listen('command-not-configured', () => {
                toast.info(t('The command has no model. Choose one in Command Mode.'), { autoClose: 5000 });
            }),
            listen('command-disabled', () => {
                toast.info(t('Command Mode is off. Turn it on in Command Mode.'), { autoClose: 5000 });
            }),
            listen('llm-connect-disabled', () => {
                toast.info(t('Prompt Mode is off. Turn it on in Prompt Mode.'), { autoClose: 5000 });
            }),
            listen('transform-selection-empty', () => {
                toast.info(t('Select text before using Transform.'), { autoClose: 5000 });
            }),
        ];

        return () => {
            unlistens.forEach((unlisten) => unlisten.then((fn) => fn()));
        };
    }, [t]);

    return null;
};
