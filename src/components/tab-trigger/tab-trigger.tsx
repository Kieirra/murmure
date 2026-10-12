import { type ComponentProps } from 'react';
import { Slot } from '@radix-ui/react-slot';
import clsx from 'clsx';

interface TabTriggerProps extends ComponentProps<'button'> {
    isActive?: boolean;
    isPlaceholder?: boolean;
    asChild?: boolean;
}

export const TabTrigger = ({
    isActive = false,
    isPlaceholder = false,
    asChild = false,
    className,
    ...props
}: TabTriggerProps) => {
    const Comp = asChild ? Slot : 'button';

    return (
        <Comp
            className={clsx(
                'relative -mb-px flex items-center gap-2 rounded-t-md border border-b-0 px-4 py-2 text-sm font-medium',
                'transition-colors outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50',
                isPlaceholder && 'border-dashed border-surface-border',
                !isPlaceholder && isActive && 'z-10 border-surface-border bg-surface-header',
                !isPlaceholder && !isActive && 'border-transparent text-muted-foreground hover:text-foreground',
                className
            )}
            {...props}
        />
    );
};
