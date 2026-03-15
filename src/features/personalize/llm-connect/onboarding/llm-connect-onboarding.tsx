import { useState } from 'react';
import { AnimatePresence } from 'framer-motion';
import { StepIntro } from './steps/step-intro';
import { StepInstall } from './steps/step-install';
import { StepModel } from './steps/step-model';
import { StepSuccess } from './steps/step-success';
import { StepRemoteConfig } from './steps/step-remote-config';
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
    isInstallOnly?: boolean;
    remoteModels: OllamaModel[];
    testRemoteConnection: (url?: string) => Promise<number>;
    fetchRemoteModels: () => Promise<OllamaModel[]>;
    storeRemoteApiKey: (apiKey: string) => Promise<void>;
}

export const LLMConnectOnboarding = ({
    settings,
    testConnection,
    pullModel,
    updateSettings,
    completeOnboarding,
    initialStep = 0,
    models,
    fetchModels,
    isInstallOnly = false,
    remoteModels,
    testRemoteConnection,
    fetchRemoteModels,
    storeRemoteApiKey,
}: LLMConnectOnboardingProps) => {
    const [step, setStep] = useState(initialStep);
    const [onboardingPath, setOnboardingPath] = useState<'local' | 'remote' | null>(null);

    const nextStep = () => {
        window.scrollTo({ top: 0 });
        setStep((s) => s + 1);
    };

    const handleComplete = async () => {
        await completeOnboarding();
    };

    const handleChooseLocal = () => {
        setOnboardingPath('local');
        nextStep();
    };

    const handleChooseRemote = () => {
        setOnboardingPath('remote');
        nextStep();
    };

    const getSteps = () => {
        const introStep = (
            <StepIntro key="intro" onChooseLocal={handleChooseLocal} onChooseRemote={handleChooseRemote} />
        );

        if (onboardingPath === 'remote') {
            return [
                introStep,
                <StepRemoteConfig
                    key="remote-config"
                    onNext={nextStep}
                    testRemoteConnection={testRemoteConnection}
                    storeRemoteApiKey={storeRemoteApiKey}
                    updateSettings={updateSettings}
                />,
                <StepModel
                    key="model"
                    onNext={nextStep}
                    pullModel={pullModel}
                    updateSettings={updateSettings}
                    settings={settings}
                    models={models}
                    fetchModels={fetchModels}
                    isRemote={true}
                    remoteModels={remoteModels}
                    fetchRemoteModels={fetchRemoteModels}
                />,
                <StepSuccess
                    key="success"
                    onComplete={handleComplete}
                    isRemote={true}
                    remoteUrl={settings.remote_url}
                />,
            ];
        }

        return [
            introStep,
            <StepInstall key="install" onNext={nextStep} testConnection={testConnection} />,
            <StepModel
                key="model"
                onNext={isInstallOnly ? handleComplete : nextStep}
                pullModel={pullModel}
                updateSettings={updateSettings}
                settings={settings}
                models={models}
                fetchModels={fetchModels}
                isInstallOnly={isInstallOnly}
            />,
            <StepSuccess key="success" onComplete={handleComplete} />,
        ];
    };

    const steps = getSteps();

    // For install-only mode, skip progress bar and show only model step
    if (isInstallOnly) {
        return (
            <div className="min-h-[600px] flex flex-col">
                <div className="flex-1 relative">
                    <AnimatePresence mode="wait">{steps[2]}</AnimatePresence>
                </div>
            </div>
        );
    }

    const totalSteps = steps.length - 1;
    const progress = Math.min((step / totalSteps) * 100, 100);

    return (
        <div className="min-h-[600px] flex flex-col">
            <ProgressBar progress={progress} />
            <div className="flex-1 relative">
                <AnimatePresence mode="wait">{steps[step]}</AnimatePresence>
            </div>
        </div>
    );
};
