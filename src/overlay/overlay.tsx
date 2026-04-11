import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef, useState } from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { useLevelState } from '@/features/home/audio-visualizer/hooks/use-level-state';
import { useStreamingState } from './streaming-text/use-streaming-state';
import { StreamingText } from './streaming-text/streaming-text';
import type { LLMConnectSettings } from '@/features/extensions/llm-connect/hooks/use-llm-connect';
import clsx from 'clsx';

type RecordingMode = 'standard' | 'llm' | 'command';

export const Overlay = () => {
    const [feedback, setFeedback] = useState<string | null>(null);
    const [isError, setIsError] = useState(false);
    const [recordingMode, setRecordingMode] = useState<RecordingMode>('standard');
    const { level } = useLevelState();
    const [hasAudio, setHasAudio] = useState(false);
    const audioTimerRef = useRef<number | null>(null);
    const [repaintKey, setRepaintKey] = useState(0);
    const { text, highlights, hasStreamingText, reset: resetStreaming } = useStreamingState();

    useEffect(() => {
        if (hasAudio) return;
        if (level > 0.01) {
            if (!audioTimerRef.current) {
                audioTimerRef.current = setTimeout(() => {
                    setHasAudio(true);
                    audioTimerRef.current = null;
                }, 50);
            }
        } else if (audioTimerRef.current) {
            clearTimeout(audioTimerRef.current);
            audioTimerRef.current = null;
        }
    }, [level, hasAudio]);

    useEffect(() => {
        const unlistenPromise = listen<string>('overlay-feedback', (event) => {
            setFeedback(event.payload);
            setIsError(false);
        });
        const unlistenSettingsPromise = listen<LLMConnectSettings>('llm-settings-updated', (event) => {
            const activeMode = event.payload.modes[event.payload.active_mode_index];
            if (activeMode?.name) {
                setFeedback(activeMode.name);
                setIsError(false);
            }
        });
        const unlistenErrorPromise = listen<string>('llm-error', (event) => {
            setFeedback(event.payload);
            setIsError(true);
        });
        const unlistenRecordingErrorPromise = listen<string>('recording-error', () => {
            setFeedback('Mic error');
            setIsError(true);
        });
        const unlistenModePromise = listen<string>('overlay-mode', (event) => {
            const mode = event.payload as RecordingMode;
            if (mode === 'llm' || mode === 'command' || mode === 'standard') {
                setRecordingMode(mode);
            }
        });
        const unlistenShowPromise = listen('show-overlay', () => {
            setHasAudio(false);
            resetStreaming();
            setRepaintKey((k) => k + 1);
            if (audioTimerRef.current) {
                clearTimeout(audioTimerRef.current);
                audioTimerRef.current = null;
            }
        });

        return () => {
            unlistenPromise.then((unlisten) => unlisten());
            unlistenSettingsPromise.then((unlisten) => unlisten());
            unlistenErrorPromise.then((unlisten) => unlisten());
            unlistenRecordingErrorPromise.then((unlisten) => unlisten());
            unlistenModePromise.then((unlisten) => unlisten());
            unlistenShowPromise.then((unlisten) => unlisten());
        };
    }, [resetStreaming]);

    useEffect(() => {
        if (feedback) {
            const timer = setTimeout(() => setFeedback(null), 2000);
            return () => clearTimeout(timer);
        }
    }, [feedback]);

    const getModeLabel = (mode: RecordingMode): string => {
        switch (mode) {
            case 'llm':
                return 'LLM';
            case 'command':
                return 'Command';
            case 'standard':
            default:
                return 'Transcription';
        }
    };

    const bgClass = clsx(
        recordingMode === 'llm' && 'bg-sky-950',
        recordingMode === 'command' && 'bg-red-950',
        recordingMode === 'standard' && 'bg-black',
    );

    return (
        <div
            key={repaintKey}
            className="w-full min-h-[36px] relative select-none flex flex-col items-center"
        >
            {feedback ? (
                <span
                    className={clsx(
                        'text-[8px]',
                        'font-medium',
                        'truncate',
                        'flex',
                        'items-center',
                        'justify-center',
                        'h-[36px]',
                        'px-2',
                        'rounded-lg',
                        'animate-in',
                        'fade-in',
                        'zoom-in',
                        'duration-200',
                        bgClass,
                        isError && 'text-red-500',
                        !isError && 'text-white'
                    )}
                >
                    {feedback}
                </span>
            ) : (
                <div className="flex flex-col items-center w-full">
                    <div className={clsx(
                        'h-[40px]', 'py-1', 'px-3', 'overflow-hidden',
                        'flex', 'items-center', 'justify-center',
                        'rounded-lg', 'w-1/2',
                        bgClass,
                    )}>
                        {hasAudio ? (
                            <AudioVisualizer
                                bars={30}
                                rows={9}
                                audioPixelWidth={3}
                                audioPixelHeight={3}
                            />
                        ) : (
                            <span className="text-white text-[8px] flex items-center justify-center h-full">
                                {getModeLabel(recordingMode)}
                            </span>
                        )}
                    </div>
                    {hasStreamingText && (
                        <div className={clsx('w-full', 'rounded-lg', 'mt-0.5', bgClass)}>
                            <StreamingText text={text} highlights={highlights} />
                        </div>
                    )}
                </div>
            )}
        </div>
    );
};
