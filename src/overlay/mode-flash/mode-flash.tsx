import clsx from 'clsx';

interface ModeFlashProps {
    text: string;
    isFadingOut: boolean;
}

export const ModeFlash = ({ text, isFadingOut }: ModeFlashProps) => (
    <div
        data-interactive
        className={clsx(
            'w-[100px] text-center rounded-sm bg-black py-1 transition-opacity duration-200',
            isFadingOut ? 'opacity-0' : 'opacity-100'
        )}
    >
        <span className="text-[10px] tracking-wider font-normal text-white">{text.slice(0, 10)}</span>
    </div>
);
