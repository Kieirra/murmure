import { SettingsUI } from '@/components/settings-ui';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { Gauge } from 'lucide-react';
import {
    isTranscriptionFinalizationStrategy,
    TranscriptionFinalizationStrategy,
    useTranscriptionFinalizationState,
} from './hooks/use-transcription-finalization-state';

const STRATEGIES: { key: TranscriptionFinalizationStrategy; label: string }[] = [
    { key: 'wav', label: 'Full WAV' },
    { key: 'streaming', label: 'Streaming' },
    { key: 'streaming_corrected', label: 'Streaming + correction' },
];

export const TranscriptionFinalizationSettings = () => {
    const { t } = useTranslation();
    const { transcriptionFinalizationStrategy, setTranscriptionFinalizationStrategy } =
        useTranscriptionFinalizationState();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Gauge className="w-4 h-4 text-muted-foreground" />
                    {t('Transcription processing')}
                    <SettingsUI.BadgeExperimental label={t('Experimental')} />
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Choose how Murmure finalizes transcription after recording.')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Select
                value={transcriptionFinalizationStrategy}
                onValueChange={(value) => {
                    if (isTranscriptionFinalizationStrategy(value)) {
                        setTranscriptionFinalizationStrategy(value);
                    }
                }}
            >
                <SelectTrigger className="w-[260px]" data-testid="transcription-finalization-select">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {STRATEGIES.map((strategy) => (
                        <SelectItem key={strategy.key} value={strategy.key}>
                            {t(strategy.label)}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </SettingsUI.Item>
    );
};
