const timeFormatter = new Intl.DateTimeFormat('default', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
});

export const formatTimestamp = (ts: number): string => timeFormatter.format(new Date(ts));
