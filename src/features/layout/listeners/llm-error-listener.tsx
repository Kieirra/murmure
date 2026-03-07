import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const LlmErrorListener = () => {
    const { t } = useTranslation();

    useEffect(() => {
        const unlisten = listen<string>('llm-error', (event) => {
            toast.error(
                t('LLM processing failed: {{error}}', { error: event.payload }),
                { autoClose: 5000 }
            );
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    return null;
};
