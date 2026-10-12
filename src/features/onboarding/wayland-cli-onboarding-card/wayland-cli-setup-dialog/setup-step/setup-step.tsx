import { ReactNode } from 'react';

interface SetupStepProps {
    number: number;
    children: ReactNode;
}

export const SetupStep = ({ number, children }: SetupStepProps) => (
    <li className="flex items-start gap-3">
        <span className="size-6 bg-sky-500/15 rounded-full flex items-center justify-center flex-shrink-0 text-xs font-semibold text-sky-400">
            {number}
        </span>
        <div className="space-y-2 flex-1 min-w-0 pt-0.5">{children}</div>
    </li>
);
