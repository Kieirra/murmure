import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { TextCursorInput } from 'lucide-react';
import { useAutoInsertState } from './hooks/use-auto-insert-state';
import { useTranslation } from '@/i18n';

export const AutoInsertSettings = () => {
    const { autoInsert, setAutoInsert } = useAutoInsertState();
    const { t } = useTranslation();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <TextCursorInput className="w-4 h-4 text-muted-foreground" />
                    {t('Automatic insert')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t(
                        'Insert the transcription automatically at the cursor. When off, use the "Paste last transcript" shortcut to insert it.'
                    )}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch checked={autoInsert} onCheckedChange={setAutoInsert} />
        </SettingsUI.Item>
    );
};
