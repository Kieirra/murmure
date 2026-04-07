import { useEffect, useRef } from 'react';
import type { Mode } from '../types';

interface RecAreaProps {
    isRecording: boolean;
    currentMode: Mode;
    modes: Mode[];
    micLevel: number;
    onToggleRec: () => void;
    onModeChange: (direction: 'prev' | 'next') => void;
}

export const RecArea = ({
    isRecording,
    currentMode,
    modes: _modes,
    micLevel,
    onToggleRec,
    onModeChange,
}: RecAreaProps) => {
    const recBtnRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const el = recBtnRef.current;
        if (!el) return;

        const handleTouch = (e: TouchEvent) => {
            e.preventDefault();
            onToggleRec();
        };

        el.addEventListener('touchstart', handleTouch, { passive: false });
        return () => {
            el.removeEventListener('touchstart', handleTouch);
        };
    }, [onToggleRec]);

    return (
        <div className="flex items-center p-2 h-[72px] shrink-0">
            <button
                className="w-11 h-14 flex items-center justify-center bg-[#181818] border border-[#333] rounded-lg text-[#888] text-lg cursor-pointer shrink-0 active:bg-[#2a2a2a] transition-colors duration-150 disabled:opacity-40"
                style={{ touchAction: 'manipulation' }}
                disabled={isRecording}
                onClick={() => onModeChange('prev')}
            >
                &#9664;
            </button>
            <div
                ref={recBtnRef}
                className={`flex-1 h-14 mx-1.5 rounded-xl text-base font-semibold flex items-center justify-center gap-2 cursor-pointer relative transition-all duration-150 ${
                    isRecording
                        ? 'bg-[#7f1d1d] border-2 border-[#dc2626] text-[#fca5a5]'
                        : 'bg-[#1a1a2e] border-2 border-[#444] text-[#e5e5e5]'
                }`}
                style={{ touchAction: 'none' }}
            >
                &#9679; REC
                <span
                    className={`text-[13px] absolute bottom-1 right-2.5 ${
                        isRecording ? 'text-[#fca5a5]' : 'text-[#888]'
                    }`}
                >
                    {currentMode.name}
                </span>
                <div
                    className="absolute bottom-0 left-0 h-[3px] bg-[#dc2626] rounded-b-[10px] transition-[width] duration-100"
                    style={{ width: `${Math.round(micLevel * 100)}%` }}
                />
            </div>
            <button
                className="w-11 h-14 flex items-center justify-center bg-[#181818] border border-[#333] rounded-lg text-[#888] text-lg cursor-pointer shrink-0 active:bg-[#2a2a2a] transition-colors duration-150 disabled:opacity-40"
                style={{ touchAction: 'manipulation' }}
                disabled={isRecording}
                onClick={() => onModeChange('next')}
            >
                &#9654;
            </button>
        </div>
    );
};
