import { Mic, SlidersHorizontal, Zap } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const VoiceModeCta = () => {
    const { t } = useTranslation();

    const benefits = [
        {
            icon: Mic,
            title: t('Hands free'),
            description: t('You dictate, your hands stay on your work. No need to touch the keyboard.'),
        },
        {
            icon: SlidersHorizontal,
            title: t('Your words, your rules'),
            description: t('Create your own trigger words to start, stop, or validate dictation.'),
        },
        {
            icon: Zap,
            title: t('Continuous flow'),
            description: t("You speak, it's sent. No Enter key between you and your text."),
        },
    ];

    return (
        <section data-testid="voice-mode-cta" className="flex flex-col items-center text-center gap-6 py-8">
            <h2 className="text-sm font-bold uppercase tracking-[0.25em] text-foreground">
                {t('Why use Voice Mode?')}
            </h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 w-full">
                {benefits.map((benefit) => (
                    <div key={benefit.title} className="bg-card/30 border border-border p-5 rounded-xl space-y-3">
                        <div className="flex items-center justify-center">
                            <benefit.icon className="w-6 h-6 text-sky-400" />
                        </div>
                        <h3 className="font-semibold text-foreground text-sm">{benefit.title}</h3>
                        <p className="text-sm text-muted-foreground leading-relaxed text-left">{benefit.description}</p>
                    </div>
                ))}
            </div>
        </section>
    );
};
