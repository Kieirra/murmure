import { Sparkles } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { LLMRewritePreview } from '../llm-rewrite-preview/llm-rewrite-preview';
import { LLMEnableAction } from '../llm-enable-action/llm-enable-action';

interface LLMConnectCtaProps {
    onEnable: () => void;
    needsSetup: boolean;
}

export const LLMConnectCta = ({ onEnable, needsSetup }: LLMConnectCtaProps) => {
    const { t } = useTranslation();

    return (
        <section data-testid="llm-connect-cta" className="flex flex-col items-center text-center gap-12 pt-6 pb-10">
            <LLMEnableAction
                icon={Sparkles}
                label={needsSetup ? t('Set up Prompt Mode') : t('Enable Prompt Mode')}
                needsSetup={needsSetup}
                onEnable={onEnable}
                testId="llm-connect-cta-enable"
                colorScheme="llm"
            />

            <LLMRewritePreview />
        </section>
    );
};
