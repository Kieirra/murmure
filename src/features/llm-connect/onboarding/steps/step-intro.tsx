import { useTranslation } from 'react-i18next';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { Sparkles, Shield, Languages, Brain } from 'lucide-react';
import { Page } from '@/components/page';

interface StepIntroProps {
    onNext: () => void;
}

export const StepIntro = ({ onNext }: StepIntroProps) => {
    const { t } = useTranslation();

    const benefits = [
        {
            icon: Languages,
            title: t('Translation & Adaptation'),
            description: t(
                'Translate your transcriptions or adapt them to a specific style.'
            ),
        },
        {
            icon: Brain,
            title: t('Smart Reformulation'),
            description: t(
                'Reformulate text to be more professional, concise, or creative.'
            ),
        },
        {
            icon: Shield,
            title: t('Private & Local'),
            description: t(
                'All processing happens locally on your device. Your data never leaves your computer.'
            ),
        },
    ];

    return (
        <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            className="flex flex-col items-center justify-center space-y-8 max-w-2xl mx-auto text-center py-8"
        >
            <div className="space-y-4">
                <div className="bg-sky-950 p-4 rounded-full w-fit mx-auto mb-6">
                    <Sparkles className="w-12 h-12 text-sky-400" />
                </div>
                <Typography.MainTitle className="text-3xl">
                    {t('Supercharge your transcriptions')}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-lg text-zinc-400">
                    {t(
                        'Connect a local LLM to automatically process, correct, and enhance your voice inputs.'
                    )}
                </Typography.Paragraph>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 w-full text-center">
                {benefits.map((benefit, index) => (
                    <motion.div
                        key={benefit.title}
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ delay: 0.1 * (index + 1) }}
                        className="bg-zinc-800/30 border border-zinc-800 p-6 rounded-xl space-y-3"
                    >
                        <div className="flex items-center justify-center">
                            <benefit.icon className="w-6 h-6 text-sky-400" />
                        </div>
                        <h3 className="font-semibold text-zinc-100">
                            {benefit.title}
                        </h3>
                        <p className="text-sm text-zinc-400 leading-relaxed text-left">
                            {benefit.description}
                        </p>
                    </motion.div>
                ))}
            </div>

            <div className="pt-8">
                <Page.PrimaryButton
                    onClick={onNext}
                    data-testid="llm-connect-start-button"
                >
                    {t('Start Configuration')}
                </Page.PrimaryButton>
                <p className="mt-4 text-xs text-zinc-500">
                    {t('Requires installing Ollama (free & open source)')}
                </p>
            </div>
        </motion.div>
    );
};
