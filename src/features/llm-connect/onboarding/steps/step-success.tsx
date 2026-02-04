import { useTranslation } from 'react-i18next';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { Check, ArrowRight, Keyboard } from 'lucide-react';
import { Page } from '@/components/page';
import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';

interface StepSuccessProps {
    onComplete: () => void;
}

export const StepSuccess = ({ onComplete }: StepSuccessProps) => {
    const { t } = useTranslation();
    const [llmShortcut, setLlmShortcut] = useState('ctrl+alt+space');

    useEffect(() => {
        invoke<string>('get_llm_record_shortcut')
            .then((shortcut) => {
                if (shortcut?.trim()) {
                    setLlmShortcut(shortcut);
                }
            })
            .catch(() => {
                // Keep default shortcut on error
            });
    }, []);

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
            </div>

            <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.3 }}
                className="w-full max-w-lg bg-emerald-500/10 border border-emerald-500/30 rounded-lg p-6 text-left space-y-4"
            >
                <div className="flex items-center gap-3">
                    <div className="w-10 h-10 bg-emerald-500/20 rounded-full flex items-center justify-center">
                        <Keyboard className="w-5 h-5 text-emerald-400" />
                    </div>
                    <Typography.Paragraph className="text-emerald-300 font-semibold text-base">
                        {t('LLM Connect is ready!')}
                    </Typography.Paragraph>
                </div>
                <Typography.Paragraph className="text-zinc-300 text-sm leading-relaxed">
                    {t(
                        'Use the shortcut'
                    )}{' '}
                    <kbd className="px-2 py-1 bg-zinc-800 border border-zinc-600 rounded text-emerald-400 font-mono text-xs">
                        {llmShortcut}
                    </kbd>{' '}
                    {t(
                        'to record your voice. Your transcription will be processed by the LLM using the prompt configured below.'
                    )}
                </Typography.Paragraph>
                <Typography.Paragraph className="text-zinc-400 text-sm">
                    {t(
                        'You can customize the prompt or create new modes on the next screen.'
                    )}
                </Typography.Paragraph>
            </motion.div>

            <Page.PrimaryButton
                onClick={onComplete}
                data-testid="llm-connect-success-button"
            >
                {t('Configure your prompt')}
                <ArrowRight className="w-4 h-4 ml-2" />
            </Page.PrimaryButton>
        </motion.div>
    );
};
