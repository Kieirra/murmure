import { listen } from '@tauri-apps/api/event';
import React, { useEffect, useState } from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { useVisualizerSensitivity } from '@/features/settings/system/visualizer-settings/hooks/use-visualizer-sensitivity';

interface LLMConnectSettings {
    modes: { name: string }[];
    active_mode_index: number;
}

export const Overlay: React.FC = () => {
    const [feedback, setFeedback] = useState<string | null>(null);
    const { sensitivity } = useVisualizerSensitivity();

    useEffect(() => {
        const unlistenPromise = listen<string>('overlay-feedback', (event) => {
            setFeedback(event.payload);
        });
        const unlistenSettingsPromise = listen<LLMConnectSettings>(
            'llm-settings-updated',
            (event) => {
                const activeMode =
                    event.payload.modes[event.payload.active_mode_index];
                if (activeMode?.name) {
                    setFeedback(activeMode.name);
                }
            }
        );

        return () => {
            unlistenPromise.then((unlisten) => unlisten());
            unlistenSettingsPromise.then((unlisten) => unlisten());
        };
    }, []);

    useEffect(() => {
        if (feedback) {
            const timer = setTimeout(() => setFeedback(null), 2000);
            return () => clearTimeout(timer);
        }
    }, [feedback]);

    return (
        <div className="w-[80px] h-[18px] bg-black rounded-sm flex items-center justify-center select-none overflow-hidden">
            {feedback ? (
                <span className="text-[10px] text-white font-medium truncate px-1 animate-in fade-in zoom-in duration-200">
                    {feedback}
                </span>
            ) : (
                <div className="origin-center">
                    <AudioVisualizer
                        className="bg-transparent"
                        bars={14}
                        rows={9}
                        audioPixelWidth={2}
                        audioPixelHeight={2}
                        sensitivity={sensitivity}
                    />
                </div>
            )}
        </div>
    );
};
