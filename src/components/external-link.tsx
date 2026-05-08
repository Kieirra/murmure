import clsx from 'clsx';
import { ExternalLink as ExternalLinkIcon } from 'lucide-react';

interface ExternalLinkProps extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
    withIcon?: boolean;
}

export const ExternalLink = ({ children, withIcon = true, className, ...props }: ExternalLinkProps) => {
    return (
        <a
            {...props}
            target="_blank"
            rel="noopener noreferrer"
            className={clsx(
                'text-sky-400 hover:text-sky-300 underline underline-offset-2 font-bold',
                withIcon && 'inline-flex items-center gap-0.5',
                className
            )}
        >
            {withIcon && <ExternalLinkIcon className="w-4 h-4 px-0.5 translate-y-px" />}
            {children}
        </a>
    );
};
