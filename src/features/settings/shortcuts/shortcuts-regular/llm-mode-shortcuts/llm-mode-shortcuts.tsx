import { Typography } from '@/components/typography';
import { RenderKeys } from '@/components/render-keys';
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
    className?: string;
}

export const LlmModeShortcuts = ({ dictate, transform, className }: LlmModeShortcutsProps) => {
    const { t } = useTranslation();

    return (
        <SettingsUI.Container className={className}>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title>{dictate.label}</Typography.Title>
                    <Typography.Paragraph>
                        {t('Press')} <RenderKeys keyString={dictate.shortcut} />
                        {t(' to dictate and rewrite with this prompt.')}
                    </Typography.Paragraph>
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
                <SettingsUI.Description>
                    <Typography.Title>{transform.label}</Typography.Title>
                    <Typography.Paragraph>
                        {t('Press')} <RenderKeys keyString={transform.shortcut} />
                        {t(' to apply this prompt to the selected text.')}
                    </Typography.Paragraph>
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
        </SettingsUI.Container>
    );
};
