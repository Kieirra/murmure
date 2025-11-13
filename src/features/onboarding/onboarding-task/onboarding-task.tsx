import { Typography } from '@/components/typography';
import { Checkbox } from '@/components/checkbox';
import clsx from 'clsx';

interface OnboardingTaskProps {
    done: boolean;
    label: string;
    description?: string;
    onToggle?: () => void;
}

export const OnboardingTask = ({
    done,
    label,
    description,
    onToggle,
}: OnboardingTaskProps) => {
    return (
        <li className="flex items-center gap-4 py-1">
            <span
                className={
                    'transition-transform duration-200 ' +
                    (done ? 'scale-100' : 'scale-75 opacity-50')
                }
            >
                <Checkbox
                    checked={done}
                    onCheckedChange={onToggle}
                    className={clsx(
                        'cursor-pointer',
                        'scale-115',
                        'data-[state=checked]:border-sky-400',
                        'data-[state=checked]:bg-sky-400',
                        'data-[state=checked]:text-white'
                    )}
                />
            </span>
            <span
                className={done ? 'text-zinc-300 line-through opacity-30!' : ''}
            >
                {label}
                {description && (
                    <Typography.Paragraph className="text-zinc-400 text-xs italic">
                        {description}
                    </Typography.Paragraph>
                )}
            </span>
        </li>
    );
};
