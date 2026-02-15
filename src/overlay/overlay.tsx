import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef, useState } from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { useLevelState } from '@/features/home/audio-visualizer/hooks/use-level-state';
import type { LLMConnectSettings } from '@/features/llm-connect/hooks/use-llm-connect';
import { CancelVisualizer } from './cancel-visualizer';
import clsx from 'clsx';

type RecordingMode = 'standard' | 'llm' | 'command';

export const Overlay = () => {
    const [feedback, setFeedback] = useState<string | null>(null);
    const [isError, setIsError] = useState(false);
    const [isCancelled, setIsCancelled] = useState(false);
    const [recordingMode, setRecordingMode] =
        useState<RecordingMode>('standard');
    const { level } = useLevelState();
    const [hasAudio, setHasAudio] = useState(false);
    const audioTimerRef = useRef<number | null>(null);
    const cancelTimerRef = useRef<number | null>(null);

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
        const unlistenFeedbackPromise = listen<string>('overlay-feedback', (event) => {
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
        const unlistenShowPromise = listen('show-overlay', () => {
            setHasAudio(false);
            if (audioTimerRef.current) {
                clearTimeout(audioTimerRef.current);
                audioTimerRef.current = null;
            }
        });
        const unlistenCancelPromise = listen('recording-cancelled', () => {
            // Clear any existing cancel timer
            if (cancelTimerRef.current) {
                clearTimeout(cancelTimerRef.current);
            }
            setIsCancelled(true);
            cancelTimerRef.current = setTimeout(() => {
                setIsCancelled(false);
                cancelTimerRef.current = null;
            }, 700);
        });

        return () => {
            unlistenFeedbackPromise.then((unlisten) => unlisten());
            unlistenSettingsPromise.then((unlisten) => unlisten());
            unlistenErrorPromise.then((unlisten) => unlisten());
            unlistenModePromise.then((unlisten) => unlisten());
            unlistenShowPromise.then((unlisten) => unlisten());
            unlistenCancelPromise.then((unlisten) => unlisten());
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
                return 'LLM';
            case 'command':
                return 'Command';
            case 'standard':
            default:
                return 'Transcription';
        }
    };

    const renderContent = () => {
        if (isCancelled) {
            return (
                <div className="origin-center h-full p-1.5 flex items-center animate-in fade-in zoom-in duration-200">
                    <CancelVisualizer bars={14} rows={9} pixelWidth={2} pixelHeight={2} />
                </div>
            );
        }
        if (feedback) {
            return (
                <span
                    className={clsx(
                        'text-[8px]',
                        'font-medium',
                        'truncate',
                        'flex',
                        'items-center',
                        'justify-center',
                        'h-full',
                        'px-1.5',
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
            );
        }
        return (
            <div className={clsx('origin-center', 'h-[20px]', 'mt-1', 'p-1.5', 'overflow-hidden')}>
                {hasAudio ? (
                    <AudioVisualizer className="-mt-3" bars={14} rows={9} audioPixelWidth={2} audioPixelHeight={2} />
                ) : (
                    <span className="text-white text-[8px] flex items-center justify-center h-full">
                        {getModeLabel(recordingMode)}
                    </span>
                )}
            </div>
        );
    };

    return (
        <div
            className={clsx(
                'w-20',
                'h-7.5',
                'rounded-sm',
                recordingMode === 'llm' && !isCancelled && 'bg-sky-950',
                recordingMode === 'command' && !isCancelled && 'bg-red-950',
                (recordingMode === 'standard' || isCancelled) && 'bg-black',
                'relative',
                'select-none',
                'overflow-hidden'
            )}
        >
            {renderContent()}
        </div>
    );
};
