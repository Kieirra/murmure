import { useEffect, useMemo, useRef, useState } from 'react';
import type { HighlightRange } from './use-streaming-state';
import { buildSegments } from './streaming-text.helpers';

const LINE_HEIGHT_RATIO = 1.625;
const VERTICAL_PADDING_PX = 12;

interface StreamingTextProps {
    text: string;
    highlights: HighlightRange[];
    textWidth: number;
    fontSize: number;
    maxLines: number;
}

export const StreamingText = ({ text, highlights, textWidth, fontSize, maxLines }: StreamingTextProps) => {
    const containerRef = useRef<HTMLDivElement>(null);
    const [hasScrolledContent, setHasScrolledContent] = useState(false);

    useEffect(() => {
        if (containerRef.current) {
            containerRef.current.scrollTop = containerRef.current.scrollHeight;
            setHasScrolledContent(containerRef.current.scrollTop > 0);
        }
    }, [text]);

    const segments = useMemo(() => buildSegments(text, highlights), [text, highlights]);

    return (
        <div className="relative">
            {hasScrolledContent && (
                <div className="absolute top-0 left-0 right-0 h-3 bg-gradient-to-b from-black to-transparent z-10 pointer-events-none rounded-t-lg" />
            )}
            <div
                ref={containerRef}
                className="overflow-y-auto px-2.5 py-1.5 leading-relaxed font-sans"
                style={{ width: `${textWidth}px`, fontSize: `${fontSize}px`, maxHeight: `${Math.ceil(maxLines * fontSize * LINE_HEIGHT_RATIO) + VERTICAL_PADDING_PX}px` }}
            >
                {segments.map((segment) => (
                    <span
                        key={segment.key}
                        className={segment.highlighted ? 'text-cyan-400' : 'text-white'}
                    >
                        {segment.content}
                    </span>
                ))}
            </div>
        </div>
    );
};

