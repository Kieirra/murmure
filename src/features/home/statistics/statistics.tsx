import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/tooltip';
import { Typography } from '@/components/typography';
import { Gauge, Hourglass, WifiOff } from 'lucide-react';
import { useGetStatistic } from './hooks/use-get-statistic';
import { computeTypingMultiplier, formatData, formatTimeSaved } from './statistics.helpers';
import { StatCard } from './stat-card/stat-card';
import { useTranslation } from '@/i18n';

export const Statistics = () => {
    const { wpm, words, localAudioMb, timeSavedSeconds } = useGetStatistic();
    const { t } = useTranslation();

    const multiplier = computeTypingMultiplier(wpm);

    // Hide stats entirely until the user has actually dictated something.
    if (wpm <= 0 && words <= 0 && localAudioMb <= 0 && timeSavedSeconds <= 0) {
        return null;
    }

    return (
        <div className="flex gap-3">
            <Tooltip>
                <TooltipTrigger asChild>
                    <div className="flex flex-1 cursor-pointer">
                        <StatCard
                            icon={Hourglass}
                            value={formatTimeSaved(timeSavedSeconds)}
                            label={t('gained this month')}
                            subtitle={t('by speaking instead of typing')}
                            accent="cyan"
                            iconNudge="down"
                        />
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    <Typography.Paragraph className="text-white text-xs max-w-64">
                        {t('The time you saved this month by dictating instead of typing on a keyboard.')}
                    </Typography.Paragraph>
                </TooltipContent>
            </Tooltip>
            <Tooltip>
                <TooltipTrigger asChild>
                    <div className="flex flex-1 cursor-pointer">
                        <StatCard
                            icon={Gauge}
                            value={wpm > 0 ? wpm.toFixed(0) : '-'}
                            label={t('wpm')}
                            accent="sky"
                            iconNudge="up"
                            subtitle={
                                multiplier != null ? t('{{multiplier}} faster than typing', { multiplier }) : undefined
                            }
                        />
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    <Typography.Paragraph className="text-white text-xs max-w-64">
                        {t('Your average words per minute with Murmure this month.')}
                        <br />
                        <br />
                        {t('A fast keyboard user usually types around 80 words per minute. You can speak much faster.')}
                    </Typography.Paragraph>
                </TooltipContent>
            </Tooltip>
            <Tooltip>
                <TooltipTrigger asChild>
                    <div className="flex flex-1 cursor-pointer">
                        <StatCard
                            icon={WifiOff}
                            value={formatData(localAudioMb > 0 ? localAudioMb.toFixed(1) : '-')}
                            label={t('Processed locally')}
                            subtitle={t('and never sent to the cloud')}
                            accent="indigo"
                            iconNudge="down-sm"
                        />
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    <Typography.Paragraph className="text-white text-xs max-w-64">
                        {t(
                            'The total volume of audio Murmure has processed locally on your device since you started using it.'
                        )}
                        <br />
                        <br />
                        {t(
                            'All audio files are deleted after processing, and your transcriptions stay in memory (RAM), never written to disk or sent to the cloud.'
                        )}
                    </Typography.Paragraph>
                </TooltipContent>
            </Tooltip>
        </div>
    );
};
