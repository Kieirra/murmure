import { LucideIcon, Server } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { type ColorScheme } from '@/features/home/audio-visualizer/audio-pixel/audio-pixel.helpers';
import { ExtensionEnableFooter } from '@/features/extensions/extension-enable-footer/extension-enable-footer';

interface LLMEnableFooterProps {
    icon: LucideIcon;
    label: string;
    needsSetup: boolean;
    onEnable: () => void;
    testId: string;
    colorScheme?: ColorScheme;
}

export const LLMEnableFooter = ({ icon, label, needsSetup, onEnable, testId, colorScheme }: LLMEnableFooterProps) => {
    const { t } = useTranslation();

    return (
        <ExtensionEnableFooter
            icon={icon}
            label={label}
            onEnable={onEnable}
            testId={testId}
            colorScheme={colorScheme}
            hintIcon={Server}
            hint={
                needsSetup
                    ? t('Requires Ollama (free) or your own server. Guided setup in the next step.')
                    : t('Uses the server you already set up.')
            }
        />
    );
};
