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
    // F13-F20
    f13: 'F13',
    f14: 'F14',
    f15: 'F15',
    f16: 'F16',
    f17: 'F17',
    f18: 'F18',
    f19: 'F19',
    f20: 'F20',
    // Numpad
    kp0: 'Num 0',
    kp1: 'Num 1',
    kp2: 'Num 2',
    kp3: 'Num 3',
    kp4: 'Num 4',
    kp5: 'Num 5',
    kp6: 'Num 6',
    kp7: 'Num 7',
    kp8: 'Num 8',
    kp9: 'Num 9',
    kpplus: 'Num +',
    kpminus: 'Num -',
    kpmultiply: 'Num *',
    kpdivide: 'Num /',
    // Special keys
    backquote: '²',
    intlbackslash: '<>',
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
                            <span className="text-muted-foreground">+</span>
                        )}
                    </span>
                );
            })}
        </span>
    );
};
