import clsx from 'clsx';
import React from 'react';
import { Button, buttonVariants } from './button';
import { VariantProps } from 'class-variance-authority';

export const Page = {
    Header: ({
        children,
        className,
        ...props
    }: React.HTMLAttributes<HTMLDivElement>) => {
        return (
            <div className={clsx(className)} {...props}>
                {children}
            </div>
        );
    },
    PrimaryButton: ({
        children,
        className,
        ...props
    }: React.ComponentProps<'button'> &
        VariantProps<typeof buttonVariants> & {
            asChild?: boolean;
        }) => {
        return (
            <Button
                variant="outline"
                className={clsx(
                    'bg-gradient-to-r from-sky-800 via-sky-700 to-sky-800 hover:from-sky-500 hover:via-sky-500 hover:to-sky-500 px-8',
                    className
                )}
                size="lg"
                {...props}
            >
                {children}
            </Button>
        );
    },
    SecondaryButton: ({
        children,
        className,
        ...props
    }: React.ComponentProps<'button'> &
        VariantProps<typeof buttonVariants> & {
            asChild?: boolean;
        }) => {
        return (
            <Button variant="outline" className={className} {...props}>
                {children}
            </Button>
        );
    },
};
