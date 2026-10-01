import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

interface LlmModeNotConfiguredPayload {
    mode: number;
}

export const LlmModeNotConfiguredListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlisten = listen<LlmModeNotConfiguredPayload>('llm-mode-not-configured', (event) => {
            toast.info(
                t('Prompt {{mode}} is not configured. Open Prompt Mode to set it up.', {
                    mode: event.payload.mode,
                }),
                { autoClose: 5000 }
            );
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    return null;
};
