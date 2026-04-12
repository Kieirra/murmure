import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { useStreamingState } from './streaming-text/use-streaming-state';
import { StreamingText } from './streaming-text/streaming-text';
import clsx from 'clsx';
import { VISUALIZER_CONFIG } from './visualizer-config';
import { useOverlayConfig } from './use-overlay-config';
import { useRecordingMode } from './use-recording-mode';
import { useOverlayError } from './use-overlay-error';

export const Overlay = () => {
    const { overlaySize, streamingTextSettings } = useOverlayConfig();
    const recordingMode = useRecordingMode();
    const error = useOverlayError();
    const { text, highlights, hasStreamingText } = useStreamingState();

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
                        'h-9',
                        'px-2',
                        'rounded-lg',
                        'bg-black',
                        'animate-in',
                        'fade-in',
                        'zoom-in',
                        'duration-200',
                        'text-red-500'
                    )}
                >
                    {error}
                </span>
            ) : (
                <div className="flex flex-col items-center w-full">
                    <div
                        className={clsx(
                            'overflow-hidden',
                            'flex',
                            'items-center',
                            'justify-center',
                            'bg-black',
                            VISUALIZER_CONFIG[overlaySize].className
                        )}
                    >
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
                            <StreamingText
                                text={text}
                                highlights={highlights}
                                textWidth={streamingTextSettings.textWidth}
                                fontSize={streamingTextSettings.fontSize}
                                maxLines={streamingTextSettings.maxLines}
                            />
                        </div>
                    )}
                </div>
            )}
        </div>
    );
};
