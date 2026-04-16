import { Lightbulb, Lock, Mic, Zap } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';

interface VoiceModeCtaProps {
    onEnable: () => void;
}

export const VoiceModeCta = ({ onEnable }: VoiceModeCtaProps) => {
    const { t } = useTranslation();

    const chips = [
        {
            icon: Mic,
            title: t('Do anything else'),
            description: t('Drink coffee, relax, do anything while typing'),
        },
        {
            icon: Zap,
            title: t('No shortcut to remember'),
            description: t('Speak the action, skip the keys'),
        },
        {
            icon: Lock,
            title: t('Stays on device'),
            description: t('Listens only when you speak, never saved.'),
        },
    ];

    return (
        <section
            data-testid="voice-mode-cta"
            className="flex flex-col items-center text-center gap-8 rounded-xl border border-border bg-card/30 px-6 py-10"
        >
            <Mic className="w-12 h-12 text-sky-400" />

            <h2 className="text-sm font-bold uppercase tracking-[0.25em] text-foreground">
                {t('Trigger Murmure with your voice')}
            </h2>

            <p className="max-w-md text-sm text-muted-foreground leading-relaxed">
                {t(
                    'Say "ok alix", talk, then "thank you alix". Typed and sent, hands free.'
                )}
            </p>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 w-full">
                {chips.map((chip) => (
                    <div key={chip.title} className="bg-card/50 border border-border p-5 rounded-xl space-y-3">
                        <div className="flex items-center justify-center">
                            <chip.icon className="w-5 h-5 text-emerald-400" />
                        </div>
                        <h3 className="font-semibold text-foreground text-sm">{chip.title}</h3>
                        <p className="text-xs text-muted-foreground leading-relaxed">{chip.description}</p>
                    </div>
                ))}
            </div>

            <div className="py-3 md:py-4">
                <Page.PrimaryButton
                    onClick={onEnable}
                    data-testid="voice-mode-cta-enable"
                >
                    <Mic className="w-4 h-4" />
                    {t('Enable Voice Mode')}
                </Page.PrimaryButton>
            </div>

            <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <Lightbulb className="w-3.5 h-3.5 text-sky-400 shrink-0" />
                <span>{t('Works best in a quiet environment.')}</span>
            </div>
        </section>
    );
};
