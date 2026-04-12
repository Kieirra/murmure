export type OverlaySize = 'small' | 'medium' | 'large';

export const VISUALIZER_CONFIG: Record<
    OverlaySize,
    { bars: number; pixelWidth: number; pixelHeight: number; className: string }
> = {
    small: { bars: 14, pixelWidth: 2, pixelHeight: 2, className: 'w-20 h-7.5 rounded-sm p-1.5' },
    medium: { bars: 16, pixelWidth: 2, pixelHeight: 2, className: 'w-[120px] h-[36px] rounded-lg py-1 px-2' },
    large: { bars: 24, pixelWidth: 3, pixelHeight: 3, className: 'w-1/2 h-[40px] rounded-lg py-1 px-3' },
};
