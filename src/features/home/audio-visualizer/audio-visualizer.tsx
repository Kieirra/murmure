import { useEffect, useRef } from 'react';
import { useLevelState } from './hooks/use-level-state';
import { useLLMState } from './hooks/use-llm-state';
import { cn } from '@/components/lib/utils';
import {
    createGradient,
    createPixelGrid,
    createVisualizerState,
    drawVisualizer,
    shouldKeepAnimating,
    startAnimation,
    type VisualizerState,
    type VisualizerSurface,
} from './audio-visualizer.helpers';

import type { ColorScheme } from '@/components/lib/color-scheme';

interface AudioVisualizerProps {
    bars?: number;
    rows?: number;
    audioPixelWidth?: number;
    audioPixelHeight?: number;
    pixelHeight?: number;
    className?: string;
    level?: number;
    isProcessing?: boolean;
    colorScheme?: ColorScheme;
}

export const AudioVisualizer = ({
    bars,
    rows = 20,
    audioPixelWidth = 12,
    audioPixelHeight = 6,
    className,
    level: levelProp,
    isProcessing: isProcessingProp,
    colorScheme = 'standard',
}: AudioVisualizerProps) => {
    const hookLevel = useLevelState();
    const hookLLM = useLLMState();
    const effectiveLevel = levelProp ?? hookLevel.level;
    const effectiveIsProcessing = isProcessingProp ?? hookLLM.isProcessing;
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const surfaceRef = useRef<VisualizerSurface | null>(null);
    const stateRef = useRef<VisualizerState | null>(null);
    stateRef.current ??= createVisualizerState();

    useEffect(() => {
        const canvas = canvasRef.current;
        const state = stateRef.current;
        const ctx = canvas?.getContext('2d');
        if (canvas == null || ctx == null || state == null) return;

        const options = { bars, rows, pixelWidth: audioPixelWidth, pixelHeight: audioPixelHeight };
        const observer = new ResizeObserver(() => {
            const pixelRatio = window.devicePixelRatio;
            const width = Math.round(canvas.clientWidth * pixelRatio);
            const height = Math.round(canvas.clientHeight * pixelRatio);
            if (width === 0 || height === 0) return;
            canvas.width = width;
            canvas.height = height;
            surfaceRef.current = {
                ctx,
                width,
                height,
                grid: createPixelGrid(width, height, options, pixelRatio),
                gradient: createGradient(ctx, width, colorScheme),
            };
            drawVisualizer(surfaceRef.current, state);
            if (shouldKeepAnimating(state)) startAnimation(surfaceRef, state);
        });
        observer.observe(canvas);

        return () => {
            observer.disconnect();
            surfaceRef.current = null;
            if (state.rafId !== null) cancelAnimationFrame(state.rafId);
            state.rafId = null;
        };
    }, [bars, rows, audioPixelWidth, audioPixelHeight, colorScheme]);

    useEffect(() => {
        const state = stateRef.current;
        if (state == null) return;

        state.level = effectiveLevel;
        state.isProcessing = effectiveIsProcessing;

        if (shouldKeepAnimating(state)) {
            startAnimation(surfaceRef, state);
        } else if (state.rafId === null) {
            drawVisualizer(surfaceRef.current, state);
        }
    }, [effectiveLevel, effectiveIsProcessing]);

    return <canvas ref={canvasRef} role="presentation" className={cn('block h-full w-full', className)} />;
};
