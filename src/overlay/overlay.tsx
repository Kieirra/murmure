import React from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';

export const Overlay: React.FC = () => {
    return (
        <div className="w-[80px] h-[18px] bg-black/70 rounded-sm flex items-center justify-center select-none overflow-hidden">
            <div className="origin-center">
                <AudioVisualizer
                    className="bg-transparent"
                    bars={14}
                    rows={9}
                    audioPixelWidth={2}
                    audioPixelHeight={2}
                />
            </div>
        </div>
    );
};
