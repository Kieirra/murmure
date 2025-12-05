import clsx from 'clsx';
import { motion } from 'framer-motion';
import { Check, Loader2, Download } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Page } from './page';
import { Typography } from './typography';

export interface RecommendedModel {
    id: string;
    name: string;
    description: string;
    size: string;
    icon: any;
    tags: string[];
}

export const ModelCard = ({
    model,
    isSelected,
    isDownloaded,
    isDownloading,
    progress,
    onSelect,
}: {
    model: RecommendedModel;
    isSelected: boolean;
    isDownloaded: boolean;
    isDownloading: boolean;
    progress: number;
    onSelect: (id: string) => void;
}) => {
    const { t } = useTranslation();

    return (
        <div
            className={clsx(
                'relative flex flex-col p-6 rounded-xl border transition-all duration-200',
                isSelected
                    ? 'bg-blue-500/10 border-blue-500/50 ring-1 ring-blue-500/50'
                    : 'bg-zinc-800/30 border-zinc-800 hover:border-zinc-700'
            )}
        >
            <div className="flex items-start justify-between mb-4">
                <div className="p-2 rounded-lg bg-zinc-700">
                    <model.icon className="w-5 h-5 text-zinc-300" />
                </div>
                {isDownloaded && (
                    <div className="bg-green-500/20 text-green-400 p-1 rounded-full">
                        <Check className="w-4 h-4" />
                    </div>
                )}
            </div>

            <Typography.MainTitle className="font-semibold text-lg mb-1">
                {model.name}
            </Typography.MainTitle>
            <Typography.Paragraph className="text-xs text-zinc-500 mb-3">
                {model.size}
            </Typography.Paragraph>
            <Typography.Paragraph className="text-sm text-zinc-400 mb-6 flex-grow">
                {model.description}
            </Typography.Paragraph>

            <div className="flex flex-wrap gap-2 mb-6">
                {model.tags.map((tag) => (
                    <span
                        key={tag}
                        className="text-[10px] px-2 py-1 rounded-full bg-zinc-700 text-zinc-300"
                    >
                        {tag}
                    </span>
                ))}
            </div>

            <Page.SecondaryButton
                onClick={() => onSelect(model.id)}
                disabled={isDownloading}
                className={clsx(
                    'w-full',
                    isDownloaded &&
                        (isSelected
                            ? 'bg-blue-600 hover:bg-blue-700'
                            : 'border-zinc-700 hover:bg-zinc-800')
                )}
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
            </Page.SecondaryButton>

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
};
