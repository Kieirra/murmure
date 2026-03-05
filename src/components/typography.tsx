import clsx from 'clsx';

export const Typography = {
    MainTitle: ({
        children,
        className,
        ...props
    }: React.HTMLAttributes<HTMLHeadingElement>) => {
        return (
            <h1
                className={clsx('text-2xl font-semibold mb-4', className)}
                {...props}
            >
                {children}
            </h1>
        );
    },

    Title: ({
        children,
        className,
        ...props
    }: React.HTMLAttributes<HTMLHeadingElement>) => {
        return (
            <h2
                className={clsx('font-medium text-white', className)}
                {...props}
            >
                {children}
            </h2>
        );
    },

    Paragraph: ({
        children,
        className,
        ...props
    }: React.HTMLAttributes<HTMLParagraphElement>) => {
        return (
            <p
                className={clsx(
                    'text-sm text-muted-foreground justify-evenly',
                    className
                )}
                {...props}
            >
                {children}
            </p>
        );
    },
};
