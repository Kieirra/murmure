import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { CornerDownLeft } from 'lucide-react';
import { useAutoSendEnterState } from './hooks/use-auto-send-enter-state';
import { useTranslation } from '@/i18n';

export const AutoSendEnterSettings = () => {
    const { autoSendEnter, setAutoSendEnter } = useAutoSendEnterState();
    const { t } = useTranslation();

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <CornerDownLeft className="w-4 h-4 text-zinc-400" />
                    {t('Auto-send Enter')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t(
                        'Automatically send Enter key after pasting transcription'
                    )}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch
                checked={autoSendEnter}
                onCheckedChange={setAutoSendEnter}
            />
        </SettingsUI.Item>
    );
};
