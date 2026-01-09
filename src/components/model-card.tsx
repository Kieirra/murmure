import clsx from 'clsx';
import { motion } from 'framer-motion';
import { Check, Download, Loader2, Star } from 'lucide-react';
import { ElementType } from 'react';
import { useTranslation } from 'react-i18next';
import { Page } from './page';
import { Typography } from './typography';

export interface RecommendedModel {
    id: string;
    name: string;
    description: string;
    bullets?: string[];
    size: string;
    ram: string;
    icon: ElementType;
    tags: string[];
    recommended?: boolean;
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

    const renderModelButtonContent = () => {
        if (isDownloading) {
            return (
                <>
                    <Loader2 className="w-4 h-4 animate-spin mr-2" />
                    {progress}%
                </>
            );
        } else if (isDownloaded) {
            if (isSelected) {
                return (
                    <>
                        <Check className="w-4 h-4 mr-2" />
                        {t('Selected')}
                    </>
                );
            }
            return t('Select');
        } else {
            return (
                <>
                    <Download className="w-4 h-4 mr-2" />
                    {t('Install')}
                </>
            );
        }
    };

    return (
        <div
            className={clsx(
                'relative flex flex-col p-4 rounded-xl border transition-all duration-200',
                isSelected
                    ? 'bg-blue-500/10 border-blue-500/50 ring-1 ring-blue-500/50'
                    : 'bg-zinc-800/30 border-zinc-800 hover:border-zinc-700'
            )}
        >
            <div className="relative py-2 flex justify-center">
                {model.recommended && (
                    <div className="text-xs text-zinc-300 absolute -top-4 -translate-y-1/2 border-1 rounded-sm p-1 z-10 bg-zinc-800 flex items-center justify-center shadow-sm border-zinc-700">
                        <Star className="w-3 h-3 mr-1 text-yellow-400 fill-yellow-400" />
                        {t('Recommended')}
                    </div>
                )}
                <model.icon className="w-10 h-10 text-zinc-200" />
                {isSelected && (
                    <div className="bg-blue-500 text-white p-1 rounded-full absolute -top-1 -right-2 shadow-sm">
                        <Check className="w-3 h-3" />
                    </div>
                )}
            </div>

            <Typography.MainTitle className="font-semibold text-lg mb-2 text-center">
                {model.name}
            </Typography.MainTitle>

            <Typography.Paragraph className="text-sm text-zinc-300 mb-4 text-center font-medium min-h-[40px] flex items-center justify-center">
                {model.description}
            </Typography.Paragraph>

            <div className="flex flex-wrap gap-2 mb-4 justify-center">
                {model.tags.map((tag) => (
                    <span
                        key={tag}
                        className="text-[10px] px-2.5 py-1 rounded-full bg-zinc-700/50 text-zinc-300 border border-zinc-700 font-medium"
                    >
                        {tag}
                    </span>
                ))}
            </div>

            {model.bullets && (
                <ul className="space-y-2 mb-4 flex-grow">
                    {model.bullets.map((bullet) => (
                        <li
                            key={bullet}
                            className="flex items-start text-xs text-zinc-400"
                        >
                            <span className="mr-2 text-zinc-500">•</span>
                            {bullet}
                        </li>
                    ))}
                </ul>
            )}

            <div className="text-[10px] text-zinc-500 text-center mb-4 mt-auto border-t border-zinc-800 pt-2">
                {model.size} · {model.ram}
            </div>

            <Page.PrimaryButton
                onClick={() => onSelect(model.id)}
                disabled={isDownloading}
                data-testid={
                    model.recommended
                        ? 'llm-connect-model-card-button-recommended'
                        : 'llm-connect-model-card-button'
                }
                className={clsx(
                    'w-full shadow-none',
                    isDownloaded &&
                        !isSelected &&
                        'bg-zinc-700 hover:bg-zinc-600 text-zinc-200'
                )}
            >
                {renderModelButtonContent()}
            </Page.PrimaryButton>

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
