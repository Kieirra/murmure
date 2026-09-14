import { useLayoutEffect, useRef, useState } from 'react';
import clsx from 'clsx';
import { Check, Copy, X } from 'lucide-react';
import { i18n } from '@/i18n';
import { computeTextMaxHeightPx, LINE_HEIGHT_RATIO } from '../streaming-text/streaming-text.helpers';

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
    const [hasOverflow, setHasOverflow] = useState(false);

    useLayoutEffect(() => {
        const container = scrollRef.current;
        if (container == null) return;
        container.scrollTop = 0;
        setHasOverflow(container.scrollHeight > container.clientHeight);
    }, [text, textWidth, fontSize, maxLines]);

    return (
        <div
            data-interactive
            role="button"
            aria-label={i18n.t('Copy')}
            onClick={onCopy}
            onMouseEnter={onPause}
            onMouseLeave={onResume}
            className={clsx(
                'group',
                'relative',
                'cursor-pointer',
                'overflow-hidden',
                'rounded-lg',
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
                {hasOverflow && (
                    <div className="pointer-events-none absolute bottom-0 left-0 right-0 h-3 bg-gradient-to-t from-black to-transparent" />
                )}
            </div>

            <div
                className={clsx(
                    'flex items-center justify-end gap-1 pt-1',
                    isCopied ? 'text-white' : 'text-neutral-600 group-hover:text-white'
                )}
            >
                {isCopied ? <Check size={12} strokeWidth={2.5} /> : <Copy size={12} strokeWidth={2.5} />}
                <span className={clsx('text-[10px] font-normal', !isCopied && 'hidden group-hover:inline')}>
                    {isCopied ? i18n.t('Copied') : i18n.t('Copy')}
                </span>
            </div>

            <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-neutral-800">
                <div
                    className={clsx('h-full w-full origin-left', isPaused ? 'bg-sky-700' : 'bg-sky-400')}
                    style={{
                        animation: `result-panel-countdown ${durationSecs}s linear forwards`,
                        animationPlayState: isPaused ? 'paused' : 'running',
                    }}
                />
            </div>
        </div>
    );
};
