import { type ComponentProps } from 'react';
import clsx from 'clsx';

interface TabPanelProps extends Omit<ComponentProps<'div'>, 'className'> {
    isFirstTabActive: boolean;
}

export const TabPanel = ({ isFirstTabActive, ...props }: TabPanelProps) => (
    <div
        className={clsx(
            'rounded-md border border-surface-border bg-surface-header p-4',
            isFirstTabActive && 'rounded-tl-none'
        )}
        {...props}
    />
);
