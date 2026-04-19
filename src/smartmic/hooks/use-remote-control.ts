import { useCallback } from 'react';
import type { ClientMessage } from '../smartmic.types';

export interface RemoteControl {
    onMove: (dx: number, dy: number) => void;
    onScroll: (dy: number) => void;
    onTap: () => void;
    onLongPress: () => void;
    onEnter: () => void;
    onBackspace: () => void;
}

export const useRemoteControl = (sendJson: (msg: ClientMessage) => void): RemoteControl => {
    // `onMove` and `onScroll` feed a useEffect dep array inside the trackpad
    // gesture hook, so their identity must be stable while `sendJson` is stable.
    const onMove = useCallback(
        (dx: number, dy: number) => {
            sendJson({ type: 'mouse_move', dx, dy });
        },
        [sendJson]
    );

    const onScroll = useCallback(
        (dy: number) => {
            sendJson({ type: 'scroll', dy });
        },
        [sendJson]
    );

    const onTap = () => sendJson({ type: 'click', button: 'left' });
    const onLongPress = () => sendJson({ type: 'click', button: 'right' });
    const onEnter = () => sendJson({ type: 'key_press', key: 'Return' });
    const onBackspace = () => sendJson({ type: 'key_press', key: 'BackSpace' });

    return { onMove, onScroll, onTap, onLongPress, onEnter, onBackspace };
};
