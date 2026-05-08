import clsx from 'clsx';
import { Link } from '@tanstack/react-router';
import type { ComponentProps } from 'react';

type InternalLinkProps = ComponentProps<typeof Link>;

export const InternalLink = ({ children, className, ...props }: InternalLinkProps) => {
    return (
        <Link
            {...props}
            className={clsx('text-sky-400 hover:text-sky-300 underline underline-offset-2 font-bold', className)}
        >
            {children}
        </Link>
    );
};
