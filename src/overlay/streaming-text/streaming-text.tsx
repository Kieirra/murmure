import { useEffect, useRef } from 'react';
import clsx from 'clsx';

interface HighlightRange {
    start: number;
    end: number;
}

interface StreamingTextProps {
    text: string;
    highlights: HighlightRange[];
}

export const StreamingText = ({ text, highlights }: StreamingTextProps) => {
    const containerRef = useRef<HTMLDivElement>(null);
    const seenHighlightsRef = useRef<Set<string>>(new Set());
    const prevTextRef = useRef<string>(text);

    useEffect(() => {
        if (text === '' && prevTextRef.current !== '') {
            seenHighlightsRef.current.clear();
        }
        prevTextRef.current = text;
    }, [text]);

    useEffect(() => {
        if (containerRef.current) {
            containerRef.current.scrollTop = containerRef.current.scrollHeight;
        }
    }, [text]);

    const segments = buildSegments(text, highlights);

    return (
        <div className="border-t border-white/20 mt-1 mb-0.5">
            <div
                ref={containerRef}
                className="overflow-y-auto max-h-[60px] px-2.5 py-1.5 text-xs leading-relaxed font-mono"
            >
                {segments.map((segment) => {
                    if (!segment.highlighted) {
                        return (
                            <span
                                key={segment.key}
                                className="text-white/90 animate-in fade-in duration-300"
                            >
                                {segment.content}
                            </span>
                        );
                    }

                    const highlightKey = `${segment.start}-${segment.end}`;
                    const isNew = !seenHighlightsRef.current.has(highlightKey);
                    if (isNew) {
                        seenHighlightsRef.current.add(highlightKey);
                    }

                    return (
                        <span
                            key={segment.key}
                            className={clsx(
                                'text-amber-400',
                                isNew && 'animate-in fade-in duration-300'
                            )}
                        >
                            {segment.content}
                        </span>
                    );
                })}
            </div>
        </div>
    );
};

interface TextSegment {
    key: string;
    content: string;
    highlighted: boolean;
    start: number;
    end: number;
}

function buildSegments(text: string, highlights: HighlightRange[]): TextSegment[] {
    if (highlights.length === 0) {
        return [{ key: 'text-0', content: text, highlighted: false, start: 0, end: text.length }];
    }

    const encoder = new TextEncoder();
    const bytes = encoder.encode(text);

    const charToByteOffset: number[] = [];
    let byteIndex = 0;
    for (let i = 0; i < text.length; i++) {
        charToByteOffset.push(byteIndex);
        const code = text.codePointAt(i);
        if (code === undefined) break;
        if (code <= 0x7f) {
            byteIndex += 1;
        } else if (code <= 0x7ff) {
            byteIndex += 2;
        } else if (code <= 0xffff) {
            byteIndex += 3;
        } else {
            byteIndex += 4;
            i++;
        }
    }
    charToByteOffset.push(bytes.length);

    const byteToCharOffset = new Map<number, number>();
    for (let i = 0; i < charToByteOffset.length; i++) {
        byteToCharOffset.set(charToByteOffset[i], i);
    }

    const sorted = [...highlights].sort((a, b) => a.start - b.start);

    const segments: TextSegment[] = [];
    let currentCharPos = 0;
    let segmentIndex = 0;

    for (const highlight of sorted) {
        const highlightCharStart = byteToCharOffset.get(highlight.start);
        const highlightCharEnd = byteToCharOffset.get(highlight.end);

        if (highlightCharStart === undefined || highlightCharEnd === undefined) {
            continue;
        }

        if (currentCharPos < highlightCharStart) {
            segments.push({
                key: `text-${segmentIndex++}`,
                content: text.slice(currentCharPos, highlightCharStart),
                highlighted: false,
                start: currentCharPos,
                end: highlightCharStart,
            });
        }

        segments.push({
            key: `hl-${segmentIndex++}`,
            content: text.slice(highlightCharStart, highlightCharEnd),
            highlighted: true,
            start: highlight.start,
            end: highlight.end,
        });

        currentCharPos = highlightCharEnd;
    }

    if (currentCharPos < text.length) {
        segments.push({
            key: `text-${segmentIndex}`,
            content: text.slice(currentCharPos),
            highlighted: false,
            start: currentCharPos,
            end: text.length,
        });
    }

    return segments;
}
