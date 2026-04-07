import { Mic, MousePointer, Wifi } from 'lucide-react';
import { useTranslation } from '@/i18n';
import VirtualMicIllustration from '../virtual-mic-illustration.svg';

export const VirtualMicCta = () => {
    const { t } = useTranslation();

    const benefits = [
        {
            icon: Mic,
            title: t('Wireless Mic'),
            description: t('Your phone becomes a high-quality wireless microphone.'),
        },
        {
            icon: MousePointer,
            title: t('Remote Touchpad'),
            description: t('Control your cursor from your phone. Left and right click included.'),
        },
        {
            icon: Wifi,
            title: t('Local Network'),
            description: t('Works on your Wi-Fi. No cloud, no account needed.'),
        },
    ];

    return (
        <section className="flex flex-col items-center text-center gap-6 py-4">
            <h2 className="text-sm font-bold uppercase tracking-[0.25em] text-foreground">
                {t('Why use Virtual Mic?')}
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

            <img src={VirtualMicIllustration} alt="Virtual Mic" className="w-full max-w-[550px]" />
        </section>
    );
};
