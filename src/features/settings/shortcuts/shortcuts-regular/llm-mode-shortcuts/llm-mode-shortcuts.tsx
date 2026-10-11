import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { useTranslation } from '@/i18n';
import { ShortcutButton } from '../shortcut-button/shortcut-button';
import { ExistingShortcut } from '../shortcut-button/hooks/use-shortcut-interactions.helpers';

interface LlmModeShortcutRow {
    label: string;
    shortcut: string;
    setShortcut: (shortcut: string) => void;
    resetShortcut: (existingShortcuts?: ExistingShortcut[]) => void;
    dataTestId: string;
    existingShortcuts: ExistingShortcut[];
}

interface LlmModeShortcutsProps {
    dictate: LlmModeShortcutRow;
    transform: LlmModeShortcutRow;
}

export const LlmModeShortcuts = ({ dictate, transform }: LlmModeShortcutsProps) => {
    const { t } = useTranslation();

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description className="flex-1 min-w-0">
                    <Typography.Title>{dictate.label}</Typography.Title>
                    <Typography.Paragraph>{t('Dictates and rewrites with this prompt.')}</Typography.Paragraph>
                </SettingsUI.Description>
                <ShortcutButton
                    keyName={dictate.label}
                    shortcut={dictate.shortcut}
                    saveShortcut={dictate.setShortcut}
                    resetShortcut={dictate.resetShortcut}
                    dataTestId={dictate.dataTestId}
                    existingShortcuts={dictate.existingShortcuts}
                />
            </SettingsUI.Item>
            <SettingsUI.Separator />
            <SettingsUI.Item>
                <SettingsUI.Description className="flex-1 min-w-0">
                    <Typography.Title>{transform.label}</Typography.Title>
                    <Typography.Paragraph>{t('Applies this prompt to the selected text.')}</Typography.Paragraph>
                </SettingsUI.Description>
                <ShortcutButton
                    keyName={transform.label}
                    shortcut={transform.shortcut}
                    saveShortcut={transform.setShortcut}
                    resetShortcut={transform.resetShortcut}
                    dataTestId={transform.dataTestId}
                    existingShortcuts={transform.existingShortcuts}
                />
            </SettingsUI.Item>
        </>
    );
};
