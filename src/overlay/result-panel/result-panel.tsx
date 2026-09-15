import { useLayoutEffect, useRef, useState, type KeyboardEvent } from 'react';
import clsx from 'clsx';
import { Check, Copy, X } from 'lucide-react';
import { i18n } from '@/i18n';
import { computeTextMaxHeightPx, LINE_HEIGHT_RATIO } from '../streaming-text/streaming-text.helpers';

const SCROLL_END_TOLERANCE_PX = 1;

interface ResultPanelProps {
    text: string;
    promptName?: string | null;
    textWidth: number;
    fontSize: number;
    maxLines: number;
    durationSecs: number;
    isPaused: boolean;
    isCopied: boolean;
    onCopy: () => void;
    onPause: () => void;
    onResume: () => void;
    onClose: () => void;
}

export const ResultPanel = ({
    text,
    promptName,
    textWidth,
    fontSize,
    maxLines,
    durationSecs,
    isPaused,
    isCopied,
    onCopy,
    onPause,
    onResume,
    onClose,
}: ResultPanelProps) => {
    const scrollRef = useRef<HTMLDivElement>(null);
    const [hasContentBelow, setHasContentBelow] = useState(false);

    useLayoutEffect(() => {
        const container = scrollRef.current;
        if (container == null) return;

        const syncContentBelow = () =>
            setHasContentBelow(
                container.scrollTop + container.clientHeight < container.scrollHeight - SCROLL_END_TOLERANCE_PX
            );

        container.scrollTop = 0;
        syncContentBelow();
        container.addEventListener('scroll', syncContentBelow, { passive: true });
        return () => container.removeEventListener('scroll', syncContentBelow);
    }, [text, textWidth, fontSize, maxLines]);

    const handleKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        onCopy();
    };

    return (
        <div
            data-interactive
            role="button"
            tabIndex={0}
            aria-label={i18n.t('Copy')}
            onClick={onCopy}
            onKeyDown={handleKeyDown}
            onMouseEnter={onPause}
            onMouseLeave={onResume}
            className={clsx(
                'group',
                'relative',
                'cursor-pointer',
                'overflow-hidden',
                'rounded-t-lg',
                'bg-black',
                'px-2.5',
                'pt-1.5',
                'pb-2'
            )}
            style={{ width: `${textWidth}px` }}
        >
            <button
                type="button"
                aria-label={i18n.t('Close')}
                onClick={(event) => {
                    event.stopPropagation();
                    onClose();
                }}
                className={clsx(
                    'absolute',
                    'top-1',
                    'right-1',
                    'z-10',
                    'hidden',
                    'group-hover:flex',
                    'h-[18px]',
                    'w-[18px]',
                    'cursor-pointer',
                    'items-center',
                    'justify-center',
                    'rounded-full',
                    'bg-black',
                    'text-neutral-300',
                    'transition-all',
                    'duration-150',
                    'ease-out',
                    'hover:scale-110',
                    'hover:bg-neutral-700',
                    'hover:text-white'
                )}
            >
                <X size={10} strokeWidth={2.5} />
            </button>

            {promptName != null && promptName.length > 0 && (
                <div className="truncate pb-0.5 pr-5 font-sans text-[10px] font-normal text-sky-400">{promptName}</div>
            )}

            <div className="relative">
                <div
                    ref={scrollRef}
                    className="no-scrollbar overflow-y-auto font-sans text-white"
                    style={{
                        fontSize: `${fontSize}px`,
                        lineHeight: LINE_HEIGHT_RATIO,
                        maxHeight: `${computeTextMaxHeightPx(maxLines, fontSize)}px`,
                    }}
                >
                    {text}
                </div>
                {hasContentBelow && (
                    <div className="pointer-events-none absolute bottom-0 left-0 right-0 h-3 bg-gradient-to-t from-black to-transparent" />
                )}
            </div>

            <div
                className={clsx(
                    'flex h-4 items-center justify-end gap-1 pt-1',
                    isCopied ? 'text-white' : 'text-neutral-600 group-hover:text-white'
                )}
            >
                {isCopied ? <Check size={12} strokeWidth={2.5} /> : <Copy size={12} strokeWidth={2.5} />}
                <span className={clsx('text-[10px] font-normal leading-3', !isCopied && 'hidden group-hover:inline')}>
                    {isCopied ? i18n.t('Copied') : i18n.t('Copy')}
                </span>
            </div>

            <div
                className="absolute bottom-0 left-0 right-0 h-1 bg-neutral-800"
                style={{
                    maskImage: 'repeating-linear-gradient(to right, #000 0 10px, transparent 10px 12px)',
                    WebkitMaskImage: 'repeating-linear-gradient(to right, #000 0 10px, transparent 10px 12px)',
                }}
            >
                <div
                    className="h-full w-full"
                    style={{
                        backgroundImage:
                            'linear-gradient(to right, hsl(180, 100%, 50%) 0%, hsl(199, 89%, 48%) 25%, hsl(239, 84%, 67%) 50%, hsl(199, 89%, 48%) 75%, hsl(180, 100%, 50%) 100%)',
                        backgroundSize: `${textWidth}px 100%`,
                        backgroundRepeat: 'no-repeat',
                        animation: `result-panel-countdown ${durationSecs}s linear forwards`,
                        animationPlayState: isPaused ? 'paused' : 'running',
                    }}
                />
            </div>
        </div>
    );
};
