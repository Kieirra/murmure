import { type CSSProperties, type ReactNode } from 'react';
import clsx from 'clsx';

interface PreviewCellProps {
    label: string;
    labelStyle?: CSSProperties;
    dashed?: boolean;
    centered?: boolean;
    children?: ReactNode;
}

export const PreviewCell = ({ label, labelStyle, dashed = false, centered = false, children }: PreviewCellProps) => (
    <div
        className={clsx(
            'flex flex-col gap-1.5 rounded-lg border border-neutral-800 bg-black px-4 py-3',
            dashed && 'border-dashed'
        )}
    >
        <p
            className={clsx('text-[11px] uppercase tracking-wider', labelStyle == null && 'text-muted-foreground')}
            style={labelStyle}
        >
            {label}
        </p>
        {centered ? <div className="flex flex-1 items-center">{children}</div> : children}
    </div>
);
