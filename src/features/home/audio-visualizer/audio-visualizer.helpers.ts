import type { RefObject } from 'react';
import type { ColorScheme } from '@/components/lib/color-scheme';

const LEVEL_STEP = 0.05;
const LEVEL_EPSILON = 0.001;
const LEVEL_GAIN = 2.5;
const EDGE_ATTENUATION = 0.6;
const PROCESSING_PHASE_STEP = 0.08;
const FIXED_GAP = 2;
const FILL_CELL_SIZE = 7;
const FILL_GAP = 4;
const GLOW_GAP_RATIO = 0.35;
const GLOW_ALPHA = 0.25;

type Rgb = [number, number, number];

const GRADIENT_PALETTES: Record<ColorScheme, { center: Rgb; mid: Rgb; edge: Rgb }> = {
    standard: { center: [34, 211, 238], mid: [14, 165, 233], edge: [59, 130, 246] },
    llm: { center: [221, 60, 221], mid: [128, 77, 230], edge: [171, 48, 232] },
    command: { center: [249, 200, 6], mid: [249, 148, 6], edge: [233, 86, 12] },
};

export interface VisualizerState {
    level: number;
    displayed: number;
    phase: number;
    isProcessing: boolean;
    rafId: number | null;
}

export interface VisualizerSurface {
    ctx: CanvasRenderingContext2D;
    width: number;
    height: number;
    grid: PixelGrid | null;
    gradient: CanvasGradient;
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

interface GridOptions {
    bars?: number;
    rows: number;
    pixelWidth: number;
    pixelHeight: number;
}

export const createVisualizerState = (): VisualizerState => ({
    level: 0,
    displayed: 0,
    phase: 0,
    isProcessing: false,
    rafId: null,
});

export const advanceVisualizer = (state: VisualizerState) => {
    if (state.isProcessing) {
        state.phase = (state.phase + PROCESSING_PHASE_STEP) % (Math.PI * 2);
        return;
    }
    const diff = state.level - state.displayed;
    state.displayed =
        Math.abs(diff) < LEVEL_EPSILON
            ? state.level
            : state.displayed + Math.sign(diff) * Math.min(Math.abs(diff), LEVEL_STEP);
};

export const shouldKeepAnimating = (state: VisualizerState) => state.isProcessing || state.displayed !== state.level;

export const columnHeight = (state: VisualizerState, index: number, columns: number) => {
    if (state.isProcessing) {
        const sigma = columns / 4; // Width of the wave proportional to bars
        const center = (state.phase / (Math.PI * 2)) * (columns + 4 * sigma) - 2 * sigma;
        const distance = index - center;
        return Math.exp(-(distance * distance) / (2 * sigma * sigma));
    }
    const amplitude = Math.min(1, Math.sqrt(state.displayed) * LEVEL_GAIN);
    const bias = Math.abs((index / (columns - 1)) * 2 - 1);
    return Math.max(0, amplitude * (1 - bias * EDGE_ATTENUATION));
};

export const isRowLit = (row: number, litHalfRows: number, rows: number) => {
    const distanceFromCenter = Math.abs(row - (rows - 1) / 2);
    const minDistance = rows % 2 === 0 ? 0.5 : 0;
    return distanceFromCenter <= Math.max(litHalfRows, minDistance);
};

const rgba = ([red, green, blue]: Rgb, alpha: number) => `rgba(${red}, ${green}, ${blue}, ${alpha})`;

export const gradientStops = (colorScheme: ColorScheme): [number, string][] => {
    const { center, mid, edge } = GRADIENT_PALETTES[colorScheme];
    return [
        [0, rgba(edge, 0)],
        [0.15, rgba(edge, 1)],
        [0.33, rgba(mid, 1)],
        [0.5, rgba(center, 1)],
        [0.67, rgba(mid, 1)],
        [0.85, rgba(edge, 1)],
        [1, rgba(edge, 0)],
    ];
};

export const startAnimation = (surfaceRef: RefObject<VisualizerSurface | null>, state: VisualizerState) => {
    if (state.rafId !== null) return;
    const tick = () => {
        advanceVisualizer(state);
        drawVisualizer(surfaceRef.current, state);
        state.rafId = shouldKeepAnimating(state) ? requestAnimationFrame(tick) : null;
    };
    state.rafId = requestAnimationFrame(tick);
};

export const drawVisualizer = (surface: VisualizerSurface | null, state: VisualizerState) => {
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

export const createPixelGrid = (
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

export const createGradient = (ctx: CanvasRenderingContext2D, width: number, colorScheme: ColorScheme) => {
    const gradient = ctx.createLinearGradient(0, 0, width, 0);
    gradientStops(colorScheme).forEach(([offset, color]) => gradient.addColorStop(offset, color));
    return gradient;
};

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

const toOdd = (count: number) => (count % 2 === 0 ? count - 1 : count);

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
