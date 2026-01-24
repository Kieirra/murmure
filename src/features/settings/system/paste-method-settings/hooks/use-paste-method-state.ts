import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export type PasteMethod = 'ctrl_v' | 'ctrl_shift_v' | 'direct';

export const usePasteMethodState = () => {
    const [pasteMethod, setPasteMethod] = useState<PasteMethod>('ctrl_v');

    useEffect(() => {
        invoke<PasteMethod>('get_paste_method').then((method) => {
            if (['ctrl_v', 'ctrl_shift_v', 'direct'].includes(method)) {
                setPasteMethod(method);
            }
        });
    }, []);

    return {
        pasteMethod,
        setPasteMethod: (method: PasteMethod) => {
            setPasteMethod(method);
            invoke('set_paste_method', { method });
        },
    };
};
