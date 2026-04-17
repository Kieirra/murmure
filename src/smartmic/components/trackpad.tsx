import { useEffect, useRef, useState } from 'react';
import { useTrackpadGestures } from '../hooks/use-trackpad-gestures';
import { useI18n } from '../i18n/use-i18n';

interface TrackpadProps {
    onMove: (dx: number, dy: number) => void;
    onScroll: (dy: number) => void;
    onTap?: () => void;
    onLongPress?: () => void;
}

export const Trackpad = ({ onMove, onScroll, onTap, onLongPress }: TrackpadProps) => {
    const { t } = useI18n();
    const ref = useRef<HTMLDivElement>(null);
    useTrackpadGestures(ref, { onMove, onScroll, onTap, onLongPress });

    const [hintsVisible, setHintsVisible] = useState(true);

    useEffect(() => {
        const el = ref.current;
        if (el === null || !hintsVisible) return;

        const handleFirstTouch = () => {
            setHintsVisible(false);
        };

        el.addEventListener('touchstart', handleFirstTouch, { once: true, passive: true });
        return () => {
            el.removeEventListener('touchstart', handleFirstTouch);
        };
    }, [hintsVisible]);

    return (
        <div
            ref={ref}
            className="h-[230px] shrink-0 bg-[#111] border border-[#333] rounded-lg m-2 relative"
            style={{ touchAction: 'none' }}
        >
            <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-10 h-10 border border-[#333] rounded-full opacity-30 pointer-events-none" />
            <div
                className={`absolute inset-0 flex flex-col items-center justify-center pointer-events-none transition-opacity duration-200 ${
                    hintsVisible ? 'opacity-100' : 'opacity-0'
                }`}
            >
                <div className="text-xs text-[#444] leading-6 text-center">
                    {t('remote.trackpad.tap')}
                </div>
                <div className="text-xs text-[#444] leading-6 text-center">
                    {t('remote.trackpad.hold')}
                </div>
                <div className="text-xs text-[#444] leading-6 text-center">
                    {t('remote.trackpad.scroll')}
                </div>
            </div>
        </div>
    );
};
