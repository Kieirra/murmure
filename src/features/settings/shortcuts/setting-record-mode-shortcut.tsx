import { useTranslation } from '@/i18n';
import { useRecordModeState } from '@/features/settings/system/record-mode-settings/hooks/use-record-mode-state.ts';
import { SettingsUI } from '@/components/settings-ui.tsx';
import { Typography } from '@/components/typography.tsx';
import { RenderKeys } from '@/components/render-keys.tsx';
import { ShortcutButton } from '@/features/settings/shortcuts/shortcut-button/shortcut-button.tsx';
import { useRecordShortcutState } from '@/features/settings/shortcuts/hooks/use-record-shortcut-state.ts';

export const SettingRecordModeShortcut = () => {
    const { recordMode } = useRecordModeState();

    if (recordMode === 'push_to_talk') {
        return <PushToTalkShortcut />
    }
    return <ToggleToTalkShortcut />
}

const PushToTalkShortcut = () => {
    const { t } = useTranslation();
    const { recordShortcut, setRecordShortcut, resetRecordShortcut } =
        useRecordShortcutState();
    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title>
                    {t('Push to talk')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Hold')}{' '}
                    <RenderKeys keyString={recordShortcut} />
                    {t(' to record, release to transcribe.')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <ShortcutButton
                keyName={t('Push to talk')}
                shortcut={recordShortcut}
                saveShortcut={setRecordShortcut}
                resetShortcut={resetRecordShortcut}
                dataTestId="push-to-talk-button"
            />
        </SettingsUI.Item>
    );
}

const ToggleToTalkShortcut = () => {
    const { t } = useTranslation();
    const { recordShortcut, setRecordShortcut, resetRecordShortcut } =
        useRecordShortcutState();
    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title>
                    {t('Toggle to talk')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Toggle')}{' '}
                    <RenderKeys keyString={recordShortcut} />
                    {t(' to start/stop recording')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <ShortcutButton
                keyName={t('Toggle to talk')}
                shortcut={recordShortcut}
                saveShortcut={setRecordShortcut}
                resetShortcut={resetRecordShortcut}
                dataTestId="toggle-to-talk-button"
            />
        </SettingsUI.Item>
    );
}