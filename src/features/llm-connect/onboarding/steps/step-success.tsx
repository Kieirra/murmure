import { useTranslation } from 'react-i18next';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { Check, ArrowRight } from 'lucide-react';
import { Page } from '@/components/page';

interface StepSuccessProps {
    onComplete: () => void;
}

export const StepSuccess = ({ onComplete }: StepSuccessProps) => {
    const { t } = useTranslation();

    return (
        <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            className="flex flex-col items-center justify-center min-h-[400px] max-w-2xl mx-auto text-center space-y-8"
        >
            <motion.div
                initial={{ scale: 0 }}
                animate={{ scale: 1 }}
                transition={{
                    type: 'spring',
                    stiffness: 260,
                    damping: 20,
                    delay: 0.2,
                }}
                className="w-24 h-24 bg-green-500 rounded-full flex items-center justify-center shadow-lg shadow-green-500/20"
            >
                <Check className="w-12 h-12 text-white stroke-[3]" />
            </motion.div>

            <div className="space-y-4">
                <Typography.MainTitle className="text-3xl">
                    {t('You are all set!')}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-lg text-zinc-400 w-128">
                    {t(
                        'LLM Connect is now configured and ready to use. You can now customize your prompts and settings.'
                    )}
                </Typography.Paragraph>
            </div>

            <Page.PrimaryButton onClick={onComplete}>
                {t('Configure your prompt')}
                <ArrowRight className="w-4 h-4 ml-2" />
            </Page.PrimaryButton>
        </motion.div>
    );
};
