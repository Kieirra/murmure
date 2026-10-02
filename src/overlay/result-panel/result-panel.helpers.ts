import { PALETTES, type ColorScheme } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';
import type { ResultPanelModeName } from './hooks/use-result-panel';

export const MIN_RESULT_PANEL_SECS = 2;
export const MAX_RESULT_PANEL_SECS = 15;

export const RESULT_PANEL_SEGMENT_MASK = 'repeating-linear-gradient(to right, #000 0 10px, transparent 10px 12px)';

const COLOR_SCHEME_BY_MODE: Record<ResultPanelModeName, ColorScheme> = {
    standard: 'standard',
    llm: 'llm',
    transform: 'llm',
    command: 'command',
};

export const clampResultPanelDurationSecs = (secs: number) =>
    Math.min(MAX_RESULT_PANEL_SECS, Math.max(MIN_RESULT_PANEL_SECS, secs));

export const getResultPanelPalette = (mode: ResultPanelModeName) => PALETTES[COLOR_SCHEME_BY_MODE[mode]];

export const buildResultPanelGradient = (mode: ResultPanelModeName) => {
    const { edge, mid, center } = getResultPanelPalette(mode);
    return `linear-gradient(to right, ${edge} 0%, ${mid} 25%, ${center} 50%, ${mid} 75%, ${edge} 100%)`;
};
