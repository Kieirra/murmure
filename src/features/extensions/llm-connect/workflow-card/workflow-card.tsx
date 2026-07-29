import { type ReactNode } from 'react';
import type { LucideIcon } from 'lucide-react';

interface WorkflowCardProps {
    icon: LucideIcon;
    title: string;
    benefit?: string;
    steps?: ReactNode[];
}

export const WorkflowCard = ({ icon: Icon, title, benefit, steps }: WorkflowCardProps) => (
    <div className="flex-1 rounded-md border border-border bg-background/50 p-4 space-y-3">
        <div className="flex items-center gap-2">
            <Icon className="w-4 h-4 shrink-0 text-sky-400" />
            <span className="text-base font-semibold text-foreground">{title}</span>
        </div>
        {benefit != null && <p className="text-sm text-muted-foreground">{benefit}</p>}
        {steps != null && (
            <ol className="list-decimal list-outside text-sm text-foreground space-y-3 pl-5 marker:text-sky-400 marker:font-semibold">
                {steps.map((step, i) => (
                    <li key={i} className="pl-1">
                        {step}
                    </li>
                ))}
            </ol>
        )}
    </div>
);
