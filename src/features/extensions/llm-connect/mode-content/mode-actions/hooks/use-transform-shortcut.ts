import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export const useTransformShortcut = (modeIndex: number) => {
    const [shortcut, setShortcut] = useState('');

    useEffect(() => {
        invoke<string>('get_llm_transform_shortcut', { index: modeIndex })
            .then(setShortcut)
            .catch(() => setShortcut(''));
    }, [modeIndex]);

    return shortcut;
};
