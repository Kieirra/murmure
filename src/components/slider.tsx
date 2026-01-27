import { forwardRef, InputHTMLAttributes } from 'react';
import { cn } from '@/components/lib/utils';

export interface SliderProps
    extends Omit<InputHTMLAttributes<HTMLInputElement>, 'onChange'> {
    value: number;
    min?: number;
    max?: number;
    step?: number;
    onChange?: (value: number) => void;
}

export const Slider = forwardRef<HTMLInputElement, SliderProps>(
    ({ className, value, min = 0, max = 100, step = 1, onChange, ...props }, ref) => {
        const percentage = ((value - min) / (max - min)) * 100;

        return (
            <div className={cn('relative flex items-center w-full', className)}>
                <input
                    type="range"
                    ref={ref}
                    value={value}
                    min={min}
                    max={max}
                    step={step}
                    onChange={(e) => onChange?.(Number(e.target.value))}
                    className={cn(
                        'w-full h-2 rounded-full appearance-none cursor-pointer',
                        'bg-zinc-700',
                        '[&::-webkit-slider-thumb]:appearance-none',
                        '[&::-webkit-slider-thumb]:w-4',
                        '[&::-webkit-slider-thumb]:h-4',
                        '[&::-webkit-slider-thumb]:rounded-full',
                        '[&::-webkit-slider-thumb]:bg-white',
                        '[&::-webkit-slider-thumb]:cursor-pointer',
                        '[&::-webkit-slider-thumb]:transition-transform',
                        '[&::-webkit-slider-thumb]:hover:scale-110',
                        '[&::-webkit-slider-thumb]:shadow-md',
                        '[&::-moz-range-thumb]:w-4',
                        '[&::-moz-range-thumb]:h-4',
                        '[&::-moz-range-thumb]:rounded-full',
                        '[&::-moz-range-thumb]:bg-white',
                        '[&::-moz-range-thumb]:cursor-pointer',
                        '[&::-moz-range-thumb]:border-0',
                        '[&::-moz-range-thumb]:transition-transform',
                        '[&::-moz-range-thumb]:hover:scale-110',
                        'focus:outline-none focus:ring-2 focus:ring-zinc-400 focus:ring-offset-2 focus:ring-offset-zinc-900'
                    )}
                    style={{
                        background: `linear-gradient(to right, #3b82f6 0%, #3b82f6 ${percentage}%, #3f3f46 ${percentage}%, #3f3f46 100%)`,
                    }}
                    {...props}
                />
            </div>
        );
    }
);

Slider.displayName = 'Slider';
