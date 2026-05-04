import clsx from 'clsx';

const MAX_CHARS = 10;
const truncate = (s: string) => (s.length > MAX_CHARS ? s.slice(0, MAX_CHARS) : s);

interface ModeFlashProps {
    text: string;
    isFadingOut: boolean;
}

export const ModeFlash = ({ text, isFadingOut }: ModeFlashProps) => (
    <div
        className={clsx(
            'w-[100px] text-center rounded-sm bg-black py-1 transition-opacity duration-200',
            isFadingOut ? 'opacity-0' : 'opacity-100'
        )}
    >
        <span className="text-[10px] tracking-wider font-normal text-white">{truncate(text)}</span>
    </div>
);
