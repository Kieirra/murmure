import clsx from 'clsx';
import { LucideIcon, Server } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { PALETTES, type ColorScheme } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';

interface LLMEnableFooterProps {
    icon: LucideIcon;
    label: string;
    needsSetup: boolean;
    onEnable: () => void;
    testId: string;
    colorScheme?: ColorScheme;
}

export const LLMEnableFooter = ({
    icon: Icon,
    label,
    needsSetup,
    onEnable,
    testId,
    colorScheme,
}: LLMEnableFooterProps) => {
    const { t } = useTranslation();
    const accentColor = colorScheme == null ? undefined : PALETTES[colorScheme].accent;

    return (
        <div className="flex flex-col items-center gap-3 py-3 md:py-4">
            <Page.PrimaryButton
                onClick={onEnable}
                data-testid={testId}
                className={clsx(
                    colorScheme === 'command' &&
                        'from-orange-800 via-orange-700 to-orange-800 hover:from-orange-700 hover:via-orange-700 hover:to-orange-700'
                )}
            >
                <Icon className="w-4 h-4" />
                {label}
            </Page.PrimaryButton>
            <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <Server
                    className={clsx('w-3.5 h-3.5 shrink-0', accentColor == null && 'text-sky-400')}
                    style={{ color: accentColor }}
                />
                <span>
                    {needsSetup
                        ? t('Requires Ollama (free) or your own server. Guided setup in the next step.')
                        : t('Uses the server you already set up.')}
                </span>
            </div>
        </div>
    );
};
