import { useCallback, useRef } from 'react';

interface StatusBarProps {
    connected: boolean;
    statusText: string;
    pcName: string;
}

export const StatusBar = ({ connected, statusText, pcName }: StatusBarProps) => {
    const lastTap = useRef(0);

    const handleTap = useCallback(() => {
        const now = Date.now();
        if (now - lastTap.current < 300) {
            if ('serviceWorker' in navigator && navigator.serviceWorker.controller) {
                caches.keys().then((keys) => Promise.all(keys.map((k) => caches.delete(k)))).then(() => {
                    location.reload();
                });
            } else {
                location.reload();
            }
        }
        lastTap.current = now;
    }, []);

    return (
        <div
            role="button"
            tabIndex={0}
            className="h-8 flex items-center justify-between px-3 text-xs text-[#888] shrink-0 border-b border-[#222]"
            onClick={handleTap}
            onKeyDown={(e) => { if (e.key === 'Enter') handleTap(); }}
        >
            <div className="flex items-center">
                <div
                    className="w-2 h-2 rounded-full mr-2 transition-colors duration-150"
                    style={{ background: connected ? '#22c55e' : '#555' }}
                />
                <span>{statusText}</span>
            </div>
            <span>{pcName}</span>
        </div>
    );
};
