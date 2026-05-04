import clsx from 'clsx';
import { ModeFlashState } from './mode-flash.helpers';

interface ModeFlashProps {
    flashState: ModeFlashState;
}

export const ModeFlash = ({ flashState }: ModeFlashProps) => (
    <div
        className={clsx(
            'w-fit rounded-sm bg-black px-3 py-1 transition-opacity duration-200',
            flashState.fadingOut ? 'opacity-0' : 'opacity-100'
        )}
    >
        <span className="text-[10px] tracking-wider font-normal text-white">
            {flashState.text}
        </span>
    </div>
);
