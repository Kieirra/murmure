import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { useStreamingState } from './streaming-text/use-streaming-state';
import { StreamingText } from './streaming-text/streaming-text';
import type { AppSettings } from '@/features/settings/settings.types';
import clsx from 'clsx';
import { VISUALIZER_CONFIG } from './visualizer-config';
import type { OverlaySize } from './visualizer-config';

type RecordingMode = 'standard' | 'llm' | 'command';

export const Overlay = () => {
    const [error, setError] = useState<string | null>(null);
    const [recordingMode, setRecordingMode] = useState<RecordingMode>('standard');
    const [overlaySize, setOverlaySize] = useState<OverlaySize>('small');
    const [streamingTextSettings, setStreamingTextSettings] = useState({ textWidth: 450, fontSize: 11, maxLines: 5 });
    const { text, highlights, hasStreamingText } = useStreamingState();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            const sz = settings.overlay_size;
            if (sz === 'small' || sz === 'medium' || sz === 'large') setOverlaySize(sz);
            setStreamingTextSettings((prev) => ({
                textWidth: typeof settings.streaming_text_width === 'number' ? settings.streaming_text_width : prev.textWidth,
                fontSize: typeof settings.streaming_font_size === 'number' ? settings.streaming_font_size : prev.fontSize,
                maxLines: typeof settings.streaming_max_lines === 'number' ? settings.streaming_max_lines : prev.maxLines,
            }));
        });
        invoke<string>('get_recording_mode').then((mode) => {
            if (mode === 'llm' || mode === 'command' || mode === 'standard') setRecordingMode(mode);
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
        const unlistenError = listen<string>('llm-error', (event) => {
            setError(event.payload);
        });
        const unlistenRecordingError = listen<string>('recording-error', () => {
            setError('Mic error');
        });
        return () => {
            unlistenError.then((u) => u());
            unlistenRecordingError.then((u) => u());
        };
    }, []);

    useEffect(() => {
        if (error) {
            const timer = setTimeout(() => setError(null), 2000);
            return () => clearTimeout(timer);
        }
    }, [error]);

    return (
        <div className="w-full min-h-[36px] relative select-none flex flex-col items-center">
            {error ? (
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
                        'bg-black',
                        'animate-in',
                        'fade-in',
                        'zoom-in',
                        'duration-200',
                        'text-red-500',
                    )}
                >
                    {error}
                </span>
            ) : (
                <div className="flex flex-col items-center w-full">
                    <div className={clsx(
                        'overflow-hidden',
                        'flex', 'items-center', 'justify-center',
                        'bg-black',
                        VISUALIZER_CONFIG[overlaySize].className,
                    )}>
                        <AudioVisualizer
                            bars={VISUALIZER_CONFIG[overlaySize].bars}
                            rows={9}
                            audioPixelWidth={VISUALIZER_CONFIG[overlaySize].pixelWidth}
                            audioPixelHeight={VISUALIZER_CONFIG[overlaySize].pixelHeight}
                            colorScheme={recordingMode}
                        />
                    </div>
                    {hasStreamingText && (
                        <div className="w-full rounded-lg mt-0.5 bg-black">
                            <StreamingText text={text} highlights={highlights} textWidth={streamingTextSettings.textWidth} fontSize={streamingTextSettings.fontSize} maxLines={streamingTextSettings.maxLines} />
                        </div>
                    )}
                </div>
            )}
        </div>
    );
};
