import { Mic, MousePointer, Wifi } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const SmartMicCta = () => {
    const { t } = useTranslation();

    const benefits = [
        {
            icon: Mic,
            title: t('All-in-one'),
            description: t('Your smartphone becomes mic, mouse, and keyboard. One device to control everything.'),
        },
        {
            icon: MousePointer,
            title: t('Fast and private'),
            description: t('Type from your phone. Nothing goes through the cloud, ever.'),
        },
        {
            icon: Wifi,
            title: t('No borders'),
            description: t('Speak your language, your friends read theirs.'),
        },
    ];

    return (
        <section data-testid="smart-mic-cta" className="flex flex-col items-center text-center gap-6 py-8">
            <h2 className="text-sm font-bold uppercase tracking-[0.25em] text-foreground">
                {t('Why use Smart Mic?')}
            </h2>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-3 w-full">
                {benefits.map((benefit) => (
                    <div key={benefit.title} className="bg-card/30 border border-border p-4 rounded-xl space-y-2">
                        <div className="flex items-center justify-center">
                            <benefit.icon className="w-5 h-5 text-sky-400" />
                        </div>
                        <h3 className="font-semibold text-foreground text-sm">{benefit.title}</h3>
                        <p className="text-sm text-muted-foreground leading-relaxed text-left">
                            {benefit.description}
                        </p>
                    </div>
                ))}
            </div>

        </section>
    );
};
