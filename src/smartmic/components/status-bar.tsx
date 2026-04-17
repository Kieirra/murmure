import { useCallback, useState } from 'react';
import { useI18n } from '../i18n/use-i18n';
import { StatusBarMenu } from './status-bar-menu';

interface StatusBarProps {
    connected: boolean;
    statusText: string;
    pcName: string;
}

const clearCachesAndReload = () => {
    const reload = () => {
        location.reload();
    };
    if ('serviceWorker' in navigator && navigator.serviceWorker.controller !== null) {
        caches
            .keys()
            .then((keys) => Promise.all(keys.map((k) => caches.delete(k))))
            .then(reload)
            .catch(reload);
    } else {
        reload();
    }
};

export const StatusBar = ({ connected, statusText, pcName }: StatusBarProps) => {
    const { t } = useI18n();
    const [menuOpen, setMenuOpen] = useState(false);

    const handleToggle = useCallback(() => {
        setMenuOpen((prev) => !prev);
    }, []);

    const handleClose = useCallback(() => {
        setMenuOpen(false);
    }, []);

    return (
        <div className="h-8 flex items-center justify-between px-3 text-xs text-[#888] shrink-0 border-b border-[#222] relative">
            <div className="flex items-center">
                <div
                    className="w-2 h-2 rounded-full mr-2 transition-colors duration-150"
                    style={{ background: connected ? '#22c55e' : '#555' }}
                />
                <span>{statusText}</span>
            </div>
            <div className="flex items-center gap-2">
                <span>{pcName}</span>
                <button
                    type="button"
                    aria-label={t('status.menu.options')}
                    aria-haspopup="menu"
                    aria-expanded={menuOpen}
                    className="h-full w-8 flex items-center justify-center text-[#888] active:text-[#e5e5e5]"
                    onClick={handleToggle}
                >
                    &#8942;
                </button>
            </div>
            {menuOpen && (
                <StatusBarMenu
                    onClose={handleClose}
                    items={[
                        {
                            label: t('status.menu.reload'),
                            onClick: clearCachesAndReload,
                        },
                    ]}
                />
            )}
        </div>
    );
};
