export type ColorScheme = 'standard' | 'llm' | 'command';

const PALETTES: Record<ColorScheme, { center: string; accent: string; mid: string; edge: string }> = {
    standard: {
        center: 'hsl(239, 84%, 67%)',
        accent: 'hsl(199, 89%, 48%)',
        mid: 'hsl(199, 89%, 48%)',
        edge: 'hsl(180, 100%, 50%)',
    },
    llm: {
        center: 'hsl(280, 80%, 55%)',
        accent: 'hsl(260, 75%, 60%)',
        mid: 'hsl(260, 75%, 60%)',
        edge: 'hsl(300, 70%, 55%)',
    },
    command: {
        center: 'hsl(20, 90%, 48%)',
        accent: 'hsl(35, 95%, 50%)',
        mid: 'hsl(35, 95%, 50%)',
        edge: 'hsl(48, 95%, 50%)',
    },
};

export const getPixelColor = (
    distanceFromCenter: number,
    isEdgeColumn: boolean,
    isCenterColumn: boolean,
    hasSound: boolean,
    colorScheme: ColorScheme = 'standard'
) => {
    const palette = PALETTES[colorScheme];
    if (distanceFromCenter <= 2) {
        if ((isEdgeColumn && hasSound) || (isCenterColumn && !hasSound)) {
            return palette.accent;
        }
        return palette.center;
    } else if (distanceFromCenter <= 4) {
        return palette.mid;
    } else {
        return palette.edge;
    }
};
