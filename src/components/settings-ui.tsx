import { Separator } from '@radix-ui/react-separator';
import clsx from 'clsx';
import React from 'react';

export const SettingsUI = {
    Container: ({
        children,
        className,
        ...props
    }: React.HTMLAttributes<HTMLDivElement>) => {
        return (
            <div
                className={clsx(
                    'border border-border rounded-md w-full',
                    className
                )}
                {...props}
            >
                {children}
            </div>
        );
    },

    Item: ({
        children,
        className,
        ...props
    }: React.HTMLAttributes<HTMLDivElement>) => {
        return (
            <div
                className={clsx(
                    'p-4 justify-between items-center flex flex-row gap-8',
                    className
                )}
                {...props}
            >
                {children}
            </div>
        );
    },

    Description: ({
        children,
        className,
        ...props
    }: React.HTMLAttributes<HTMLDivElement>) => {
        return (
            <div className={clsx('w-96 space-y-2', className)} {...props}>
                {children}
            </div>
        );
    },

    Separator: ({
        className,
        ...props
    }: React.HTMLAttributes<HTMLDivElement>) => {
        return (
            <Separator
                className={clsx('border-t border-border', className)}
                {...props}
            />
        );
    },
};
