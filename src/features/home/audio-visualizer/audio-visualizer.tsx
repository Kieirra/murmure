import { useEffect, useRef, type RefObject } from 'react';
import { useLevelState } from './hooks/use-level-state';
import { useLLMState } from './hooks/use-llm-state';
import { cn } from '@/components/lib/utils';
import {
    advanceVisualizer,
    columnHeight,
    createVisualizerState,
    gradientStops,
    isRowLit,
    shouldKeepAnimating,
    type VisualizerState,
} from './audio-visualizer.helpers';

import type { ColorScheme } from './audio-pixel/audio-pixel.helpers';

const FIXED_GAP = 2;
const FILL_CELL_SIZE = 7;
const FILL_GAP = 4;
const GLOW_GAP_RATIO = 0.35;
const GLOW_ALPHA = 0.25;

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

interface GridOptions {
    bars?: number;
    rows: number;
    pixelWidth: number;
    pixelHeight: number;
}

interface PixelGrid {
    columns: number;
    rows: number;
    cellWidth: number;
    cellHeight: number;
    pitchX: number;
    pitchY: number;
    originX: number;
    originY: number;
    glowPadding: number;
}

interface VisualizerSurface {
    ctx: CanvasRenderingContext2D;
    width: number;
    height: number;
    grid: PixelGrid | null;
    gradient: CanvasGradient;
}

const toOdd = (count: number) => (count % 2 === 0 ? count - 1 : count);

const createFillGrid = (width: number, height: number, pixelRatio: number): PixelGrid | null => {
    const cell = FILL_CELL_SIZE * pixelRatio;
    const gap = FILL_GAP * pixelRatio;
    const pitch = cell + gap;
    const columns = toOdd(Math.floor((width + gap) / pitch));
    const rows = toOdd(Math.floor((height + gap) / pitch));
    if (columns < 3 || rows < 1) return null;
    return {
        columns,
        rows,
        cellWidth: cell,
        cellHeight: cell,
        pitchX: pitch,
        pitchY: pitch,
        originX: (width - (columns * pitch - gap)) / 2,
        originY: (height - (rows * pitch - gap)) / 2,
        glowPadding: gap * GLOW_GAP_RATIO,
    };
};

const createPixelGrid = (
    width: number,
    height: number,
    { bars, rows, pixelWidth, pixelHeight }: GridOptions,
    pixelRatio: number
): PixelGrid | null => {
    if (bars == null) return createFillGrid(width, height, pixelRatio);
    if (bars < 2 || rows < 1) return null;
    const cellWidth = pixelWidth * pixelRatio;
    const cellHeight = pixelHeight * pixelRatio;
    const gap = FIXED_GAP * pixelRatio;
    const pitchX = width / bars;
    const pitchY = cellHeight + gap;
    return {
        columns: bars,
        rows,
        cellWidth,
        cellHeight,
        pitchX,
        pitchY,
        originX: (pitchX - cellWidth) / 2,
        originY: (height - (rows * pitchY - gap)) / 2,
        glowPadding: gap * GLOW_GAP_RATIO,
    };
};

const createGradient = (ctx: CanvasRenderingContext2D, width: number, colorScheme: ColorScheme) => {
    const gradient = ctx.createLinearGradient(0, 0, width, 0);
    gradientStops(colorScheme).forEach(([offset, color]) => gradient.addColorStop(offset, color));
    return gradient;
};

const traceCells = (ctx: CanvasRenderingContext2D, state: VisualizerState, grid: PixelGrid, padding: number) => {
    const halfRows = Math.floor(grid.rows / 2);
    const width = grid.cellWidth + padding * 2;
    const height = grid.cellHeight + padding * 2;
    ctx.beginPath();
    for (let column = 0; column < grid.columns; column++) {
        const litHalfRows = Math.floor(columnHeight(state, column, grid.columns) * halfRows);
        const x = grid.originX + column * grid.pitchX - padding;
        for (let row = 0; row < grid.rows; row++) {
            if (isRowLit(row, litHalfRows, grid.rows)) {
                ctx.rect(x, grid.originY + row * grid.pitchY - padding, width, height);
            }
        }
    }
};

const drawVisualizer = (surface: VisualizerSurface | null, state: VisualizerState) => {
    if (surface == null) return;
    const { ctx, width, height, grid, gradient } = surface;

    ctx.clearRect(0, 0, width, height);
    if (grid == null) return;

    ctx.fillStyle = gradient;

    ctx.globalAlpha = GLOW_ALPHA;
    traceCells(ctx, state, grid, grid.glowPadding);
    ctx.fill();

    ctx.globalAlpha = 1;
    traceCells(ctx, state, grid, 0);
    ctx.fill();
};

const startAnimation = (surfaceRef: RefObject<VisualizerSurface | null>, state: VisualizerState) => {
    if (state.rafId !== null) return;
    const tick = () => {
        advanceVisualizer(state);
        drawVisualizer(surfaceRef.current, state);
        state.rafId = shouldKeepAnimating(state) ? requestAnimationFrame(tick) : null;
    };
    state.rafId = requestAnimationFrame(tick);
};

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
    if (stateRef.current == null) {
        stateRef.current = createVisualizerState();
    }

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

    return <canvas ref={canvasRef} aria-hidden="true" className={cn('block h-full w-full', className)} />;
};
