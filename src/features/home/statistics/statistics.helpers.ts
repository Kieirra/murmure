export const formatWords = (value: string) => {
    if (value === '-' || value == null) return '-';
    const n = Number(value);
    if (!Number.isFinite(n) || n <= 0) return '-';
    if (n >= 1000) return `${(n / 1000).toFixed(1)}K`;
    return `${Math.round(n)}`;
};

export const formatData = (value: string) => {
    if (value === '-' || value == null) return '-';
    const n = Number(value);
    if (!Number.isFinite(n) || n <= 0) return '-';
    let v = n;
    let unit = 'MB';
    if (v >= 1000) {
        v = v / 1000;
        unit = 'GB';
    }
    if (v >= 1000) {
        v = v / 1000;
        unit = 'To';
    }
    const shown = v >= 100 ? v.toFixed(0) : v.toFixed(1);
    return `${shown} ${unit}`;
};
