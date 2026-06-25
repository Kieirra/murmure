export interface HighlightRange {
    start: number;
    end: number;
}

export interface Segment {
    seq: number;
    text: string;
    highlights: HighlightRange[];
}

export type FrozenSegment = Segment;
export type ProvisionalText = Segment;

export interface StreamingState {
    frozenSegments: FrozenSegment[];
    provisional: ProvisionalText | null;
}

export const EMPTY_STATE: StreamingState = { frozenSegments: [], provisional: null };

export const lastFrozenSeq = (frozenSegments: FrozenSegment[]) => {
    if (frozenSegments.length === 0) return -1;
    return frozenSegments[frozenSegments.length - 1].seq;
};

export const upsertFrozenSegment = (
    frozenSegments: FrozenSegment[],
    segment: FrozenSegment
): FrozenSegment[] => {
    const withoutSeq = frozenSegments.filter((existing) => existing.seq !== segment.seq);
    withoutSeq.push(segment);
    withoutSeq.sort((a, b) => a.seq - b.seq);
    return withoutSeq;
};

export const applyFreeze = (state: StreamingState, payload: Segment): StreamingState => {
    const frozenSegments = upsertFrozenSegment(state.frozenSegments, {
        seq: payload.seq,
        text: payload.text,
        highlights: payload.highlights,
    });
    const provisional = state.provisional != null && state.provisional.seq <= payload.seq ? null : state.provisional;
    return { frozenSegments, provisional };
};

export const applyProvisional = (state: StreamingState, payload: Segment): StreamingState => {
    if (payload.seq <= lastFrozenSeq(state.frozenSegments)) {
        return state;
    }
    return {
        frozenSegments: state.frozenSegments,
        provisional: { seq: payload.seq, text: payload.text, highlights: payload.highlights },
    };
};
