import { Separator } from '@radix-ui/react-separator';
import clsx from 'clsx';
import type { LucideIcon } from 'lucide-react';
import React from 'react';

interface SectionProps extends React.HTMLAttributes<HTMLDivElement> {
    title: string;
    badge?: React.ReactNode;
    icon?: LucideIcon;
}

export const SettingsUI = {
    Container: ({ children, className, ...props }: React.HTMLAttributes<HTMLDivElement>) => {
        return (
            <div className={clsx('border border-border rounded-md w-full', className)} {...props}>
                {children}
            </div>
        );
    },

    Section: ({ title, badge, icon: Icon, children, className, ...props }: SectionProps) => {
        return (
            <div className={clsx('border border-border rounded-md w-full', className)} {...props}>
                <div className="flex items-center gap-2 px-4 py-4 border-b border-border bg-muted/30">
                    {Icon && <Icon className="w-5 h-5 text-sky-400" />}
                    <span className="font-medium text-base text-sky-400">{title}</span>
                    {badge}
                </div>
                <div>{children}</div>
            </div>
        );
    },

    Item: ({ children, className, ...props }: React.HTMLAttributes<HTMLDivElement>) => {
        return (
            <div className={clsx('p-4 justify-between items-center flex flex-row gap-8', className)} {...props}>
                {children}
            </div>
        );
    },

    Description: ({ children, className, ...props }: React.HTMLAttributes<HTMLDivElement>) => {
        return (
            <div className={clsx('w-96 space-y-2', className)} {...props}>
                {children}
            </div>
        );
    },

    Separator: ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => {
        return <Separator className={clsx('border-t border-border', className)} {...props} />;
    },

    BadgeExperimental: ({ label }: { label: string }) => {
        return <span className="text-xs font-medium bg-yellow-300/10 text-yellow-300 px-2 py-0.5 rounded">{label}</span>;
    },
};
