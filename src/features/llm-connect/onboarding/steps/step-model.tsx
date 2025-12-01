import { useTranslation } from 'react-i18next';
import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { Download, Check, Loader2, Zap, Brain, Globe } from 'lucide-react';
import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

interface StepModelProps {
    onNext: () => void;
    pullModel: (model: string) => Promise<void>;
    updateSettings: (settings: { model: string }) => Promise<void>;
}

interface RecommendedModel {
    id: string;
    name: string;
    description: string;
    size: string;
    icon: any;
    tags: string[];
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
            id: 'qwen2.5:7b',
            name: 'Qwen 2.5 (7B)',
            description: t(
                'Precise and efficient, but slower. Requires a good computer.'
            ),
            size: '4.7 GB',
            icon: Brain,
            tags: [t('Precise'), t('Multilingual')],
        },
        {
            id: 'gemma2:2b',
            name: 'Gemma 2 (2B)',
            description: t(
                'Very fast and lightweight. Ideal for older computers.'
            ),
            size: '1.6 GB',
            icon: Zap,
            tags: [t('Fast'), t('Lightweight')],
        },
    ];

    const handleCustomModel = async () => {
        // Just proceed without downloading, user will configure manually
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
                    {t(
                        'Choose a local AI model to power your transcriptions. We recommend starting with a small, fast model.'
                    )}
                </Typography.Paragraph>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 w-full">
                {recommendedModels.map((model) => {
                    const isDownloaded = downloadedModels.has(model.id);
                    const isDownloading = downloadingModel === model.id;
                    const isSelected = selectedModel === model.id;

                    return (
                        <div
                            key={model.id}
                            className={`relative flex flex-col p-6 rounded-xl border transition-all duration-200 ${
                                isSelected
                                    ? 'bg-blue-500/10 border-blue-500/50 ring-1 ring-blue-500/50'
                                    : 'bg-zinc-900/50 border-zinc-800 hover:border-zinc-700'
                            }`}
                        >
                            <div className="flex items-start justify-between mb-4">
                                <div className="p-2 rounded-lg bg-zinc-800">
                                    <model.icon className="w-5 h-5 text-zinc-300" />
                                </div>
                                {isDownloaded && (
                                    <div className="bg-green-500/20 text-green-400 p-1 rounded-full">
                                        <Check className="w-4 h-4" />
                                    </div>
                                )}
                            </div>

                            <h3 className="font-semibold text-lg mb-1">
                                {model.name}
                            </h3>
                            <p className="text-xs text-zinc-500 mb-3">
                                {model.size}
                            </p>
                            <p className="text-sm text-zinc-400 mb-6 flex-grow">
                                {model.description}
                            </p>

                            <div className="flex flex-wrap gap-2 mb-6">
                                {model.tags.map((tag) => (
                                    <span
                                        key={tag}
                                        className="text-[10px] px-2 py-1 rounded-full bg-zinc-800 text-zinc-400"
                                    >
                                        {tag}
                                    </span>
                                ))}
                            </div>

                            <Button
                                onClick={() => handleDownload(model.id)}
                                disabled={isDownloading || isDownloaded}
                                variant={
                                    isDownloaded
                                        ? isSelected
                                            ? 'default'
                                            : 'outline'
                                        : 'default'
                                }
                                className={`w-full ${
                                    isDownloaded
                                        ? isSelected
                                            ? 'bg-blue-600 hover:bg-blue-700'
                                            : 'border-zinc-700 hover:bg-zinc-800'
                                        : ''
                                }`}
                            >
                                {isDownloading ? (
                                    <>
                                        <Loader2 className="w-4 h-4 animate-spin mr-2" />
                                        {progress}%
                                    </>
                                ) : isDownloaded ? (
                                    isSelected ? (
                                        t('Selected')
                                    ) : (
                                        t('Select')
                                    )
                                ) : (
                                    <>
                                        <Download className="w-4 h-4 mr-2" />
                                        {t('Download')}
                                    </>
                                )}
                            </Button>

                            {/* Progress bar for downloading */}
                            {isDownloading && (
                                <div className="absolute bottom-0 left-0 w-full h-1 bg-zinc-800 rounded-b-xl overflow-hidden">
                                    <motion.div
                                        className="h-full bg-blue-500"
                                        initial={{ width: 0 }}
                                        animate={{ width: `${progress}%` }}
                                        transition={{ duration: 0.2 }}
                                    />
                                </div>
                            )}
                        </div>
                    );
                })}

                {/* Custom Model Option */}
                <div className="relative flex flex-col p-6 rounded-xl border border-zinc-800 bg-zinc-900/50 hover:border-zinc-700 transition-all duration-200">
                    <div className="flex items-start justify-between mb-4">
                        <div className="p-2 rounded-lg bg-zinc-800">
                            <Globe className="w-5 h-5 text-zinc-300" />
                        </div>
                    </div>

                    <h3 className="font-semibold text-lg mb-1">
                        {t('Custom Model')}
                    </h3>
                    <p className="text-xs text-zinc-500 mb-3">
                        {t('Manual Setup')}
                    </p>
                    <p className="text-sm text-zinc-400 mb-6 flex-grow">
                        {t(
                            'I want to use a specific model or configure it later.'
                        )}
                    </p>

                    <Button
                        onClick={handleCustomModel}
                        variant="outline"
                        className="w-full border-zinc-700 hover:bg-zinc-800"
                    >
                        {t('Select')}
                    </Button>
                </div>
            </div>

            <div className="flex justify-between w-full pt-4">
                <div />
                <Button
                    onClick={onNext}
                    disabled={!selectedModel}
                    size="lg"
                    className="px-8"
                >
                    {t('Finish Setup')}
                </Button>
            </div>
        </motion.div>
    );
};
