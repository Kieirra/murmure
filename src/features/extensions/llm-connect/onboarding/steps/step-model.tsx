import { useTranslation } from '@/i18n';
import { Button } from '@/components/button';
import { Typography } from '@/components/typography';
import { motion } from 'framer-motion';
import { Mistral, Qwen } from '@lobehub/icons';
import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { Page } from '@/components/page';
import { ModelCard, RecommendedModel } from '@/components/model-card';
import { AlertCircle, RefreshCw } from 'lucide-react';
import { getPresetLabel, getPromptByPreset } from '../../llm-connect.helpers';

import { OllamaModel, LLMConnectSettings } from '../../hooks/use-llm-connect';

interface StepModelProps {
    onNext: () => void;
    pullModel: (model: string) => Promise<void>;
    updateSettings: (updates: Partial<LLMConnectSettings>) => Promise<void>;
    settings: LLMConnectSettings;
    models: OllamaModel[];
    fetchModels: () => Promise<OllamaModel[]>;
    isInstallOnly?: boolean;
    isRemote?: boolean;
    remoteModels?: OllamaModel[];
    fetchRemoteModels?: () => Promise<OllamaModel[]>;
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
    settings,
    models,
    fetchModels,
    isInstallOnly = false,
    isRemote = false,
    remoteModels = [],
    fetchRemoteModels,
}: StepModelProps) => {
    const { t, i18n } = useTranslation();
    const [selectedModel, setSelectedModel] = useState<string | null>(null);
    const [downloadingModel, setDownloadingModel] = useState<string | null>(null);
    const [progress, setProgress] = useState<number>(0);
    const [downloadedModels, setDownloadedModels] = useState<Set<string>>(new Set());
    const [error, setError] = useState<string | null>(null);
    const [isRefreshing, setIsRefreshing] = useState(false);

    const recommendedModels: RecommendedModel[] = [
        {
            id: 'qwen3.5:latest',
            name: 'Qwen 3.5 (9B)',
            description: t('Best adherence to instructions'),
            bullets: [t('Highly reliable formatting'), t('Follows directives precisely')],
            size: t('~ 6.6 GB on disk'),
            vram: t('8 GB VRAM recommended'),
            icon: Qwen.Color,
            tags: [t('Balanced'), t('Obedient')],
            recommended: true,
        },
        {
            id: 'ministral-3:latest',
            name: 'Ministral 3 (8B)',
            description: t('Great raw reasoning power'),
            bullets: [t('Strong analytical abilities'), t('Less strict with instructions')],
            size: t('~ 6 GB on disk'),
            vram: t('7 GB VRAM recommended'),
            icon: Mistral.Color,
            tags: [t('Powerful'), t('Reasoning')],
        },
        {
            id: 'qwen3.5:4b',
            name: 'Qwen 3.5 (4B)',
            description: t('Optimized for low-end hardware'),
            bullets: [t('Low resource usage'), t('Less capable still reliable')],
            size: t('~ 3.4 GB on disk'),
            vram: t('4 GB VRAM recommended'),
            icon: Qwen.Color,
            tags: [t('Lightweight'), t('Efficient')],
        },
    ];

    const applyModelToFirstMode = async (modelName: string) => {
        const existingMode = settings.modes[0];
        await updateSettings({
            model: modelName,
            modes: [
                {
                    name: t(getPresetLabel('general')),
                    prompt: getPromptByPreset('general', i18n.language),
                    model: modelName,
                    shortcut: existingMode?.shortcut ?? 'Ctrl + Shift + 1',
                    provider: isRemote ? 'remote' : 'local',
                    wake_word: `alix ${t(getPresetLabel('general')).toLowerCase()}`,
                },
            ],
            active_mode_index: 0,
        });
    };

    const handleCustomModel = async () => {
        if (!isInstallOnly) {
            await applyModelToFirstMode('');
        }
        onNext();
    };

    useEffect(() => {
        if (isRemote) return;

        const unlisten = listen<OllamaPullProgressPayload>('llm-pull-progress', (event) => {
            const { total, completed, status } = event.payload;
            if (status === 'success') {
                setProgress(100);
            } else if (total && completed) {
                setProgress(Math.round((completed / total) * 100));
            }
        });

        fetchModels();

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    const isModelDownloaded = (modelId: string) => {
        return downloadedModels.has(modelId) || models.some((m) => m.name === modelId);
    };

    const handleDownload = async (modelId: string) => {
        if (isModelDownloaded(modelId)) {
            if (!isInstallOnly) {
                await applyModelToFirstMode(modelId);
            }
            setSelectedModel(modelId);
            return;
        }

        setDownloadingModel(modelId);
        setProgress(0);
        setError(null);
        try {
            await pullModel(modelId);
            setDownloadedModels((prev) => new Set(prev).add(modelId));

            if (!isInstallOnly) {
                await applyModelToFirstMode(modelId);
            }
            setSelectedModel(modelId);
        } catch (error: unknown) {
            console.error('Failed to download model', error);
            const errorMessage = error instanceof Error ? error.message : String(error);
            setError(errorMessage || t('Failed to download model. Please check your connection and try again.'));
        } finally {
            setDownloadingModel(null);
            setProgress(0);
        }
    };

    const handleRemoteSelect = async (modelName: string) => {
        setSelectedModel(modelName);
        if (!isInstallOnly) {
            await applyModelToFirstMode(modelName);
        }
    };

    const handleRefreshRemoteModels = async () => {
        if (fetchRemoteModels) {
            setIsRefreshing(true);
            try {
                await fetchRemoteModels();
            } catch {
                // Error handled in hook
            } finally {
                setIsRefreshing(false);
            }
        }
    };

    const title = isInstallOnly ? t('Install a Model') : t('Select a Model');
    const getSubtitle = () => {
        if (isInstallOnly) return t('Download another model to use with your prompts.');
        if (isRemote) return t('Choose a model available on your remote server.');
        return t('Choose a local AI model to power your transcriptions.');
    };
    const subtitle = getSubtitle();
    const finishButtonText = isInstallOnly ? t('Done') : t('Finish Setup');

    if (isRemote) {
        return (
            <motion.div
                initial={{ opacity: 0, x: 20 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: -20 }}
                className="flex flex-col items-center max-w-4xl mx-auto space-y-3 py-4 h-fit"
            >
                <div className="text-center space-y-2">
                    <Typography.MainTitle>{title}</Typography.MainTitle>
                    <Typography.Paragraph className="text-muted-foreground max-w-lg mx-auto">
                        {subtitle}
                    </Typography.Paragraph>
                </div>

                <div className="w-full bg-card/30 border border-border rounded-xl p-4">
                    {remoteModels.length === 0 ? (
                        <div className="text-center py-4 text-muted-foreground">
                            <Typography.Paragraph>{t('No models found on this server.')}</Typography.Paragraph>
                        </div>
                    ) : (
                        <div className="max-h-[400px] overflow-y-auto pr-1">
                            {remoteModels.map((model) => (
                                <label
                                    key={model.name}
                                    className="flex items-center gap-3 p-2 rounded-lg hover:bg-card/50 cursor-pointer transition-colors"
                                >
                                    <input
                                        type="radio"
                                        name="remote-model"
                                        checked={selectedModel === model.name}
                                        onChange={() => handleRemoteSelect(model.name)}
                                        className="accent-sky-500"
                                    />
                                    <span className="text-sm text-foreground">{model.name}</span>
                                </label>
                            ))}
                        </div>
                    )}
                </div>

                <div className="flex flex-col items-center gap-1">
                    <Button
                        onClick={handleRefreshRemoteModels}
                        variant="ghost"
                        className="text-muted-foreground hover:text-foreground hover:bg-transparent"
                        disabled={isRefreshing}
                    >
                        <RefreshCw className={`w-4 h-4 mr-2 ${isRefreshing ? 'animate-spin' : ''}`} />
                        {t('Refresh list')}
                    </Button>
                    <p className="text-xs text-muted-foreground">
                        {t('On a remote server, you cannot install new models from Murmure.')}
                    </p>
                </div>

                <div className="flex justify-between w-full">
                    <div />
                    <div>
                        <Page.PrimaryButton
                            onClick={onNext}
                            disabled={!selectedModel}
                            size="lg"
                            className="px-8"
                            data-testid="llm-connect-next-button"
                        >
                            {finishButtonText}
                        </Page.PrimaryButton>
                    </div>
                </div>
            </motion.div>
        );
    }

    return (
        <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            className="flex flex-col items-center max-w-4xl mx-auto space-y-8 py-8 h-fit"
        >
            <div className="text-center space-y-4">
                <Typography.MainTitle>{title}</Typography.MainTitle>
                <Typography.Paragraph className="text-muted-foreground max-w-lg mx-auto">
                    {subtitle}
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
                    className="text-muted-foreground hover:text-foreground hover:bg-transparent"
                >
                    {t('Choose an other model manually')}
                </Button>
            </div>

            <div className="flex justify-between w-full">
                <div />
                <div>
                    <Page.PrimaryButton
                        onClick={onNext}
                        disabled={!isInstallOnly && !selectedModel}
                        size="lg"
                        className="px-8"
                        data-testid="llm-connect-next-button"
                    >
                        {finishButtonText}
                    </Page.PrimaryButton>
                </div>
            </div>
        </motion.div>
    );
};
