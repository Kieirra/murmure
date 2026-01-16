import { listen } from '@tauri-apps/api/event';
import React, { useEffect, useState } from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';

export const Overlay: React.FC = () => {
    const [feedback, setFeedback] = useState<string | null>(null);

    useEffect(() => {
        const unlistenPromise = listen<string>('overlay-feedback', (event) => {
            setFeedback(event.payload);
        });

        return () => {
            unlistenPromise.then((unlisten) => unlisten());
        };
    }, []);

    useEffect(() => {
        if (feedback) {
            const timer = setTimeout(() => setFeedback(null), 2000);
            return () => clearTimeout(timer);
        }
    }, [feedback]);

    return (
        <div className="w-[80px] h-[18px] bg-black/70 rounded-sm flex items-center justify-center select-none overflow-hidden">
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
                    />
                </div>
            )}
        </div>
    );
};
