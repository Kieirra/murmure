import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useRef } from 'react';

interface InputRect {
    x: number;
    y: number;
    width: number;
    height: number;
}

const toPhysicalRect = (rect: DOMRect, dpr: number): InputRect => ({
    x: Math.round(rect.left * dpr),
    y: Math.round(rect.top * dpr),
    width: Math.round(rect.width * dpr),
    height: Math.round(rect.height * dpr),
});

export const useOverlayInputRegion = () => {
    const cleanupRef = useRef<(() => void) | null>(null);

    const setRoot = useCallback((root: HTMLElement | null) => {
        cleanupRef.current?.();
        cleanupRef.current = null;
        if (root == null) return;

        let frame: number | null = null;

        const compute = () => {
            const dpr = window.devicePixelRatio;
            const rects = Array.from(root.querySelectorAll<HTMLElement>('[data-interactive]'))
                .map((element) => toPhysicalRect(element.getBoundingClientRect(), dpr))
                .filter((rect) => rect.width > 0 && rect.height > 0);
            invoke('set_overlay_input_region', { rects }).catch(() => {});
        };

        const scheduleCompute = () => {
            if (frame != null) return;
            frame = requestAnimationFrame(() => {
                frame = null;
                compute();
            });
        };

        scheduleCompute();

        const resizeObserver = new ResizeObserver(scheduleCompute);
        resizeObserver.observe(root);

        const mutationObserver = new MutationObserver(scheduleCompute);
        mutationObserver.observe(root, { childList: true, subtree: true, attributes: true });

        // `animate-in zoom-in` enters from scale 0, and the MutationObserver
        // measures the node on the frame it is inserted, so the rect is
        // captured at almost zero size. A CSS animation raises no further
        // mutation and the ResizeObserver only watches the root, whose size
        // never changes, so nothing would ever correct it. Animation events
        // bubble, so one listener covers every animated descendant.
        root.addEventListener('animationend', scheduleCompute);
        root.addEventListener('transitionend', scheduleCompute);

        cleanupRef.current = () => {
            if (frame != null) cancelAnimationFrame(frame);
            resizeObserver.disconnect();
            mutationObserver.disconnect();
            root.removeEventListener('animationend', scheduleCompute);
            root.removeEventListener('transitionend', scheduleCompute);
        };
    }, []);

    useEffect(() => () => cleanupRef.current?.(), []);

    return setRoot;
};
