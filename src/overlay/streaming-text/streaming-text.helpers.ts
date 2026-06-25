import type { FrozenSegment, HighlightRange, ProvisionalText } from './use-streaming-state';

export enum SegmentTone {
    Frozen = 'frozen',
    Provisional = 'provisional',
}

export interface TextSegment {
    key: string;
    content: string;
    highlighted: boolean;
    tone: SegmentTone;
}

const splitByHighlights = (
    text: string,
    highlights: HighlightRange[],
    tone: SegmentTone,
    keyPrefix: string
): TextSegment[] => {
    if (highlights.length === 0) {
        return [{ key: `${keyPrefix}-0`, content: text, highlighted: false, tone }];
    }

    const sorted = [...highlights].sort((a, b) => a.start - b.start);
    const segments: TextSegment[] = [];
    let pos = 0;
    let idx = 0;

    for (const highlight of sorted) {
        if (pos < highlight.start) {
            segments.push({
                key: `${keyPrefix}-t-${idx++}`,
                content: text.slice(pos, highlight.start),
                highlighted: false,
                tone,
            });
        }
        segments.push({
            key: `${keyPrefix}-h-${idx++}`,
            content: text.slice(highlight.start, highlight.end),
            highlighted: true,
            tone,
        });
        pos = highlight.end;
    }

    if (pos < text.length) {
        segments.push({ key: `${keyPrefix}-t-${idx}`, content: text.slice(pos), highlighted: false, tone });
    }

    return segments;
};

export const buildSegments = (frozenSegments: FrozenSegment[], provisional: ProvisionalText | null): TextSegment[] => {
    const segments: TextSegment[] = [];

    for (const frozen of frozenSegments) {
        segments.push(...splitByHighlights(frozen.text, frozen.highlights, SegmentTone.Frozen, `f-${frozen.seq}`));
    }

    if (provisional != null && provisional.text.length > 0) {
        segments.push(
            ...splitByHighlights(
                provisional.text,
                provisional.highlights,
                SegmentTone.Provisional,
                `p-${provisional.seq}`
            )
        );
    }

    return segments;
};
