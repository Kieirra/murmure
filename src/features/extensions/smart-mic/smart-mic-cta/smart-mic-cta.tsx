import { Languages, Lock, Mic, Smartphone } from 'lucide-react';
import { Page } from '@/components/page';
import { useTranslation } from '@/i18n';

interface SmartMicCtaProps {
    onEnable: () => void;
}

export const SmartMicCta = ({ onEnable }: SmartMicCtaProps) => {
    const { t } = useTranslation();

    return (
        <section
            data-testid="smart-mic-cta"
            className="flex flex-col items-center text-center gap-8 rounded-xl border border-border bg-card/30 px-6 py-10"
        >
            <Smartphone className="w-12 h-12 text-sky-400" />

            <h2 className="text-sm font-bold uppercase tracking-[0.25em] text-foreground">
                {t('Two ways to use your phone with Murmure')}
            </h2>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 w-full">
                <div className="bg-card/50 border border-border p-5 rounded-xl space-y-3 text-left">
                    <div className="flex items-center gap-3">
                        <Mic className="w-5 h-5 text-emerald-400 shrink-0" />
                        <h3 className="font-semibold text-foreground text-sm">
                            {t('A new device')}
                        </h3>
                    </div>
                    <p className="text-sm text-muted-foreground leading-relaxed">
                        {t(
                            'No mic on your laptop? Working on a second screen? Your phone becomes a wireless mic, trackpad, and keyboard.'
                        )}
                    </p>
                </div>

                <div className="bg-card/50 border border-border p-5 rounded-xl space-y-3 text-left">
                    <div className="flex items-center gap-3">
                        <Languages className="w-5 h-5 text-emerald-400 shrink-0" />
                        <h3 className="font-semibold text-foreground text-sm">
                            {t('Live translation')}
                        </h3>
                    </div>
                    <p className="text-sm text-muted-foreground leading-relaxed">
                        {t(
                            'You speak one language, they speak another. Murmure translates both sides.'
                        )}
                    </p>
                </div>
            </div>

            <div className="py-3 md:py-4">
                <Page.PrimaryButton onClick={onEnable} data-testid="smart-mic-cta-enable">
                    <Smartphone className="w-4 h-4" />
                    {t('Enable Smart Mic')}
                </Page.PrimaryButton>
            </div>

            <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <Lock className="w-3.5 h-3.5 text-sky-400 shrink-0" />
                <span>{t('On local WiFi only.')}</span>
            </div>
        </section>
    );
};
