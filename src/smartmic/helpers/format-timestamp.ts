// Shared helper: lives in `helpers/` because it is consumed by more than one component.
// Rule: component-specific helpers stay next to their component as `.helpers.ts`;
// promote here as soon as a second consumer appears.
const timeFormatter = new Intl.DateTimeFormat('default', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
});

export const formatTimestamp = (ts: number): string => timeFormatter.format(new Date(ts));
