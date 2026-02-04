import { listen } from '@tauri-apps/api/event';
import React, { useEffect, useState } from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';

interface LLMConnectSettings {
    modes: { name: string }[];
    active_mode_index: number;
}

type RecordingMode = 'standard' | 'llm' | 'command';

export const Overlay: React.FC = () => {
    const [feedback, setFeedback] = useState<string | null>(null);
    const [isError, setIsError] = useState(false);
    const [recordingMode, setRecordingMode] = useState<RecordingMode>('standard');

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
        const unlistenModePromise = listen<string>('overlay-mode', (event) => {
            const mode = event.payload as RecordingMode;
            if (mode === 'llm' || mode === 'command' || mode === 'standard') {
                setRecordingMode(mode);
            }
        });

        return () => {
            unlistenPromise.then((unlisten) => unlisten());
            unlistenSettingsPromise.then((unlisten) => unlisten());
            unlistenErrorPromise.then((unlisten) => unlisten());
            unlistenModePromise.then((unlisten) => unlisten());
        };
    }, []);

    useEffect(() => {
        if (feedback) {
            const timer = setTimeout(() => setFeedback(null), 2000);
            return () => clearTimeout(timer);
        }
    }, [feedback]);

    // Determine background color based on recording mode
    const getBackgroundClass = () => {
        switch (recordingMode) {
            case 'llm':
                return 'bg-emerald-900';
            case 'command':
                return 'bg-violet-900';
            default:
                return 'bg-black';
        }
    };

    return (
        <div className={`w-[80px] h-[18px] rounded-sm flex items-center justify-center select-none overflow-hidden transition-colors duration-300 ${getBackgroundClass()}`}>
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
