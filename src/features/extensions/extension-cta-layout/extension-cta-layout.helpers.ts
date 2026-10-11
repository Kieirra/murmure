import { PALETTES, type ColorScheme } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';

export const buildExtensionHaloGradient = (colorScheme: ColorScheme, intensity: number) =>
    `radial-gradient(ellipse 50% 100% at 50% 0%, color-mix(in srgb, ${PALETTES[colorScheme].center} ${intensity}%, transparent), transparent)`;
