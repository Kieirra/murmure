import { useState } from 'react';
import { AnimatePresence } from 'framer-motion';
import { StepIntro } from './steps/step-intro';
import { StepInstall } from './steps/step-install';
import { StepModel } from './steps/step-model';
import { StepSuccess } from './steps/step-success';
import { LLMConnectSettings, OllamaModel } from '../hooks/use-llm-connect';
import { ProgressBar } from '@/components/progress-bar';

interface LLMConnectOnboardingProps {
    settings: LLMConnectSettings;
    testConnection: (url?: string) => Promise<boolean>;
    pullModel: (model: string) => Promise<void>;
    updateSettings: (updates: Partial<LLMConnectSettings>) => Promise<void>;
    completeOnboarding: () => Promise<void>;
    initialStep?: number;
    models: OllamaModel[];
    fetchModels: () => Promise<OllamaModel[]>;
}

export const LLMConnectOnboarding = ({
    testConnection,
    pullModel,
    updateSettings,
    completeOnboarding,
    initialStep = 0,
    models,
    fetchModels,
}: LLMConnectOnboardingProps) => {
    const [step, setStep] = useState(initialStep);

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
            models={models}
            fetchModels={fetchModels}
        />,
        <StepSuccess key="success" onComplete={handleComplete} />,
    ];

    const progress = Math.min((step / 3) * 100, 100);

    return (
        <div className="min-h-[600px] flex flex-col">
            <ProgressBar progress={progress} />
            <div className="flex-1 relative">
                <AnimatePresence mode="wait">{steps[step]}</AnimatePresence>
            </div>
        </div>
    );
};
