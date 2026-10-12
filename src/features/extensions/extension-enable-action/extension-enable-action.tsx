import clsx from 'clsx';
import { type LucideIcon } from 'lucide-react';
import { Page } from '@/components/page';
import { PALETTES, type ColorScheme } from '@/components/lib/color-scheme';

interface ExtensionEnableActionProps {
    icon: LucideIcon;
    label: string;
    onEnable: () => void;
    testId: string;
    hintIcon: LucideIcon;
    hint: string;
    colorScheme?: ColorScheme;
    disabled?: boolean;
}

export const ExtensionEnableAction = ({
    icon: Icon,
    label,
    onEnable,
    testId,
    hintIcon: HintIcon,
    hint,
    colorScheme,
    disabled,
}: ExtensionEnableActionProps) => {
    const accentColor = colorScheme == null ? undefined : PALETTES[colorScheme].accent;

    return (
        <div className="flex flex-col items-center gap-3 py-3 md:py-4">
            <Page.PrimaryButton
                onClick={onEnable}
                disabled={disabled}
                data-testid={testId}
                className={clsx(
                    colorScheme === 'command' &&
                        'from-orange-800 via-orange-700 to-orange-800 hover:from-orange-700 hover:via-orange-700 hover:to-orange-700',
                    colorScheme === 'llm' &&
                        'from-violet-800 via-violet-700 to-violet-800 hover:from-violet-700 hover:via-violet-700 hover:to-violet-700'
                )}
            >
                <Icon className="w-4 h-4" />
                {label}
            </Page.PrimaryButton>
            <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <HintIcon
                    className={clsx('w-3.5 h-3.5 shrink-0', accentColor == null && 'text-sky-400')}
                    style={{ color: accentColor }}
                />
                <span>{hint}</span>
            </div>
        </div>
    );
};
