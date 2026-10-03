import clsx from 'clsx';

interface CodeBlockProps {
    children: string;
}

export const CodeBlock = ({ children }: CodeBlockProps) => {
    return (
        <pre
            className={clsx(
                'font-mono text-xs text-foreground whitespace-pre-wrap break-all',
                'bg-card/50 border border-border rounded-md p-3',
                'max-h-80 overflow-auto'
            )}
        >
            {children}
        </pre>
    );
};
