import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef, useState } from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { useLevelState } from '@/features/home/audio-visualizer/hooks/use-level-state';
import { useStreamingState } from './streaming-text/use-streaming-state';
import { StreamingText } from './streaming-text/streaming-text';
import type { LLMConnectSettings } from '@/features/extensions/llm-connect/hooks/use-llm-connect';
import clsx from 'clsx';

type OverlaySize = 'small' | 'medium' | 'large';

const VISUALIZER_CONFIG: Record<OverlaySize, { bars: number; pixelWidth: number; pixelHeight: number }> = {
    small: { bars: 14, pixelWidth: 2, pixelHeight: 2 },
    medium: { bars: 16, pixelWidth: 2, pixelHeight: 2 },
    large: { bars: 24, pixelWidth: 3, pixelHeight: 3 },
};

type RecordingMode = 'standard' | 'llm' | 'command';

export const Overlay = () => {
    const [feedback, setFeedback] = useState<string | null>(null);
    const [isError, setIsError] = useState(false);
    const [recordingMode, setRecordingMode] = useState<RecordingMode>('standard');
    const { level } = useLevelState();
    const [hasAudio, setHasAudio] = useState(false);
    const audioTimerRef = useRef<number | null>(null);
    const [overlaySize, setOverlaySize] = useState<OverlaySize>('small');
    const [streamingTextSettings, setStreamingTextSettings] = useState({ textWidth: 450, fontSize: 11, maxLines: 5 });
    const { text, highlights, hasStreamingText } = useStreamingState();
    const [streamingEpoch, setStreamingEpoch] = useState(0);

    useEffect(() => {
        invoke<{ overlay_size?: string; streaming_text_width?: number; streaming_font_size?: number; streaming_max_lines?: number }>('get_all_settings').then((settings) => {
            const sz = settings.overlay_size;
            if (sz === 'small' || sz === 'medium' || sz === 'large') setOverlaySize(sz);
            setStreamingTextSettings((prev) => ({
                textWidth: typeof settings.streaming_text_width === 'number' ? settings.streaming_text_width : prev.textWidth,
                fontSize: typeof settings.streaming_font_size === 'number' ? settings.streaming_font_size : prev.fontSize,
                maxLines: typeof settings.streaming_max_lines === 'number' ? settings.streaming_max_lines : prev.maxLines,
            }));
        });
    }, []);

    useEffect(() => {
        const unlisten = listen<string>('overlay-size-changed', (event) => {
            const sz = event.payload;
            if (sz === 'small' || sz === 'medium' || sz === 'large') setOverlaySize(sz);
        });
        return () => { unlisten.then((u) => u()); };
    }, []);

    useEffect(() => {
        const unlisten = listen<{ text_width: number; font_size: number; max_lines: number }>('streaming-text-settings-changed', (event) => {
            setStreamingTextSettings({
                textWidth: event.payload.text_width,
                fontSize: event.payload.font_size,
                maxLines: event.payload.max_lines,
            });
        });
        return () => { unlisten.then((u) => u()); };
    }, []);

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
            setStreamingEpoch((prev) => prev + 1);
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

    const bgClass = clsx(
        recordingMode === 'llm' && 'bg-sky-950',
        recordingMode === 'command' && 'bg-red-950',
        recordingMode === 'standard' && 'bg-black',
    );

    return (
        <div className="w-full min-h-[36px] relative select-none flex flex-col items-center">
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
                        'overflow-hidden',
                        'flex', 'items-center', 'justify-center',
                        bgClass,
                        overlaySize === 'small' && 'w-20 h-7.5 rounded-sm p-1.5',
                        overlaySize === 'medium' && 'w-[120px] h-[36px] rounded-lg py-1 px-2',
                        overlaySize === 'large' && 'w-1/2 h-[40px] rounded-lg py-1 px-3',
                    )}>
                        {hasAudio ? (
                            <AudioVisualizer
                                bars={VISUALIZER_CONFIG[overlaySize].bars}
                                rows={9}
                                audioPixelWidth={VISUALIZER_CONFIG[overlaySize].pixelWidth}
                                audioPixelHeight={VISUALIZER_CONFIG[overlaySize].pixelHeight}
                            />
                        ) : (
                            <span className="text-white text-[8px] flex items-center justify-center h-full">
                                {getModeLabel(recordingMode)}
                            </span>
                        )}
                    </div>
                    {hasStreamingText && (
                        <div className={clsx('w-full', 'rounded-lg', 'mt-0.5', bgClass)}>
                            <StreamingText key={streamingEpoch} text={text} highlights={highlights} textWidth={streamingTextSettings.textWidth} fontSize={streamingTextSettings.fontSize} maxLines={streamingTextSettings.maxLines} />
                        </div>
                    )}
                </div>
            )}
        </div>
    );
};
