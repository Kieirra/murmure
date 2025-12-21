import { useTranslation } from 'react-i18next';
import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { Mistral, Qwen } from '@lobehub/icons';
import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { Page } from '@/components/page';
import { ModelCard, RecommendedModel } from '@/components/model-card';
import { AlertCircle } from 'lucide-react';

import { OllamaModel } from '../../hooks/use-llm-connect';

interface StepModelProps {
    onNext: () => void;
    pullModel: (model: string) => Promise<void>;
    updateSettings: (settings: { model: string }) => Promise<void>;
    models: OllamaModel[];
    fetchModels: () => Promise<OllamaModel[]>;
}

interface OllamaPullProgressPayload {
    status: string;
    digest?: string;
    total?: number;
    completed?: number;
}

export const StepModel = ({
    onNext,
    pullModel,
    updateSettings,
    models,
    fetchModels,
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
    const [error, setError] = useState<string | null>(null);

    const recommendedModels: RecommendedModel[] = [
        {
            id: 'ministral-3:latest',
            name: 'Ministral 3 (8B)',
            description: t('Great raw reasoning power'),
            bullets: [
                t('Strong analytical abilities'),
                t('Less strict with instructions'),
            ],
            size: t('~ 6 GB on disk'),
            ram: t('7 GB RAM recommended'),
            icon: Mistral.Color,
            tags: [t('Powerful'), t('Reasoning')],
            recommended: true,
        },
        {
            id: 'qwen3:latest',
            name: 'Qwen 3 (8B)',
            description: t('Best adherence to instructions'),
            bullets: [
                t('Highly reliable formatting'),
                t('Follows directives precisely'),
            ],
            size: t('~ 5.2 GB on disk'),
            ram: t('6 GB RAM recommended'),
            icon: Qwen.Color,
            tags: [t('Balanced'), t('Obedient')],
        },
        {
            id: 'qwen3:4b',
            name: 'Qwen 3 (4B)',
            description: t('Optimized for low-end hardware'),
            bullets: [
                t('Low resource usage'),
                t('Less capable still reliable'),
            ],
            size: t('~ 2.5 GB on disk'),
            ram: t('3 GB RAM recommended'),
            icon: Qwen.Color,
            tags: [t('Lightweight'), t('Efficient')],
        },
    ];

    const handleCustomModel = async () => {
        await updateSettings({ model: '' });
        onNext();
    };

    useEffect(() => {
        const unlisten = listen<OllamaPullProgressPayload>(
            'llm-pull-progress',
            (event) => {
                const { total, completed, status } = event.payload;
                if (status === 'success') {
                    setProgress(100);
                } else if (total && completed) {
                    setProgress(Math.round((completed / total) * 100));
                }
            }
        );

        fetchModels();

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    const isModelDownloaded = (modelId: string) => {
        return (
            downloadedModels.has(modelId) ||
            models.some((m) => m.name === modelId)
        );
    };

    const handleDownload = async (modelId: string) => {
        if (isModelDownloaded(modelId)) {
            await updateSettings({ model: modelId });
            setSelectedModel(modelId);
            return;
        }

        setDownloadingModel(modelId);
        setProgress(0);
        setError(null);
        try {
            await pullModel(modelId);
            setDownloadedModels((prev) => new Set(prev).add(modelId));
            await updateSettings({ model: modelId });
            setSelectedModel(modelId);
        } catch (error: unknown) {
            console.error('Failed to download model', error);
            const errorMessage =
                error instanceof Error ? error.message : String(error);
            setError(
                errorMessage ||
                    t(
                        'Failed to download model. Please check your connection and try again.'
                    )
            );
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
            className="flex flex-col items-center max-w-4xl mx-auto space-y-8 py-8 h-fit"
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
                        isDownloaded={isModelDownloaded(model.id)}
                        isDownloading={downloadingModel === model.id}
                        progress={progress}
                        onSelect={handleDownload}
                    />
                ))}
            </div>

            <div className="flex flex-col items-center gap-4 w-full">
                {error && (
                    <div className="flex items-center gap-2 text-red-400 bg-red-400/10 px-4 py-2 rounded-lg text-sm animate-in fade-in slide-in-from-bottom-2">
                        <AlertCircle className="w-4 h-4" />
                        {error}
                    </div>
                )}

                <Button
                    onClick={handleCustomModel}
                    variant="ghost"
                    className="text-zinc-500 hover:text-zinc-300 hover:bg-transparent"
                >
                    {t('Choose an other model manually')}
                </Button>
            </div>

            <div className="flex justify-between w-full pt-4">
                <div />
                <div>
                    <Page.PrimaryButton
                        onClick={onNext}
                        disabled={!selectedModel}
                        size="lg"
                        className="px-8"
                        data-testid="llm-connect-next-button"
                    >
                        {t('Finish Setup')}
                    </Page.PrimaryButton>
                </div>
            </div>
        </motion.div>
    );
};
