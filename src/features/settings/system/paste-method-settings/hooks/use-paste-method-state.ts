import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export type PasteMethod = 'ctrl_v' | 'ctrl_shift_v' | 'direct';

export const usePasteMethodState = () => {
    const [pasteMethod, setPasteMethod] = useState<PasteMethod>('ctrl_v');
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            const method = settings.paste_method;
            if (method === 'ctrl_v' || method === 'ctrl_shift_v' || method === 'direct') {
                setPasteMethod(method);
            }
        });
    }, []);

    return {
        pasteMethod,
        setPasteMethod: (method: PasteMethod) => {
            setPasteMethod(method);
            invoke('set_paste_method', { method }).catch(() => {
                toast.error(t('Failed to save paste method'));
            });
        },
    };
};
