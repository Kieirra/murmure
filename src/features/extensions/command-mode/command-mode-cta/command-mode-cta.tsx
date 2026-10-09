import { Zap } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { CommandPreview } from '../command-preview/command-preview';
import { LLMEnableFooter } from '@/features/extensions/llm-connect/llm-enable-footer/llm-enable-footer';

interface CommandModeCtaProps {
    onEnable: () => void;
    needsSetup: boolean;
}

export const CommandModeCta = ({ onEnable, needsSetup }: CommandModeCtaProps) => {
    const { t } = useTranslation();

    return (
        <section data-testid="command-mode-cta" className="flex flex-col items-center text-center gap-8 pb-10">
            <div className="flex w-full flex-col gap-8">
                <CommandPreview variant="selection" />
                <CommandPreview variant="question" />
            </div>

            <LLMEnableFooter
                icon={Zap}
                label={needsSetup ? t('Set up Command Mode') : t('Enable Command Mode')}
                needsSetup={needsSetup}
                onEnable={onEnable}
                testId="command-mode-cta-enable"
            />
        </section>
    );
};
