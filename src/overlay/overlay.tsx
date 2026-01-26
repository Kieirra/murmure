import { listen } from '@tauri-apps/api/event';
import React, { useEffect, useState } from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';

interface LLMConnectSettings {
    modes: { name: string }[];
    active_mode_index: number;
}

export const Overlay: React.FC = () => {
    const [feedback, setFeedback] = useState<string | null>(null);
    const [isError, setIsError] = useState(false);

    useEffect(() => {
        const unlistenPromise = listen<string>('overlay-feedback', (event) => {
            setFeedback(event.payload);
            setIsError(false);
        });
        const unlistenSettingsPromise = listen<LLMConnectSettings>(
            'llm-settings-updated',
            (event) => {
                const activeMode =
                    event.payload.modes[event.payload.active_mode_index];
                if (activeMode?.name) {
                    setFeedback(activeMode.name);
                    setIsError(false);
                }
            }
        );
        const unlistenErrorPromise = listen<string>('llm-error', (event) => {
            setFeedback(event.payload);
            setIsError(true);
        });

        return () => {
            unlistenPromise.then((unlisten) => unlisten());
            unlistenSettingsPromise.then((unlisten) => unlisten());
            unlistenErrorPromise.then((unlisten) => unlisten());
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
                <span className={`text-[10px] font-medium truncate px-1 animate-in fade-in zoom-in duration-200 ${
                    isError ? 'text-red-500' : 'text-white'
                }`}>
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
                    />
                </div>
            )}
        </div>
    );
};
