import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/tooltip';
import { Typography } from '@/components/typography';
import { ChevronsUp, FileText, WifiOff } from 'lucide-react';
import { useGetStatistic } from './hooks/use-get-statistic';
import { formatData, formatWords } from './statistics.helpers';
import clsx from 'clsx';
import { useTranslation } from '@/i18n';

export const Statistics = ({
    className,
    ...props
}: React.HTMLAttributes<HTMLDivElement>) => {
    const { wpm, words, data } = useGetStatistic();
    const { t } = useTranslation('home');

    return (
        <div
            className={clsx(
                'flex border border-zinc-700 bg-zinc-800 rounded-full text-xs space-x-2 px-2',
                className
            )}
            {...props}
        >
            <Tooltip>
                <TooltipTrigger asChild>
                    <div className="flex items-center gap-2 cursor-pointer p-1">
                        <ChevronsUp
                            width={16}
                            height={16}
                            className="text-emerald-400"
                        />
                        <span>{wpm} {t('statistics.wpmUnit')}</span>
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    <Typography.Paragraph className="text-white text-xs max-w-64">
                        {t('statistics.wpmTooltip.line1')}
                        <br />
                        <br />
                        {t('statistics.wpmTooltip.line2')}
                    </Typography.Paragraph>
                </TooltipContent>
            </Tooltip>
            <span className="text-zinc-400">|</span>
            <Tooltip>
                <TooltipTrigger asChild>
                    <div className="flex items-center gap-2 cursor-pointer p-1">
                        <FileText
                            width={16}
                            height={16}
                            className="text-yellow-400"
                        />
                        {formatWords(words)} {t('statistics.wordsUnit')}
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    <Typography.Paragraph className="text-white text-xs max-w-64">
                        {t('statistics.wordsTooltip.line1')}
                        <br />
                        <br />
                        {t('statistics.wordsTooltip.line2')}
                    </Typography.Paragraph>
                </TooltipContent>
            </Tooltip>
            <span className="text-zinc-400">|</span>
            <Tooltip>
                <TooltipTrigger asChild>
                    <div className="flex items-center gap-2 cursor-pointer p-1">
                        <WifiOff
                            width={16}
                            height={16}
                            className="text-red-400"
                        />
                        {formatData(data)}
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    <Typography.Paragraph className="text-white text-xs max-w-64">
                        {t('statistics.dataTooltip.line1')}
                        <br />
                        <br />
                        {t('statistics.dataTooltip.line2')}
                    </Typography.Paragraph>
                </TooltipContent>
            </Tooltip>
        </div>
    );
};
