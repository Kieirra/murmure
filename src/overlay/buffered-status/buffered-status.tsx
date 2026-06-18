import clsx from 'clsx';
import { Check } from 'lucide-react';

interface BufferedStatusProps {
    label: string;
    isDone: boolean;
}

export const BufferedStatus = ({ label, isDone }: BufferedStatusProps) => (
    <div
        className={clsx(
            'flex items-center justify-center gap-1.5',
            'h-9 px-3',
            'rounded-lg bg-black',
            'animate-in fade-in duration-200'
        )}
    >
        {isDone && <Check className="h-3 w-3 shrink-0 text-emerald-400" />}
        <span className={clsx('text-[9px] font-medium tracking-wide', isDone ? 'text-emerald-400' : 'text-white')}>
            {label}
        </span>
    </div>
);
