import { useCallback, useRef } from 'react';
import { CornerDownLeft, Delete } from 'lucide-react';
import { useI18n } from '../i18n/use-i18n';

interface EnterButtonProps {
    onPress: () => void;
    onBackspace: () => void;
}

export const EnterButton = ({ onPress, onBackspace }: EnterButtonProps) => {
    const { t } = useI18n();
    const onPressRef = useRef(onPress);
    onPressRef.current = onPress;
    const onBackspaceRef = useRef(onBackspace);
    onBackspaceRef.current = onBackspace;

    const handleEnter = useCallback((e: React.TouchEvent) => {
        e.preventDefault();
        e.stopPropagation();
        onPressRef.current();
    }, []);

    const repeatTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
    const repeatInterval = useRef<ReturnType<typeof setInterval> | null>(null);

    const clearRepeat = useCallback(() => {
        if (repeatTimer.current !== null) {
            clearTimeout(repeatTimer.current);
            repeatTimer.current = null;
        }
        if (repeatInterval.current !== null) {
            clearInterval(repeatInterval.current);
            repeatInterval.current = null;
        }
    }, []);

    const handleBackspaceStart = useCallback(
        (e: React.TouchEvent) => {
            e.preventDefault();
            e.stopPropagation();
            onBackspaceRef.current();
            clearRepeat();
            repeatTimer.current = setTimeout(() => {
                repeatInterval.current = setInterval(() => {
                    onBackspaceRef.current();
                }, 50);
            }, 400);
        },
        [clearRepeat]
    );

    const handleBackspaceEnd = useCallback(
        (e: React.TouchEvent) => {
            e.preventDefault();
            clearRepeat();
        },
        [clearRepeat]
    );

    return (
        <div className="flex gap-2 px-2 h-14 shrink-0">
            <button
                onTouchStart={handleBackspaceStart}
                onTouchEnd={handleBackspaceEnd}
                onTouchCancel={handleBackspaceEnd}
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
                {t('remote.rec.enter')}
            </button>
        </div>
    );
};
