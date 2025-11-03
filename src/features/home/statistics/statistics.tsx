import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/tooltip';
import { Typography } from '@/components/typography';
import { ChevronsUp, FileText, WifiOff } from 'lucide-react';
import { useGetStatistic } from './hooks/use-get-statistic';
import { formatData, formatWords } from './statistics.helpers';
import clsx from 'clsx';

export const Statistics = ({
    className,
    ...props
}: React.HTMLAttributes<HTMLDivElement>) => {
    const { wpm, words, data } = useGetStatistic();

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
                        <span>{wpm} wpm</span>
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    <Typography.Paragraph className="text-white text-xs max-w-64">
                        Your average words per minute with Murmure this month.
                        <br />
                        <br />A fast keyboard user usually types around 80 words
                        per minute. You can speak much faster.
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
                        {formatWords(words)} words
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    <Typography.Paragraph className="text-white text-xs max-w-64">
                        Total words written with Murmure this month.
                        <br />
                        <br />
                        That’s thousands of ideas turned into text, without
                        typing a single key.
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
                        Data processed locally instead of being sent to the
                        Cloud this month.
                        <br />
                        <br />
                        Murmure removes all audio files after processing and
                        only keeps your five latest transcriptions, stored
                        locally on your device.
                    </Typography.Paragraph>
                </TooltipContent>
            </Tooltip>
        </div>
    );
};
