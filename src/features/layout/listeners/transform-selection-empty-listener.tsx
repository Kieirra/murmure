import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const TransformSelectionEmptyListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlisten = listen('transform-selection-empty', () => {
            toast.info(t('Select text before using Transform.'), { autoClose: 5000 });
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    return null;
};
