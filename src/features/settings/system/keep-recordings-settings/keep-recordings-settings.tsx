import { invoke } from '@tauri-apps/api/core';
import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Button } from '@/components/button';
import { Switch } from '@/components/switch';
import { useKeepRecordingsState } from './hooks/use-keep-recordings-state';
import { FileAudio, FolderOpen } from 'lucide-react';
import { revealItemInDir } from '@tauri-apps/plugin-opener';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

export const KeepRecordingsSettings = () => {
    const { keepRecordings, setKeepRecordings } = useKeepRecordingsState();
    const { t } = useTranslation();

    const handleOpenRecordingsFolder = async () => {
        try {
            const recordingsDir = await invoke<string>('get_recordings_dir');
            await revealItemInDir(recordingsDir);
        } catch (error) {
            console.error('Failed to open recordings folder:', error);
            toast.error(t('Failed to open recordings folder'));
        }
    };

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <FileAudio className="w-4 h-4 text-muted-foreground" />
                    {t('Keep audio recordings')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t(
                        'Keep the last five recordings in the system temporary folder instead of deleting them after transcription.'
                    )}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <div className="flex items-center gap-2">
                <Switch checked={keepRecordings} onCheckedChange={setKeepRecordings} />
                <Button
                    variant="outline"
                    size="icon"
                    onClick={handleOpenRecordingsFolder}
                    title={t('View recordings')}
                    data-testid="open-recordings-folder-button"
                >
                    <FolderOpen className="w-4 h-4" />
                </Button>
            </div>
        </SettingsUI.Item>
    );
};
