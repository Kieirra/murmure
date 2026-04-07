import { useEffect, type RefObject } from 'react';

interface TrackpadGestureCallbacks {
    onMove: (dx: number, dy: number) => void;
    onScroll: (dy: number) => void;
}

export const useTrackpadGestures = (
    ref: RefObject<HTMLDivElement | null>,
    { onMove, onScroll }: TrackpadGestureCallbacks
) => {
    useEffect(() => {
        const el = ref.current;
        if (!el) return;

        let lastTouch: { x: number; y: number; scroll?: boolean } | null = null;

        const handleTouchStart = (e: TouchEvent) => {
            e.preventDefault();
            if (e.touches.length === 1) {
                lastTouch = { x: e.touches[0].clientX, y: e.touches[0].clientY };
            } else if (e.touches.length === 2) {
                const midY = (e.touches[0].clientY + e.touches[1].clientY) / 2;
                lastTouch = { x: 0, y: midY, scroll: true };
            }
        };

        const handleTouchMove = (e: TouchEvent) => {
            e.preventDefault();
            if (e.touches.length === 1 && lastTouch && !lastTouch.scroll) {
                const dx = (e.touches[0].clientX - lastTouch.x) * 1.5;
                const dy = (e.touches[0].clientY - lastTouch.y) * 1.5;
                lastTouch = { x: e.touches[0].clientX, y: e.touches[0].clientY };
                onMove(dx, dy);
            } else if (e.touches.length === 2 && lastTouch) {
                const midY = (e.touches[0].clientY + e.touches[1].clientY) / 2;
                const scrollDy = (midY - lastTouch.y) * 0.5;
                lastTouch = { x: 0, y: midY, scroll: true };
                onScroll(-scrollDy);
            }
        };

        const handleTouchEnd = (e: TouchEvent) => {
            e.preventDefault();
            lastTouch = null;
        };

        el.addEventListener('touchstart', handleTouchStart, { passive: false });
        el.addEventListener('touchmove', handleTouchMove, { passive: false });
        el.addEventListener('touchend', handleTouchEnd, { passive: false });

        return () => {
            el.removeEventListener('touchstart', handleTouchStart);
            el.removeEventListener('touchmove', handleTouchMove);
            el.removeEventListener('touchend', handleTouchEnd);
        };
    }, [ref, onMove, onScroll]);
};
