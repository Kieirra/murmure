import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export const useTransformProcessing = () => {
    const [isTransformProcessing, setIsTransformProcessing] = useState(false);

    useEffect(() => {
        invoke<boolean>('is_transform_processing')
            .then(setIsTransformProcessing)
            .catch(() => {});

        const unlistenStart = listen('transform-processing-start', () => setIsTransformProcessing(true));
        const unlistenEnd = listen('transform-processing-end', () => setIsTransformProcessing(false));

        return () => {
            unlistenStart.then((u) => u()).catch(() => {});
            unlistenEnd.then((u) => u()).catch(() => {});
        };
    }, []);

    return { isTransformProcessing };
};
