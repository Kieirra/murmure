import { useEffect, useRef } from 'react';

interface ClickButtonsProps {
    onLeftClick: () => void;
    onRightClick: () => void;
}

export const ClickButtons = ({ onLeftClick, onRightClick }: ClickButtonsProps) => {
    const leftRef = useRef<HTMLButtonElement>(null);
    const rightRef = useRef<HTMLButtonElement>(null);

    useEffect(() => {
        const leftEl = leftRef.current;
        const rightEl = rightRef.current;

        const handleLeft = (e: TouchEvent) => {
            e.preventDefault();
            onLeftClick();
        };

        const handleRight = (e: TouchEvent) => {
            e.preventDefault();
            onRightClick();
        };

        leftEl?.addEventListener('touchstart', handleLeft, { passive: false });
        rightEl?.addEventListener('touchstart', handleRight, { passive: false });

        return () => {
            leftEl?.removeEventListener('touchstart', handleLeft);
            rightEl?.removeEventListener('touchstart', handleRight);
        };
    }, [onLeftClick, onRightClick]);

    return (
        <div className="flex gap-2 px-2 h-14 shrink-0">
            <button
                ref={leftRef}
                className="flex-[6] border border-[#333] bg-[#181818] text-[#ccc] rounded-lg text-sm font-medium cursor-pointer active:bg-[#2a2a2a] transition-colors duration-150"
                style={{ touchAction: 'manipulation' }}
            >
                CLIC G
            </button>
            <button
                ref={rightRef}
                className="flex-[4] border border-[#333] bg-[#181818] text-[#ccc] rounded-lg text-sm font-medium cursor-pointer active:bg-[#2a2a2a] transition-colors duration-150"
                style={{ touchAction: 'manipulation' }}
            >
                CLIC D
            </button>
        </div>
    );
};
