import clsx from 'clsx';

interface CodeBlockProps {
    children: string;
    bare?: boolean;
}

export const CodeBlock = ({ children, bare = false }: CodeBlockProps) => {
    return (
        <pre
            className={clsx(
                'font-mono text-xs text-foreground whitespace-pre-wrap break-all',
                !bare && 'bg-field border border-input rounded-md p-3',
                'max-h-80 overflow-auto'
            )}
        >
            {children}
        </pre>
    );
};
