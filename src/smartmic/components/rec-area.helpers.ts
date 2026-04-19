// Helpers specific to rec-area.tsx. Promote to ../helpers/ if a second consumer appears.
export const formatElapsed = (sec: number): string => {
    const mm = Math.floor(sec / 60);
    const ss = sec % 60;
    return `${mm}:${ss < 10 ? '0' : ''}${ss}`;
};
