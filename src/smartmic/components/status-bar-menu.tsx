import { useEffect, useRef } from 'react';

export interface StatusBarMenuItem {
    label: string;
    onClick: () => void;
}

interface StatusBarMenuProps {
    items: StatusBarMenuItem[];
    onClose: () => void;
}

export const StatusBarMenu = ({ items, onClose }: StatusBarMenuProps) => {
    const menuRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent | TouchEvent) => {
            if (menuRef.current !== null && !menuRef.current.contains(event.target as Node)) {
                onClose();
            }
        };
        document.addEventListener('mousedown', handleClickOutside);
        document.addEventListener('touchstart', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
            document.removeEventListener('touchstart', handleClickOutside);
        };
    }, [onClose]);

    return (
        <div
            ref={menuRef}
            role="menu"
            className="absolute top-full right-0 mt-1 bg-[#111] border border-[#333] rounded-md z-30 min-w-[140px] py-1 shadow-lg"
        >
            {items.map((item) => (
                <button
                    key={item.label}
                    type="button"
                    role="menuitem"
                    className="block w-full text-left px-3 py-2 text-sm text-[#e5e5e5] active:bg-[#222]"
                    onClick={() => {
                        item.onClick();
                        onClose();
                    }}
                >
                    {item.label}
                </button>
            ))}
        </div>
    );
};
