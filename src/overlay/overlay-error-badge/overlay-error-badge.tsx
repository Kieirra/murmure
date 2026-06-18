import clsx from 'clsx';
import { TriangleAlert } from 'lucide-react';

interface OverlayErrorBadgeProps {
    message: string;
}

export const OverlayErrorBadge = ({ message }: OverlayErrorBadgeProps) => (
    <div
        className={clsx(
            'flex items-center gap-1.5',
            'h-9 px-2.5',
            'rounded-lg bg-black',
            'animate-in fade-in zoom-in duration-200'
        )}
    >
        <TriangleAlert className="h-3 w-3 shrink-0 text-amber-400" />
        <span className="text-[8px] font-medium truncate text-amber-400">{message}</span>
    </div>
);
