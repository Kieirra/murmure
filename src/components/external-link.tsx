import clsx from 'clsx';

export const ExternalLink = ({ children, ...props }: React.AnchorHTMLAttributes<HTMLAnchorElement>) => {
    return (
        <a
            {...props}
            target="_blank"
            rel="noopener noreferrer"
            className={clsx('text-sky-400 hover:text-sky-300', props.className)}
        >
            {children}
        </a>
    );
};
