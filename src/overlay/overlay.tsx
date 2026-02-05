import { listen } from '@tauri-apps/api/event';
import React, { useEffect, useState } from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import clsx from 'clsx';

interface LLMConnectSettings {
    modes: { name: string }[];
    active_mode_index: number;
}

type RecordingMode = 'standard' | 'llm' | 'command';

export const Overlay: React.FC = () => {
    const [feedback, setFeedback] = useState<string | null>(null);
    const [isError, setIsError] = useState(false);
    const [recordingMode, setRecordingMode] =
        useState<RecordingMode>('standard');

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

    const getModeLabel = (mode: RecordingMode): string => {
        switch (mode) {
            case 'llm':
                return 'Llm-co.';
            case 'command':
                return 'Command';
            case 'standard':
            default:
                return 'Transcription';
        }
    };

    return (
        <div
            className={clsx(
                'w-[80px]',
                'h-[30px]',
                'bg-transparent',
                'relative',
                'select-none',
                'overflow-hidden'
            )}
        >
            <div className="text-white text-[8px] text-center absolute -top-0.5 left-1/2 -translate-x-1/2">
                {getModeLabel(recordingMode)}
            </div>
            {feedback ? (
                <span
                    className={clsx(
                        'text-[10px]',
                        'font-medium',
                        'truncate',
                        'animate-in',
                        'fade-in',
                        'zoom-in',
                        'duration-200',
                        isError && 'text-red-500',
                        !isError && 'text-white'
                    )}
                >
                    {feedback}
                </span>
            ) : (
                <div
                    className={clsx(
                        'origin-center',
                        'h-[20px]',
                        'mt-1',
                        'p-1.5',
                        'rounded-sm',
                        'overflow-hidden',
                        recordingMode === 'llm' && 'bg-sky-950',
                        recordingMode === 'command' && 'bg-red-950',
                        recordingMode === 'standard' && 'bg-black'
                    )}
                >
                    <AudioVisualizer
                        className="-mt-3"
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
