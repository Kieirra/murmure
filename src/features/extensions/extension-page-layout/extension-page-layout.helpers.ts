import { PALETTES, type ColorScheme } from '@/components/lib/color-scheme';

const ACTIVE_HALO_INTENSITY = 12;
const INACTIVE_HALO_INTENSITY = 20;

export const buildExtensionHaloGradient = (colorScheme: ColorScheme, isActive: boolean) => {
    const intensity = isActive ? ACTIVE_HALO_INTENSITY : INACTIVE_HALO_INTENSITY;
    return `radial-gradient(ellipse 50% 100% at 50% 0%, color-mix(in srgb, ${PALETTES[colorScheme].center} ${intensity}%, transparent), transparent)`;
};
