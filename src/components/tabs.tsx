import clsx from 'clsx';
import React, { useId, useRef } from 'react';
import { type LucideIcon } from 'lucide-react';
import { ACTIVE_TAB_CLASS, INACTIVE_TAB_CLASS, TAB_CLASS, TAB_PANEL_CLASS, getAdjacentTabIndex } from './tabs.helpers';

export interface TabItem<T extends string> {
    value: T;
    label: string;
    count?: number;
    icon?: LucideIcon;
}

interface TabsProps<T extends string> {
    value: T;
    onValueChange: (value: T) => void;
    tabs: TabItem<T>[];
    children: React.ReactNode;
    className?: string;
    activeColor?: string;
}

export const Tabs = <T extends string>({
    value,
    onValueChange,
    tabs,
    children,
    className,
    activeColor,
}: TabsProps<T>) => {
    const baseId = useId();
    const tabRefs = useRef<(HTMLButtonElement | null)[]>([]);
    const panelId = `${baseId}-panel`;
    const getTabId = (tabValue: T) => `${baseId}-tab-${tabValue}`;
    const isFirstTabActive = tabs.length > 0 && tabs[0].value === value;

    const handleKeyDown = (event: React.KeyboardEvent<HTMLButtonElement>, index: number) => {
        const nextIndex = getAdjacentTabIndex(index, event.key, tabs.length);
        if (nextIndex == null) return;
        event.preventDefault();
        onValueChange(tabs[nextIndex].value);
        tabRefs.current[nextIndex]?.focus();
    };

    return (
        <div className={className}>
            <div role="tablist" className="flex">
                {tabs.map((tab, index) => {
                    const isActive = tab.value === value;
                    return (
                        <button
                            key={tab.value}
                            ref={(element) => {
                                tabRefs.current[index] = element;
                            }}
                            type="button"
                            role="tab"
                            id={getTabId(tab.value)}
                            aria-selected={isActive}
                            aria-controls={panelId}
                            tabIndex={isActive ? 0 : -1}
                            onClick={() => onValueChange(tab.value)}
                            onKeyDown={(event) => handleKeyDown(event, index)}
                            className={clsx(
                                TAB_CLASS,
                                'cursor-pointer',
                                isActive ? ACTIVE_TAB_CLASS : INACTIVE_TAB_CLASS,
                                isActive && activeColor == null && 'text-sky-400'
                            )}
                            style={isActive ? { color: activeColor } : undefined}
                        >
                            {tab.icon != null && <tab.icon className="w-4 h-4" />}
                            {tab.label}
                            {tab.count != null && (
                                <span
                                    className={clsx(
                                        'min-w-5 rounded-full px-1.5 py-0.5 text-center text-xs leading-none tabular-nums',
                                        isActive ? 'bg-sky-500/15 text-sky-300' : 'bg-muted text-muted-foreground'
                                    )}
                                >
                                    {tab.count}
                                </span>
                            )}
                        </button>
                    );
                })}
            </div>
            <div
                role="tabpanel"
                id={panelId}
                aria-labelledby={getTabId(value)}
                className={clsx(TAB_PANEL_CLASS, isFirstTabActive && 'rounded-tl-none')}
            >
                {children}
            </div>
        </div>
    );
};
