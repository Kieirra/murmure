import { invoke } from '@tauri-apps/api/core';
import { X } from 'lucide-react';
import clsx from 'clsx';
import type { OverlaySize } from './visualizer-config';

const SIZE: Record<OverlaySize, { button: string; icon: number }> = {
    small: { button: 'w-[18px] h-[18px]', icon: 10 },
    medium: { button: 'w-5 h-5', icon: 12 },
    large: { button: 'w-6 h-6', icon: 14 },
};

export const CancelButton = ({ size }: { size: OverlaySize }) => {
    return (
        <button
            type="button"
            data-interactive
            aria-label="Annuler la dictée"
            title="Annuler la dictée"
            onClick={() => invoke('cancel_recording')}
            className={clsx(
                'absolute',
                '-top-2',
                '-right-2',
                'z-10',
                'cursor-pointer',
                'flex',
                'items-center',
                'justify-center',
                'rounded-full',
                'bg-black',
                'hover:bg-neutral-700',
                'text-neutral-300',
                'hover:text-white',
                'transition-all',
                'duration-150',
                'ease-out',
                'hover:scale-110',
                SIZE[size].button
            )}
        >
            <X size={SIZE[size].icon} strokeWidth={2.5} />
        </button>
    );
};
