import { useTranslation } from '@/i18n';
import { motion } from 'framer-motion';
import { Sparkles, Shield, Monitor, Cloud, AlertTriangle, Star } from 'lucide-react';
import { Page } from '@/components/page';

interface StepIntroProps {
    onChooseLocal: () => void;
    onChooseRemote: () => void;
}

export const StepIntro = ({ onChooseLocal, onChooseRemote }: StepIntroProps) => {
    const { t } = useTranslation();

    return (
        <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0, x: -20 }}
            className="flex flex-col items-center justify-center space-y-12 max-w-2xl mx-auto text-center pt-8 pb-4"
        >
            <div className="w-full">
                <h2 className="text-sm font-bold uppercase tracking-[0.25em] text-foreground mb-8">
                    {t('How do you want to connect your LLM?')}
                </h2>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 w-full">
                    <div className="relative bg-card/30 border-2 border-cyan-500/50 p-6 rounded-xl space-y-4 text-left">
                        <div className="absolute -top-3 left-1/2 -translate-x-1/2 text-xs text-foreground border border-border rounded-sm px-2 py-0.5 bg-card flex items-center gap-1 shadow-sm z-10">
                            <Star className="w-3 h-3 text-yellow-400 fill-yellow-400" />
                            {t('Recommended')}
                        </div>
                        <div className="flex items-center gap-3">
                            <Monitor className="w-6 h-6 text-cyan-400" />
                            <h3 className="font-semibold text-foreground">{t('Local (Ollama)')}</h3>
                        </div>
                        <p className="text-sm text-muted-foreground leading-relaxed">
                            {t('Run models on your machine. Your data never leaves your computer.')}
                        </p>
                        <div className="flex items-center justify-between text-xs text-muted-foreground">
                            <span className="flex items-center gap-1.5">
                                <Shield className="w-3.5 h-3.5" />
                                {t('100% private')}
                            </span>
                            <span className="flex items-center gap-1.5">
                                <Sparkles className="w-3.5 h-3.5" />
                                {t('100% free')}
                            </span>
                        </div>
                        <Page.PrimaryButton
                            onClick={onChooseLocal}
                            className="w-full"
                            data-testid="llm-connect-choose-local"
                        >
                            {t('Choose Local')}
                        </Page.PrimaryButton>
                        <p className="text-xs text-muted-foreground text-center">
                            {t('Requires installing Ollama (free & open source)')}
                        </p>
                    </div>

                    <div className="bg-card/30 border border-border p-6 rounded-xl flex flex-col gap-4 text-left">
                        <div className="flex items-center gap-3">
                            <Cloud className="w-6 h-6 text-sky-400" />
                            <h3 className="font-semibold text-foreground">{t('Remote Server')}</h3>
                        </div>
                        <p className="text-sm text-muted-foreground leading-relaxed">
                            {t('Connect to an OpenAI-compatible server.')}
                        </p>
                        <div className="flex items-center gap-2 text-xs text-yellow-300/90">
                            <AlertTriangle className="w-5 h-5 flex-shrink-0" />
                            {t('If using a remote server, Murmure cannot guarantee your data privacy.')}
                        </div>
                        <div className="flex-1 flex items-center">
                            <Page.SecondaryButton
                                onClick={onChooseRemote}
                                variant="outline"
                                className="w-full"
                                data-testid="llm-connect-choose-remote"
                            >
                                {t('Choose Remote')}
                            </Page.SecondaryButton>
                        </div>
                    </div>
                </div>
            </div>
        </motion.div>
    );
};
