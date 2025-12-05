import { useTranslation } from 'react-i18next';
import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { Zap, Brain, BicepsFlexed } from 'lucide-react';
import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { Page } from '@/components/page';
import { ModelCard, RecommendedModel } from '@/components/model-card';

interface StepModelProps {
    onNext: () => void;
    pullModel: (model: string) => Promise<void>;
    updateSettings: (settings: { model: string }) => Promise<void>;
}

export const StepModel = ({
    onNext,
    pullModel,
    updateSettings,
}: StepModelProps) => {
    const { t } = useTranslation();
    const [selectedModel, setSelectedModel] = useState<string | null>(null);
    const [downloadingModel, setDownloadingModel] = useState<string | null>(
        null
    );
    const [progress, setProgress] = useState<number>(0);
    const [downloadedModels, setDownloadedModels] = useState<Set<string>>(
        new Set()
    );

    const recommendedModels: RecommendedModel[] = [
        {
            id: 'ministral-3:latest',
            name: 'Ministral 3 (8B)',
            description: t(
                'High-performance model optimized for local use. Excellent reasoning.'
            ),
            size: '6.0 GB',
            icon: BicepsFlexed,
            tags: [t('Smart'), t('European')],
        },
        {
            id: 'qwen3:latest',
            name: 'Qwen 3 (8B)',
            description: t(
                'Versatile and robust. Good at following complex instructions but slower than others.'
            ),
            size: '5.2GB',
            icon: Brain,
            tags: [t('Balanced'), t('Obedient')],
        },
        {
            id: 'gemma3n:latest',
            name: 'Gemma 3n (4B)',
            description: t(
                'Very fast and Resource-efficient. Runs smoothly on older hardware with limited memory.'
            ),
            size: '7.5 GB',
            icon: Zap,
            tags: [t('Fast'), t('Efficient')],
        },
    ];

    const handleCustomModel = async () => {
        await updateSettings({ model: '' });
        onNext();
    };

    useEffect(() => {
        const unlisten = listen<any>('llm-pull-progress', (event) => {
            const { total, completed, status } = event.payload;
            if (status === 'success') {
                setProgress(100);
            } else if (total && completed) {
                setProgress(Math.round((completed / total) * 100));
            }
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    const handleDownload = async (modelId: string) => {
        if (downloadedModels.has(modelId)) {
            setSelectedModel(modelId);
            await updateSettings({ model: modelId });
            return;
        }

        setDownloadingModel(modelId);
        setProgress(0);
        try {
            await pullModel(modelId);
            setDownloadedModels((prev) => new Set(prev).add(modelId));
            setSelectedModel(modelId);
            await updateSettings({ model: modelId });
        } catch (error) {
            console.error('Failed to download model', error);
        } finally {
            setDownloadingModel(null);
            setProgress(0);
        }
    };

    return (
        <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            className="flex flex-col items-center max-w-4xl mx-auto space-y-8 py-8"
        >
            <div className="text-center space-y-4">
                <Typography.MainTitle>
                    {t('Select a Model')}
                </Typography.MainTitle>
                <Typography.Paragraph className="text-zinc-400 max-w-lg mx-auto">
                    {t('Choose a local AI model to power your transcriptions.')}
                </Typography.Paragraph>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 w-full">
                {recommendedModels.map((model) => (
                    <ModelCard
                        key={model.id}
                        model={model}
                        isSelected={selectedModel === model.id}
                        isDownloaded={downloadedModels.has(model.id)}
                        isDownloading={downloadingModel === model.id}
                        progress={progress}
                        onSelect={handleDownload}
                    />
                ))}
            </div>

            <div className="flex justify-center w-full">
                <Button
                    onClick={handleCustomModel}
                    variant="ghost"
                    className="text-zinc-500 hover:text-zinc-300 hover:bg-transparent"
                >
                    {t('I want to choose my own model')}
                </Button>
            </div>

            <div className="flex justify-between w-full pt-4">
                <div />
                <Page.PrimaryButton
                    onClick={onNext}
                    disabled={!selectedModel}
                    size="lg"
                    className="px-8"
                >
                    {t('Finish Setup')}
                </Page.PrimaryButton>
            </div>
        </motion.div>
    );
};
