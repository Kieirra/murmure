import { useTranslation } from '@/i18n';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { Sparkles, Shield, Languages, Brain, Monitor, Cloud, AlertTriangle, Star } from 'lucide-react';
import { Page } from '@/components/page';

interface StepIntroProps {
    onChooseLocal: () => void;
    onChooseRemote: () => void;
}

export const StepIntro = ({ onChooseLocal, onChooseRemote }: StepIntroProps) => {
    const { t } = useTranslation();

    const benefits = [
        {
            icon: Languages,
            title: t('Translation & Adaptation'),
            description: t('Translate or adapt your transcriptions to any style.'),
        },
        {
            icon: Brain,
            title: t('Smart Reformulation'),
            description: t('Reformulate text to be more professional or concise.'),
        },
    ];

    return (
        <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0, x: -20 }}
            className="flex flex-col items-center justify-center space-y-3 max-w-2xl mx-auto text-center pb-4"
        >
            <div className="space-y-3">
                <div className="bg-sky-950 p-3 rounded-full w-fit mx-auto mb-2">
                    <Sparkles className="w-12 h-12 text-sky-400" />
                </div>
                <Typography.MainTitle className="text-3xl">{t('Supercharge your transcriptions')}</Typography.MainTitle>
                <Typography.Paragraph className="text-lg text-muted-foreground">
                    {t('Connect a local LLM to automatically process, correct, and enhance your voice inputs.')}
                </Typography.Paragraph>
            </div>

            <div className="grid grid-cols-2 gap-3 max-w-sm mx-auto text-center">
                {benefits.map((benefit) => (
                    <div
                        key={benefit.title}
                        className="bg-card/30 border border-border p-3 rounded-xl flex flex-col items-center gap-1.5"
                    >
                        <div className="flex items-center justify-center">
                            <benefit.icon className="w-5 h-5 text-sky-400" />
                        </div>
                        <h3 className="text-sm font-semibold text-foreground min-h-[2.5rem] flex items-center text-center">
                            {benefit.title}
                        </h3>
                        <p className="text-xs text-muted-foreground leading-relaxed text-left">{benefit.description}</p>
                    </div>
                ))}
            </div>

            <div className="w-full pt-2">
                <h2 className="text-sm font-bold uppercase tracking-[0.25em] text-foreground mb-8">
                    {t('How do you want to connect your LLM?')}
                </h2>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 w-full">
                    <div className="relative bg-card/30 border-2 border-sky-500/50 p-6 rounded-xl space-y-4 text-left">
                        <div className="absolute -top-3 left-1/2 -translate-x-1/2 text-xs text-foreground border border-border rounded-sm px-2 py-0.5 bg-card flex items-center gap-1 shadow-sm z-10">
                            <Star className="w-3 h-3 text-yellow-400 fill-yellow-400" />
                            {t('Recommended')}
                        </div>
                        <div className="flex items-center gap-3">
                            <Monitor className="w-6 h-6 text-emerald-400" />
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
                        <div className="flex items-center gap-2 text-xs text-amber-500">
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
