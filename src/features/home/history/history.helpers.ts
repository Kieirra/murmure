export const formatTime = (timestamp: number, language: string, justNowLabel: string) => {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const formatter = new Intl.RelativeTimeFormat(language, { style: 'short' });

    if (diffMins < 1) return justNowLabel;
    if (diffMins < 60) return formatter.format(-diffMins, 'minute');
    const diffHours = Math.floor(diffMins / 60);
    if (diffHours < 24) return formatter.format(-diffHours, 'hour');
    const diffDays = Math.floor(diffHours / 24);
    return formatter.format(-diffDays, 'day');
};
