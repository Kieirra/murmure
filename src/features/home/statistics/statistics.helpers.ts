export const formatWords = (value: string) => {
    if (value === '-' || value == null) return '-';
    const n = Number(value);
    if (!Number.isFinite(n) || n <= 0) return '-';
    if (n >= 1000) return `${(n / 1000).toFixed(1)}K`;
    return `${Math.round(n)}`;
};

export type DataUnit = 'MB' | 'GB' | 'TB';

export const formatData = (megabytes: number, language: string, unitLabels: Record<DataUnit, string>) => {
    if (!Number.isFinite(megabytes) || megabytes <= 0) return '-';
    let v = megabytes;
    let unit: DataUnit = 'MB';
    if (v >= 1000) {
        v = v / 1000;
        unit = 'GB';
    }
    if (v >= 1000) {
        v = v / 1000;
        unit = 'TB';
    }
    const fractionDigits = v >= 100 ? 0 : 1;
    const shown = new Intl.NumberFormat(language, {
        minimumFractionDigits: fractionDigits,
        maximumFractionDigits: fractionDigits,
    }).format(v);
    return `${shown} ${unitLabels[unit]}`;
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
