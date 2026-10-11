import type { ColorScheme } from './audio-pixel/audio-pixel.helpers';

const LEVEL_STEP = 0.05;
const LEVEL_EPSILON = 0.001;
const LEVEL_GAIN = 2.5;
const EDGE_ATTENUATION = 0.6;
const PROCESSING_PHASE_STEP = 0.08;

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
