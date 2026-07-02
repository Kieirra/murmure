import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { useStreamingState } from './streaming-text/use-streaming-state';
import { StreamingText } from './streaming-text/streaming-text';
import clsx from 'clsx';
import { VISUALIZER_CONFIG } from './visualizer-config';
import { useOverlayConfig } from './use-overlay-config';
import { useRecordingMode } from './use-recording-mode';
import { useOverlayError } from './use-overlay-error';
import { useModeFlash } from './mode-flash/hooks/use-mode-flash';
import { useLlmPromptFlash } from './mode-flash/hooks/use-llm-prompt-flash';
import { ModeFlash } from './mode-flash/mode-flash';
import { CancelButton } from './cancel-button';
import { useOverlayInputRegion } from './use-overlay-input-region';

export const Overlay = () => {
    const { overlaySize, overlayPosition, streamingTextSettings } = useOverlayConfig();
    const recordingMode = useRecordingMode();
    const error = useOverlayError();
    const { frozenSegments, provisional, hasStreamingText } = useStreamingState();
    const { text: flashText, isFadingOut } = useModeFlash();
    const { promptName } = useLlmPromptFlash();
    const setRoot = useOverlayInputRegion();
    const showPromptName = promptName != null && !hasStreamingText;

    const renderContent = () => {
        if (error != null) {
            return (
                <span
                    data-interactive
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
            );
        }
        if (flashText != null && !hasStreamingText) {
            return <ModeFlash text={flashText} isFadingOut={isFadingOut} />;
        }

        const visualizer = (
            <div className="relative w-fit">
                <CancelButton size={overlaySize} />
                <div
                    data-interactive
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
            </div>
        );

        const textBlock = (() => {
            if (showPromptName) {
                return (
                    <div
                        data-interactive
                        className={clsx(
                            'max-w-[160px]',
                            'truncate',
                            'text-center',
                            'rounded',
                            'bg-black',
                            'px-2',
                            'py-1',
                            'text-[10px]',
                            'font-normal',
                            'text-white',
                            'transition-opacity',
                            'duration-200'
                        )}
                    >
                        {promptName}
                    </div>
                );
            }
            if (hasStreamingText) {
                return (
                    <div data-interactive className={clsx('w-fit', 'rounded-lg', 'bg-black')}>
                        <StreamingText
                            frozenSegments={frozenSegments}
                            provisional={provisional}
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

        return (
            <div className="flex flex-col items-center w-full">
                {overlayPosition === 'bottom' ? (
                    <>
                        {topBlock}
                        {visualizer}
                    </>
                ) : (
                    <>
                        {visualizer}
                        {topBlock}
                    </>
                )}
            </div>
        );
    };

    if (overlayPosition === undefined) return null;

    return (
        <div
            ref={setRoot}
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
