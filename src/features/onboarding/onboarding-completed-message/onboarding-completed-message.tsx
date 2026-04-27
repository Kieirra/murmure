import { Typography } from '@/components/typography';
import { useTranslation } from '@/i18n';
import { useMicState } from '@/features/settings/system/mic-settings/hooks/use-mic-state';

export const OnboardingCompletedMessage = () => {
    const { t } = useTranslation();
    const { currentMic, micList } = useMicState();
    const selectedMic = currentMic === 'automatic' ? null : micList.find((m) => m.id === currentMic);

    return (
        <Typography.Paragraph className="text-muted-foreground">
            {selectedMic ? (
                <>
                    {t('Recording with ')}
                    <span className="text-sky-400">{selectedMic.label}</span>
                </>
            ) : (
                t('Murmure uses your default microphone to record your voice.')
            )}
        </Typography.Paragraph>
    );
};
