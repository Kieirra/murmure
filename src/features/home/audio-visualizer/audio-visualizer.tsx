import { useEffect, useMemo, useRef, useState } from 'react';
import { useLevelState } from './hooks/use-level-state';
import { useLLMState } from './hooks/use-llm-state';
import clsx from 'clsx';
import { AudioPixel } from './audio-pixel/audio-pixel';

interface AudioVisualizerProps {
    bars?: number;
    rows?: number;
    audioPixelWidth?: number;
    audioPixelHeight?: number;
    pixelHeight?: number;
    className?: string;
}

export const AudioVisualizer = ({
    bars = 16,
    rows = 20,
    audioPixelWidth = 12,
    audioPixelHeight = 6,
    className,
}: AudioVisualizerProps) => {
    const { level } = useLevelState();
    const { isProcessing } = useLLMState();
    const rafRef = useRef<number | null>(null);
    const displayedRef = useRef(0);
    const [wavePhase, setWavePhase] = useState(0);

    useEffect(() => {
        const tick = () => {
            if (isProcessing) {
                setWavePhase((p) => (p + 0.08) % (Math.PI * 2));
                rafRef.current = requestAnimationFrame(tick);
            } else {
                const current = displayedRef.current;
                const target = level;
                const diff = target - current;
                const step = Math.sign(diff) * Math.min(Math.abs(diff), 0.05);
                displayedRef.current = current + step;
                rafRef.current = requestAnimationFrame(tick);
            }
        };
        rafRef.current = requestAnimationFrame(tick);
        return () => {
            if (rafRef.current) cancelAnimationFrame(rafRef.current);
        };
    }, [level, isProcessing]);

    const heights = useMemo(() => {
        if (isProcessing) {
            const arr: number[] = [];
            const sigma = bars / 4; // Width of the wave proportional to bars
            for (let i = 0; i < bars; i++) {
                const progress = wavePhase / (Math.PI * 2);
                const center = progress * (bars + 4 * sigma) - 2 * sigma;
                const dist = Math.abs(i - center);
                const h = Math.max(
                    0,
                    Math.exp(-Math.pow(dist, 2) / (2 * Math.pow(sigma, 2)))
                );
                arr.push(h);
            }
            return arr;
        }

        const v = Math.min(1, displayedRef.current * 10);
        const arr: number[] = [];
        for (let i = 0; i < bars; i++) {
            const bias = Math.abs((i / (bars - 1)) * 2 - 1);
            const h = Math.max(0, v * (1 - bias * 0.6));
            arr.push(h);
        }
        return arr;
    }, [bars, isProcessing, wavePhase]);

    return (
        <div className={clsx('flex gap-0.5 w-full', className)}>
            {heights.map((h, colIdx) => {
                const halfRows = Math.floor(rows / 2);
                const litHalfRows = Math.floor(h * halfRows);
                const isEdgeColumn = colIdx === 0 || colIdx === bars - 1;
                const centerStart = Math.floor(bars / 2) - 4;
                const centerEnd = Math.floor(bars / 2) + 3;
                const isCenterColumn =
                    colIdx >= centerStart && colIdx <= centerEnd;
                const hasSound = litHalfRows > 1;
                return (
                    <div key={colIdx} className="flex flex-col gap-0.5 flex-1">
                        {Array.from({ length: rows }).map((_, rowIdx) => {
                            const centerIndex = (rows - 1) / 2;
                            const distanceFromCenter = Math.abs(
                                rowIdx - centerIndex
                            );
                            const minDistance = rows % 2 === 0 ? 0.5 : 0;
                            const isLit =
                                distanceFromCenter <=
                                Math.max(litHalfRows, minDistance);
                            return (
                                <AudioPixel
                                    key={rowIdx}
                                    isLit={isLit}
                                    distanceFromCenter={distanceFromCenter}
                                    isEdgeColumn={isEdgeColumn}
                                    isCenterColumn={isCenterColumn}
                                    hasSound={hasSound}
                                    width={audioPixelWidth}
                                    height={audioPixelHeight}
                                />
                            );
                        })}
                    </div>
                );
            })}
        </div>
    );
};
