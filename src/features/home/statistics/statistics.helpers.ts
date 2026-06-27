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

export const TYPING_WPM = 40;

export const formatTimeSaved = (seconds: number) => {
    if (!Number.isFinite(seconds) || seconds <= 0) return '-';
    const totalMinutes = Math.round(seconds / 60);
    if (totalMinutes < 1) return '1 min';
    const hours = Math.floor(totalMinutes / 60);
    const minutes = totalMinutes % 60;
    if (hours === 0) return `${minutes} min`;
    return `${hours}h${minutes.toString().padStart(2, '0')}`;
};

export const computeTypingMultiplier = (wpm: number) => {
    if (!Number.isFinite(wpm) || wpm <= TYPING_WPM) return null;
    const ratio = wpm / TYPING_WPM;
    const shown = Number.isInteger(ratio) ? ratio.toString() : ratio.toFixed(1);
    return `${shown}×`;
};
