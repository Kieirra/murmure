import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export function useVisualizerSensitivity() {
    const [sensitivity, setSensitivity] = useState(10);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        invoke<number>('get_visualizer_sensitivity')
            .then((value) => {
                setSensitivity(value);
                setIsLoading(false);
            })
            .catch(() => {
                setIsLoading(false);
            });
    }, []);

    const updateSensitivity = async (value: number) => {
        setSensitivity(value);
        await invoke('set_visualizer_sensitivity', { sensitivity: value });
    };

    return { sensitivity, setSensitivity: updateSensitivity, isLoading };
}
