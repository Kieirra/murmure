import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import {
    applyFreeze,
    applyProvisional,
    EMPTY_STATE,
    type Segment,
    type StreamingState,
} from './use-streaming-state.helpers';

export type { FrozenSegment, HighlightRange, ProvisionalText } from './use-streaming-state.helpers';

export const useStreamingState = () => {
    const [state, setState] = useState<StreamingState>(EMPTY_STATE);

    useEffect(() => {
        const unlistenFreeze = listen<Segment>('freeze-segment', (event) => {
            setState((current) => applyFreeze(current, event.payload));
        });
        const unlistenProvisional = listen<Segment>('preview-provisional', (event) => {
            setState((current) => applyProvisional(current, event.payload));
        });
        const unlistenReset = listen('recording-mode', () => {
            setState(EMPTY_STATE);
        });

        return () => {
            unlistenFreeze.then((unlisten) => unlisten());
            unlistenProvisional.then((unlisten) => unlisten());
            unlistenReset.then((unlisten) => unlisten());
        };
    }, []);

    const hasStreamingText = state.frozenSegments.length > 0 || (state.provisional?.text.length ?? 0) > 0;

    return {
        frozenSegments: state.frozenSegments,
        provisional: state.provisional,
        hasStreamingText,
    };
};
