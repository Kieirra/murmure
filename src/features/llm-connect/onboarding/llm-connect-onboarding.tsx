import { useState } from 'react';
import { AnimatePresence, motion } from 'framer-motion';
import { StepIntro } from './steps/step-intro';
import { StepInstall } from './steps/step-install';
import { StepModel } from './steps/step-model';
import { StepSuccess } from './steps/step-success';
import { LLMConnectSettings } from '../hooks/use-llm-connect';

interface LLMConnectOnboardingProps {
    settings: LLMConnectSettings;
    testConnection: (url?: string) => Promise<boolean>;
    pullModel: (model: string) => Promise<void>;
    updateSettings: (updates: Partial<LLMConnectSettings>) => Promise<void>;
    completeOnboarding: () => Promise<void>;
}

export const LLMConnectOnboarding = ({
    testConnection,
    pullModel,
    updateSettings,
    completeOnboarding,
}: LLMConnectOnboardingProps) => {
    const [step, setStep] = useState(0);

    const nextStep = () => setStep((s) => s + 1);

    const handleComplete = async () => {
        await completeOnboarding();
    };

    const steps = [
        <StepIntro key="intro" onNext={nextStep} />,
        <StepInstall
            key="install"
            onNext={nextStep}
            testConnection={testConnection}
        />,
        <StepModel
            key="model"
            onNext={nextStep}
            pullModel={pullModel}
            updateSettings={updateSettings}
        />,
        <StepSuccess key="success" onComplete={handleComplete} />,
    ];

    // Progress bar calculation (3 steps before success)
    const progress = Math.min((step / 3) * 100, 100);

    return (
        <div className="min-h-[600px] flex flex-col">
            {/* Progress Bar */}
            <div className="w-full h-1 bg-zinc-800 rounded-full mb-8 overflow-hidden">
                <motion.div
                    className="h-full bg-blue-500"
                    initial={{ width: 0 }}
                    animate={{ width: `${progress}%` }}
                    transition={{ duration: 0.5, ease: 'easeInOut' }}
                />
            </div>

            {/* Content */}
            <div className="flex-1 relative">
                <AnimatePresence mode="wait">{steps[step]}</AnimatePresence>
            </div>
        </div>
    );
};
