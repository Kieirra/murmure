import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const RecordingErrorListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlisten = listen<string>('recording-error', () => {
            toast.error(
                t(
                    'Microphone unavailable. Please check your device connection.'
                )
            );
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    return null;
};
