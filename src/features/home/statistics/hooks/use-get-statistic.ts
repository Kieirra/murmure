import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

interface Statistic {
    writing_speed_wpm: number;
    words_current_month: number;
    local_audio_mb: number;
    time_saved_seconds: number;
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
            wpm: 0,
            words: 0,
            localAudioMb: 0,
            timeSavedSeconds: 0,
        };
    }

    return {
        wpm: statistic.writing_speed_wpm,
        words: statistic.words_current_month,
        localAudioMb: statistic.local_audio_mb,
        timeSavedSeconds: statistic.time_saved_seconds,
    };
};
