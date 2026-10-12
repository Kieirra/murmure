import type { ReactNode } from 'react';
import { Link } from '@tanstack/react-router';
import { CheckCircle2, Circle } from 'lucide-react';
import clsx from 'clsx';
import { Typography } from '@/components/typography';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/tooltip';

interface OnboardingTaskProps {
    done: boolean;
    label: string;
    description: string;
    dataTestId: string;
    to?: string;
    children?: ReactNode;
}

const rowClassName =
    'flex min-w-0 items-center gap-2 rounded-sm h-7 outline-none focus-visible:ring-2 focus-visible:ring-ring/50';

export const OnboardingTask = ({ done, label, description, dataTestId, to, children }: OnboardingTaskProps) => {
    const isLink = to !== undefined;
    const content = (
        <>
            {done ? (
                <CheckCircle2 className="size-3.5 shrink-0 text-emerald-400" />
            ) : (
                <Circle className="size-3.5 shrink-0 text-zinc-500" />
            )}
            <span
                className={clsx(
                    'truncate',
                    done && 'text-muted-foreground',
                    !done && !isLink && 'text-foreground',
                    !done && isLink && 'text-sky-400 underline underline-offset-2 group-hover:text-sky-300'
                )}
            >
                {label}
            </span>
            {children}
        </>
    );

    return (
        <li data-testid={dataTestId} data-done={done}>
            <Tooltip>
                <TooltipTrigger asChild>
                    {isLink ? (
                        <Link to={to} className={clsx(rowClassName, 'group')}>
                            {content}
                        </Link>
                    ) : (
                        <div tabIndex={0} className={rowClassName}>
                            {content}
                        </div>
                    )}
                </TooltipTrigger>
                <TooltipContent side="right" sideOffset={8}>
                    <Typography.Paragraph className="text-white text-xs max-w-56">{description}</Typography.Paragraph>
                </TooltipContent>
            </Tooltip>
        </li>
    );
};
