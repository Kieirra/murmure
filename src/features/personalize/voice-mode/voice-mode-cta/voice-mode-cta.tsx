import { Mic, SlidersHorizontal, Zap, Lock } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const VoiceModeCta = () => {
    const { t } = useTranslation();

    const benefits = [
        {
            icon: Mic,
            title: t('True hands-free'),
            description: t('Say your word, we start transcribing instantly. No shortcut needed.'),
        },
        {
            icon: SlidersHorizontal,
            title: t('Your custom words'),
            description: t('Choose specific trigger words that feel natural to you.'),
        },
        {
            icon: Zap,
            title: t('Auto-press Enter'),
            description: t('Can also press Enter for you after transcription.'),
        },
    ];

    return (
        <section data-testid="voice-mode-cta" className="flex flex-col items-center text-center gap-6 py-4">
            <h2 className="text-sm font-bold uppercase tracking-[0.25em] text-foreground">{t('Why enable it?')}</h2>

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

            <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <Lock className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                {t(
                    '100% on-device privacy. Listens locally to detect your voice, but audio is never saved or sent anywhere.'
                )}
            </div>
        </section>
    );
};
