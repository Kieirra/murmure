import * as React from 'react';
import * as SliderPrimitive from '@radix-ui/react-slider';

import { cn } from '@/components/lib/utils';

type SliderProps = React.ComponentPropsWithoutRef<typeof SliderPrimitive.Root> & {
    showValue?: boolean;
};

const Slider = React.forwardRef<
    React.ElementRef<typeof SliderPrimitive.Root>,
    SliderProps
>(({ className, showValue, value, ...props }, ref) => (
    <div className={cn('w-full', className)}>
        <SliderPrimitive.Root
            ref={ref}
            className="relative flex w-full touch-none select-none items-center"
            value={value}
            {...props}
        >
            <SliderPrimitive.Track className="relative h-1.5 w-full grow overflow-hidden rounded-full bg-accent">
                <SliderPrimitive.Range className="absolute h-full bg-sky-500" />
            </SliderPrimitive.Track>
            <SliderPrimitive.Thumb className="block h-4 w-4 rounded-full border border-border bg-zinc-200 shadow transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50" />
        </SliderPrimitive.Root>
        {showValue && value != null && (
            <span className="block text-center text-sm font-medium text-foreground mt-1">
                {value[0]}
            </span>
        )}
    </div>
));
Slider.displayName = SliderPrimitive.Root.displayName;

export { Slider };
