import { type CSSProperties, type ReactNode } from 'react';
import clsx from 'clsx';

interface PreviewCellProps {
    label: string;
    labelStyle?: CSSProperties;
    centered?: boolean;
    children?: ReactNode;
}

export const PreviewCell = ({ label, labelStyle, centered = false, children }: PreviewCellProps) => (
    <div className="flex flex-col gap-1.5 rounded-lg border border-neutral-800 bg-black px-4 py-3">
        <p
            className={clsx('text-[11px] uppercase tracking-wider', labelStyle == null && 'text-muted-foreground')}
            style={labelStyle}
        >
            {label}
        </p>
        {centered ? <div className="flex flex-1 items-center">{children}</div> : children}
    </div>
);
