import { Languages, Layers, Lock, Sparkles } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';
import { LLMRewritePreview } from '../llm-rewrite-preview/llm-rewrite-preview';

interface LLMConnectCtaProps {
    onEnable: () => void;
}

export const LLMConnectCta = ({ onEnable }: LLMConnectCtaProps) => {
    const { t } = useTranslation();

    const chips = [
        {
            icon: Sparkles,
            title: t('Clean text, first time'),
            description: t('Hesitations, repeats and typos are gone.'),
        },
        {
            icon: Languages,
            title: t('Speak one language, write another'),
            description: t('Dictate in your language, get English.'),
        },
        {
            icon: Layers,
            title: t('One prompt per use'),
            description: t('Up to 4 prompts, each with its own shortcut.'),
        },
    ];

    return (
        <section data-testid="llm-connect-cta" className="flex flex-col items-center text-center gap-8 pb-10">
            <LLMRewritePreview />

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 w-full">
                {chips.map((chip) => (
                    <div key={chip.title} className="bg-card/50 border border-border p-5 rounded-xl space-y-3">
                        <div className="flex items-center justify-center">
                            <chip.icon className="w-5 h-5 text-sky-400" />
                        </div>
                        <h3 className="font-semibold text-foreground text-sm">{chip.title}</h3>
                        <p className="text-xs text-muted-foreground leading-relaxed">{chip.description}</p>
                    </div>
                ))}
            </div>

            <div className="flex flex-col items-center gap-3 py-3 md:py-4">
                <Page.PrimaryButton onClick={onEnable} data-testid="llm-connect-cta-enable">
                    <Sparkles className="w-4 h-4" />
                    {t('Enable Prompt Mode')}
                </Page.PrimaryButton>
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    <Lock className="w-3.5 h-3.5 text-sky-400 shrink-0" />
                    <span>{t('Runs on your computer with Ollama, or on your own server.')}</span>
                </div>
            </div>
        </section>
    );
};
