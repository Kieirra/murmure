import type { HighlightRange } from './use-streaming-state';

export interface TextSegment {
    key: string;
    content: string;
    highlighted: boolean;
}

export const buildSegments = (text: string, highlights: HighlightRange[]): TextSegment[] => {
    if (highlights.length === 0) {
        return [{ key: 'text-0', content: text, highlighted: false }];
    }

    const sorted = [...highlights].sort((a, b) => a.start - b.start);
    const segments: TextSegment[] = [];
    let pos = 0;
    let idx = 0;

    for (const h of sorted) {
        if (pos < h.start) {
            segments.push({ key: `t-${idx++}`, content: text.slice(pos, h.start), highlighted: false });
        }
        segments.push({ key: `h-${idx++}`, content: text.slice(h.start, h.end), highlighted: true });
        pos = h.end;
    }

    if (pos < text.length) {
        segments.push({ key: `t-${idx}`, content: text.slice(pos), highlighted: false });
    }

    return segments;
};
