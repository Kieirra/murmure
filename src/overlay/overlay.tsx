import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { useStreamingState } from './streaming-text/use-streaming-state';
import { StreamingText } from './streaming-text/streaming-text';
import clsx from 'clsx';
import { VISUALIZER_CONFIG } from './visualizer-config';
import { useOverlayConfig } from './use-overlay-config';
import { useRecordingMode } from './use-recording-mode';
import { useOverlayError, OverlayErrorKind } from './use-overlay-error';
import { useModeFlash } from './mode-flash/hooks/use-mode-flash';
import { ModeFlash } from './mode-flash/mode-flash';
import { OverlayErrorBadge } from './overlay-error-badge/overlay-error-badge';

export const Overlay = () => {
    const { overlaySize, overlayPosition, streamingTextSettings } = useOverlayConfig();
    const recordingMode = useRecordingMode();
    const error = useOverlayError();
    const { text, highlights, hasStreamingText } = useStreamingState();
    const { text: flashText, isFadingOut } = useModeFlash();

    const renderContent = () => {
        if (error?.kind === OverlayErrorKind.Fatal) {
            return (
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
                    {error.message}
                </span>
            );
        }
        if (flashText != null && !hasStreamingText) {
            return <ModeFlash text={flashText} isFadingOut={isFadingOut} />;
        }

        const chunkErrorBadge = error?.kind === OverlayErrorKind.Chunk && (
            <OverlayErrorBadge message={error.message} />
        );
        const visualizer = (
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
        );

        const textBlock = (() => {
            if (hasStreamingText) {
                return (
                    <div className={clsx('w-fit', 'rounded-lg', 'bg-black')}>
                        <StreamingText
                            text={text}
                            highlights={highlights}
                            textWidth={streamingTextSettings.textWidth}
                            fontSize={streamingTextSettings.fontSize}
                            maxLines={streamingTextSettings.maxLines}
                        />
                    </div>
                );
            }
            return null;
        })();

        const topBlock = textBlock != null && (
            <div className={clsx(overlayPosition === 'bottom' ? 'mb-0.5' : 'mt-0.5')}>{textBlock}</div>
        );
        const badge = chunkErrorBadge && (
            <div className={clsx(overlayPosition === 'bottom' ? 'mb-0.5' : 'mt-0.5')}>{chunkErrorBadge}</div>
        );

        return (
            <div className="flex flex-col items-center w-full">
                {overlayPosition === 'bottom' ? (
                    <>
                        {badge}
                        {topBlock}
                        {visualizer}
                    </>
                ) : (
                    <>
                        {visualizer}
                        {topBlock}
                        {badge}
                    </>
                )}
            </div>
        );
    };

    if (overlayPosition === undefined) return null;

    return (
        <div
            className={clsx(
                'w-full',
                'h-screen',
                'min-h-[36px]',
                'relative',
                'select-none',
                'flex',
                'flex-col',
                'items-center',
                overlayPosition === 'bottom' ? 'justify-end pb-3' : 'justify-start pt-3'
            )}
        >
            {renderContent()}
        </div>
    );
};
