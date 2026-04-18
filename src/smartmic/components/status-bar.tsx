import { useCallback, useState } from 'react';
import { useI18n } from '../i18n/use-i18n';
import { StatusBarMenu } from './status-bar-menu';
import { clearCachesAndReload } from './status-bar.helpers';

interface StatusBarProps {
    connected: boolean;
    statusText: string;
    pcName: string;
}

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
            <div className="flex items-center min-w-0 flex-1">
                <div
                    className="w-2 h-2 rounded-full mr-2 shrink-0 transition-colors duration-150"
                    style={{ background: connected ? '#22c55e' : '#555' }}
                />
                <span className="truncate">{statusText}</span>
            </div>
            <div className="flex items-center gap-2 shrink-0">
                <span className="max-w-[10rem] truncate" title={pcName}>
                    {pcName}
                </span>
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
