import { useCallback, useRef } from 'react';
import { CornerDownLeft, Delete } from 'lucide-react';

interface EnterButtonProps {
    onPress: () => void;
    onBackspace: () => void;
}

export const EnterButton = ({ onPress, onBackspace }: EnterButtonProps) => {
    const onPressRef = useRef(onPress);
    onPressRef.current = onPress;
    const onBackspaceRef = useRef(onBackspace);
    onBackspaceRef.current = onBackspace;

    const handleEnter = useCallback((e: React.TouchEvent) => {
        e.preventDefault();
        e.stopPropagation();
        onPressRef.current();
    }, []);

    const handleBackspace = useCallback((e: React.TouchEvent) => {
        e.preventDefault();
        e.stopPropagation();
        onBackspaceRef.current();
    }, []);

    return (
        <div className="flex gap-2 px-2 h-14 shrink-0">
            <button
                onTouchStart={handleBackspace}
                className="flex-[4] border border-[#333] bg-[#181818] text-[#ccc] rounded-lg text-sm font-medium cursor-pointer active:bg-[#2a2a2a] transition-colors duration-150 flex items-center justify-center gap-2"
                style={{ touchAction: 'manipulation' }}
            >
                <Delete size={16} />
            </button>
            <button
                onTouchStart={handleEnter}
                className="flex-[6] border border-[#333] bg-[#181818] text-[#ccc] rounded-lg text-sm font-medium cursor-pointer active:bg-[#2a2a2a] transition-colors duration-150 flex items-center justify-center gap-2"
                style={{ touchAction: 'manipulation' }}
            >
                <CornerDownLeft size={16} />
                Entree
            </button>
        </div>
    );
};
