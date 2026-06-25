import { useLayoutEffect, useMemo, useRef, useState } from 'react';
import clsx from 'clsx';
import type { FrozenSegment, ProvisionalText } from './use-streaming-state';
import { buildSegments, SegmentTone, type TextSegment } from './streaming-text.helpers';

const LINE_HEIGHT_RATIO = 1.625;
const VERTICAL_PADDING_PX = 12;

interface StreamingTextProps {
    frozenSegments: FrozenSegment[];
    provisional: ProvisionalText | null;
    textWidth: number;
    fontSize: number;
    maxLines: number;
}

const segmentClassName = (segment: TextSegment) =>
    clsx(
        segment.tone === SegmentTone.Frozen && segment.highlighted && 'text-cyan-400',
        segment.tone === SegmentTone.Frozen && !segment.highlighted && 'text-white',
        segment.tone === SegmentTone.Provisional && segment.highlighted && 'text-cyan-700',
        segment.tone === SegmentTone.Provisional && !segment.highlighted && 'text-neutral-400'
    );

export const StreamingText = ({ frozenSegments, provisional, textWidth, fontSize, maxLines }: StreamingTextProps) => {
    const containerRef = useRef<HTMLDivElement>(null);
    const [hasScrolledContent, setHasScrolledContent] = useState(false);

    const segments = useMemo(() => buildSegments(frozenSegments, provisional), [frozenSegments, provisional]);
    const scrollKey = segments.map((segment) => segment.content).join('');

    useLayoutEffect(() => {
        const container = containerRef.current;
        if (!container) return;

        const scrollToBottom = () => {
            container.scrollTop = container.scrollHeight;
            setHasScrolledContent(container.scrollTop > 0);
        };

        scrollToBottom();
        const rafId = requestAnimationFrame(scrollToBottom);
        return () => cancelAnimationFrame(rafId);
    }, [scrollKey, fontSize, textWidth, maxLines]);

    return (
        <div className="relative">
            {hasScrolledContent && (
                <div className="absolute top-0 left-0 right-0 h-3 bg-gradient-to-b from-black to-transparent z-10 pointer-events-none rounded-t-lg" />
            )}
            <div
                ref={containerRef}
                className="overflow-y-auto px-2.5 py-1.5 leading-relaxed font-sans"
                style={{
                    width: `${textWidth}px`,
                    fontSize: `${fontSize}px`,
                    maxHeight: `${Math.ceil(maxLines * fontSize * LINE_HEIGHT_RATIO) + VERTICAL_PADDING_PX}px`,
                }}
            >
                {segments.map((segment) => (
                    <span key={segment.key} className={segmentClassName(segment)}>
                        {segment.content}
                    </span>
                ))}
            </div>
        </div>
    );
};
