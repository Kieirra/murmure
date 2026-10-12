import { LucideIcon, Server } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { type ColorScheme } from '@/components/lib/color-scheme';
import { ExtensionEnableAction } from '@/features/extensions/extension-enable-action/extension-enable-action';

interface LLMEnableActionProps {
    icon: LucideIcon;
    label: string;
    needsSetup: boolean;
    onEnable: () => void;
    testId: string;
    colorScheme?: ColorScheme;
}

export const LLMEnableAction = ({ icon, label, needsSetup, onEnable, testId, colorScheme }: LLMEnableActionProps) => {
    const { t } = useTranslation();

    return (
        <ExtensionEnableAction
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
