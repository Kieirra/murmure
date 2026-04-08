import { useRef } from 'react';
import { useTrackpadGestures } from '../hooks/use-trackpad-gestures';

interface TrackpadProps {
    onMove: (dx: number, dy: number) => void;
    onScroll: (dy: number) => void;
    onTap?: () => void;
    onLongPress?: () => void;
}

export const Trackpad = ({ onMove, onScroll, onTap, onLongPress }: TrackpadProps) => {
    const ref = useRef<HTMLDivElement>(null);
    useTrackpadGestures(ref, { onMove, onScroll, onTap, onLongPress });

    return (
        <div
            ref={ref}
            className="h-[230px] shrink-0 bg-[#111] border border-[#333] rounded-lg m-2 relative"
            style={{ touchAction: 'none' }}
        >
            <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-10 h-10 border border-[#333] rounded-full opacity-30 pointer-events-none" />
        </div>
    );
};
