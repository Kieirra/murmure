import { useEffect, useRef, type RefObject } from 'react';

interface TrackpadGestureCallbacks {
    onMove: (dx: number, dy: number) => void;
    onScroll: (dy: number) => void;
    onTap?: () => void;
    onLongPress?: () => void;
}

export const useTrackpadGestures = (
    ref: RefObject<HTMLDivElement | null>,
    { onMove, onScroll, onTap, onLongPress }: TrackpadGestureCallbacks
) => {
    const callbacksRef = useRef({ onTap, onLongPress });
    callbacksRef.current = { onTap, onLongPress };

    useEffect(() => {
        const el = ref.current;
        if (!el) return;

        let lastTouch: { x: number; y: number; scroll?: boolean } | null = null;
        let touchStartPos: { x: number; y: number } | null = null;
        let touchStartTime = 0;
        let hasMoved = false;
        let longPressTimer: ReturnType<typeof setTimeout> | null = null;

        const clearLongPress = () => {
            if (longPressTimer !== null) {
                clearTimeout(longPressTimer);
                longPressTimer = null;
            }
        };

        const handleTouchStart = (e: TouchEvent) => {
            e.preventDefault();
            if (e.touches.length === 1) {
                const x = e.touches[0].clientX;
                const y = e.touches[0].clientY;
                lastTouch = { x, y };
                touchStartPos = { x, y };
                touchStartTime = Date.now();
                hasMoved = false;

                clearLongPress();
                longPressTimer = setTimeout(() => {
                    if (!hasMoved && touchStartPos) {
                        navigator.vibrate?.(30);
                        callbacksRef.current.onLongPress?.();
                        touchStartPos = null;
                    }
                }, 500);
            } else if (e.touches.length === 2) {
                const midY = (e.touches[0].clientY + e.touches[1].clientY) / 2;
                lastTouch = { x: 0, y: midY, scroll: true };
                touchStartPos = null;
                clearLongPress();
            }
        };

        const handleTouchMove = (e: TouchEvent) => {
            e.preventDefault();
            if (e.touches.length === 1 && lastTouch && !lastTouch.scroll) {
                const dx = (e.touches[0].clientX - lastTouch.x) * 1.5;
                const dy = (e.touches[0].clientY - lastTouch.y) * 1.5;
                lastTouch = { x: e.touches[0].clientX, y: e.touches[0].clientY };

                if (touchStartPos) {
                    const totalDx = Math.abs(e.touches[0].clientX - touchStartPos.x);
                    const totalDy = Math.abs(e.touches[0].clientY - touchStartPos.y);
                    if (totalDx > 5 || totalDy > 5) {
                        hasMoved = true;
                        clearLongPress();
                    }
                }

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
            clearLongPress();

            if (touchStartPos && !hasMoved && Date.now() - touchStartTime < 200) {
                callbacksRef.current.onTap?.();
            }

            lastTouch = null;
            touchStartPos = null;
        };

        el.addEventListener('touchstart', handleTouchStart, { passive: false });
        el.addEventListener('touchmove', handleTouchMove, { passive: false });
        el.addEventListener('touchend', handleTouchEnd, { passive: false });

        return () => {
            clearLongPress();
            el.removeEventListener('touchstart', handleTouchStart);
            el.removeEventListener('touchmove', handleTouchMove);
            el.removeEventListener('touchend', handleTouchEnd);
        };
    }, [ref, onMove, onScroll]);
};
