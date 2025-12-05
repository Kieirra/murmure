import clsx from 'clsx';

export const StepItem = ({
    step,
    title,
    description,
    children,
    isActive = false,
}: {
    step: number;
    title: string;
    description: string;
    children: React.ReactNode;
    isActive?: boolean;
}) => (
    <div className="flex gap-4">
        <div
            className={clsx(
                'flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center font-bold transition-colors',
                isActive
                    ? 'bg-sky-500/20 text-sky-400'
                    : 'bg-zinc-800 text-zinc-400'
            )}
        >
            {step}
        </div>
        <div className="space-y-3 flex-1">
            <h3 className="font-semibold text-lg">{title}</h3>
            <p className="text-sm text-zinc-400">{description}</p>
            {children}
        </div>
    </div>
);
