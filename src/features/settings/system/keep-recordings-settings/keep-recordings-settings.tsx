import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { useKeepRecordingsState } from './hooks/use-keep-recordings-state';
import { FileAudio } from 'lucide-react';
import { useTranslation } from '@/i18n';

export const KeepRecordingsSettings = () => {
    const { keepRecordings, setKeepRecordings } = useKeepRecordingsState();
    const { t } = useTranslation();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <FileAudio className="w-4 h-4 text-muted-foreground" />
                    {t('Keep audio recordings')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t(
                        'Keep the recorded audio files in the system temporary folder instead of deleting them after transcription. Useful to report transcription issues.'
                    )}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch checked={keepRecordings} onCheckedChange={setKeepRecordings} />
        </SettingsUI.Item>
    );
};
