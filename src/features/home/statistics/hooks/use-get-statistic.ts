import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

interface Statistic {
    writing_speed_wpm: number;
    words_current_month: number;
    local_audio_mb: number;
}

export const useGetStatistic = () => {
    const [statistic, setStatistic] = useState<Statistic | null>(null);

    useEffect(() => {
        const fetchStatistic = async () => {
            const stats = await invoke<Statistic>('get_usage_stats');
            setStatistic(stats);
        };

        fetchStatistic();

        const unlisten = listen('stats_updated', () => {
            fetchStatistic();
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    if (statistic == null) {
        return {
            wpm: '-',
            words: '-',
            data: '-',
        };
    }

    return {
        wpm:
            statistic?.writing_speed_wpm > 80
                ? statistic?.writing_speed_wpm.toFixed(1)
                : '-',
        words:
            statistic?.words_current_month > 0
                ? statistic?.words_current_month.toFixed(1)
                : '-',
        data:
            statistic?.local_audio_mb > 0
                ? statistic?.local_audio_mb.toFixed(1)
                : '-',
    };
};
