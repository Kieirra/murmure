export const TAB_CLASS =
    'relative -mb-px flex items-center gap-2 rounded-t-md border border-b-0 px-4 py-2 text-sm font-medium transition-colors outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50';

export const ACTIVE_TAB_CLASS = 'z-10 border-surface-border bg-surface-header';

export const INACTIVE_TAB_CLASS = 'border-transparent text-muted-foreground hover:text-foreground';

export const TAB_PANEL_CLASS = 'rounded-md border border-surface-border bg-surface-header p-4';

export const getAdjacentTabIndex = (currentIndex: number, key: string, tabCount: number): number | undefined => {
    switch (key) {
        case 'ArrowRight':
            return (currentIndex + 1) % tabCount;
        case 'ArrowLeft':
            return (currentIndex - 1 + tabCount) % tabCount;
        default:
            return undefined;
    }
};
