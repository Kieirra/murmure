import clsx from 'clsx';
import { LucideIcon } from 'lucide-react';

interface StatCardProps extends React.ComponentProps<'div'> {
    icon: LucideIcon;
    value: string;
    label: string;
}

export const StatCard = ({ icon: Icon, value, label, className, ...props }: StatCardProps) => {
    return (
        <div
            className={clsx(
                'flex min-w-0 flex-1 cursor-pointer items-center gap-2 whitespace-nowrap rounded-md border border-[#2D2D32] bg-[#1C1C1F] px-3 py-2 text-sm outline-none transition max-lg:gap-1.5 max-lg:px-2 max-lg:text-xs',
                'hover:border-sky-500/40 hover:bg-card hover:shadow-[0_0_24px_-6px] hover:shadow-sky-500/40',
                'focus-visible:border-sky-500/40 focus-visible:bg-card focus-visible:shadow-[0_0_24px_-6px] focus-visible:shadow-sky-500/40',
                className
            )}
            {...props}
        >
            <Icon className="size-4 shrink-0 text-sky-400" />
            <span className="shrink-0 font-bold text-foreground">{value}</span>
            <span className="min-w-0 truncate text-muted-foreground">{label}</span>
        </div>
    );
};
