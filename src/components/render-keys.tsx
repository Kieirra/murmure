import { Kbd } from '@/components/kbd';
import clsx from 'clsx';

const KEY_LABELS: Record<string, string> = {
    mousebutton1: 'LMB',
    mousebutton2: 'RMB',
    mousebutton3: 'MMB',
    mousebutton4: 'MB4',
    mousebutton5: 'MB5',
    arrowup: '↑',
    arrowdown: '↓',
    arrowleft: '←',
    arrowright: '→',
    pageup: 'PgUp',
    pagedown: 'PgDn',
    delete: 'Del',
    insert: 'Ins',
    escape: 'Esc',
    backspace: '⌫',
    enter: '↵',
};

interface RenderKeysProps extends React.HTMLAttributes<HTMLSpanElement> {
    keyString: string;
}

export const RenderKeys = ({
    keyString,
    className,
    ...props
}: RenderKeysProps) => {
    const keys = keyString.split('+');
    return (
        <span
            className={clsx('inline-flex items-center gap-0.5', className)}
            {...props}
        >
            {keys.map((key, i) => {
                const displayKey = KEY_LABELS[key.toLowerCase()] || key;
                return (
                    <span
                        key={key}
                        className="inline-flex items-center gap-0.5"
                    >
                        <Kbd>{displayKey}</Kbd>
                        {i < keys.length - 1 && (
                            <span className="text-zinc-500">+</span>
                        )}
                    </span>
                );
            })}
        </span>
    );
};
