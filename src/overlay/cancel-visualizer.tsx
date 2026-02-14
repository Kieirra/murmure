import { useEffect, useRef, useState } from 'react';

interface CancelVisualizerProps {
    bars?: number;
    rows?: number;
    pixelWidth?: number;
    pixelHeight?: number;
}

function getRedPixelColor(distanceFromCenter: number): string {
    if (distanceFromCenter <= 2) {
        return `hsl(0, 90%, 60%)`;   // bright red center
    } else if (distanceFromCenter <= 4) {
        return `hsl(0, 85%, 45%)`;   // mid red
    } else {
        return `hsl(10, 80%, 35%)`;  // dark red edges
    }
}

// Returns true if pixel at (col, row) is on the X diagonals
function isOnX(col: number, row: number, bars: number, rows: number): boolean {
    const colNorm = col / (bars - 1);
    const rowNorm = row / (rows - 1);
    const thickness = 0.18;
    return (
        Math.abs(colNorm - rowNorm) < thickness ||
        Math.abs(colNorm - (1 - rowNorm)) < thickness
    );
}

export const CancelVisualizer = ({
    bars = 14,
    rows = 9,
    pixelWidth = 2,
    pixelHeight = 2,
}: CancelVisualizerProps) => {
    const [phase, setPhase] = useState(0);
    const rafRef = useRef<number | null>(null);

    useEffect(() => {
        let running = true;
        const tick = () => {
            if (!running) return;
            setPhase((p) => (p + 0.12) % (Math.PI * 2));
            rafRef.current = requestAnimationFrame(tick);
        };
        rafRef.current = requestAnimationFrame(tick);
        return () => {
            running = false;
            if (rafRef.current) cancelAnimationFrame(rafRef.current);
        };
    }, []);

    return (
        <div className="flex gap-0.5 w-full">
            {Array.from({ length: bars }).map((_, colIdx) => {
                const centerIndex = (rows - 1) / 2;
                const colKey = `col-${colIdx}`;
                return (
                    <div key={colKey} className="flex flex-col gap-0.5 flex-1">
                        {Array.from({ length: rows }).map((_, rowIdx) => {
                            const lit = isOnX(colIdx, rowIdx, bars, rows);
                            const distanceFromCenter = Math.abs(rowIdx - centerIndex);
                            // Pulse brightness with sine wave, offset per column for shimmer effect
                            const brightness = lit
                                ? 0.6 + 0.4 * Math.sin(phase + colIdx * 0.3)
                                : 0;
                            const baseColor = getRedPixelColor(distanceFromCenter);
                            const color = lit
                                ? baseColor.replace(')', `, ${brightness})`.replace('hsl', 'hsla'))
                                : 'transparent';
                            const pixelKey = `pixel-${colIdx}-${rowIdx}`;
                            return (
                                <div
                                    key={pixelKey}
                                    style={{
                                        width: `${pixelWidth}px`,
                                        height: `${pixelHeight}px`,
                                        backgroundColor: color,
                                    }}
                                />
                            );
                        })}
                    </div>
                );
            })}
        </div>
    );
};
