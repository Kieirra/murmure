import { useTranslation } from 'react-i18next';
import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { Sparkles, Shield, Languages, Brain } from 'lucide-react';

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
            className="flex flex-col items-center justify-center space-y-8 max-w-2xl mx-auto text-center py-12"
        >
            <div className="space-y-4">
                <div className="bg-amber-500/10 p-4 rounded-full w-fit mx-auto mb-6">
                    <Sparkles className="w-12 h-12 text-amber-500" />
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

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 w-full text-left">
                {benefits.map((benefit, index) => (
                    <motion.div
                        key={index}
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ delay: 0.1 * (index + 1) }}
                        className="bg-zinc-900/50 border border-zinc-800 p-6 rounded-xl space-y-3"
                    >
                        <benefit.icon className="w-6 h-6 text-blue-400" />
                        <h3 className="font-semibold text-zinc-100">
                            {benefit.title}
                        </h3>
                        <p className="text-sm text-zinc-400 leading-relaxed">
                            {benefit.description}
                        </p>
                    </motion.div>
                ))}
            </div>

            <div className="pt-8">
                <Button size="lg" onClick={onNext} className="px-8">
                    {t('Start Configuration')}
                </Button>
                <p className="mt-4 text-xs text-zinc-500">
                    {t('Requires installing Ollama (free & open source)')}
                </p>
            </div>
        </motion.div>
    );
};
