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

        cleanupRef.current = () => {
            if (frame != null) cancelAnimationFrame(frame);
            resizeObserver.disconnect();
            mutationObserver.disconnect();
        };
    }, []);

    useEffect(() => () => cleanupRef.current?.(), []);

    return setRoot;
};
