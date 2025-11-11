import { Typography } from '@/components/typography';
import { ShortcutButton } from './shortcut-button/shortcut-button';
import { RenderKeys } from '../../../components/render-keys';
import { SettingsUI } from '@/components/settings-ui';
import { useRecordShortcutState } from './hooks/use-record-shortcut-state';
import { Page } from '@/components/page';
import { useLastTranscriptShortcutState } from './hooks/use-last_transcript-shortcut-state';
import { useTranslation } from '@/i18n';

interface ShortcutsProps {}

export const Shortcuts = ({}: ShortcutsProps) => {
    const { recordShortcut, setRecordShortcut, resetRecordShortcut } =
        useRecordShortcutState();
    const {
        lastTranscriptShortcut,
        setLastTranscriptShortcut,
        resetLastTranscriptShortcut,
    } = useLastTranscriptShortcutState();
    const { t } = useTranslation('settings');

    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle>{t('shortcuts.title')}</Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        {t('shortcuts.description')}
                    </Typography.Paragraph>
                </Page.Header>

                <SettingsUI.Container>
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>{t('shortcuts.pushToTalk.title')}</Typography.Title>
                            <Typography.Paragraph>
                                {t('shortcuts.pushToTalk.description')}
                                <RenderKeys keyString={recordShortcut} />
                                {t('shortcuts.pushToTalk.descriptionSuffix')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={t('shortcuts.pushToTalk.title')}
                            shortcut={recordShortcut}
                            saveShortcut={setRecordShortcut}
                            resetShortcut={resetRecordShortcut}
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>
                                {t('shortcuts.pasteLastTranscript.title')}
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('shortcuts.pasteLastTranscript.description')}
                                <RenderKeys
                                    keyString={lastTranscriptShortcut}
                                />
                                {t('shortcuts.pasteLastTranscript.descriptionSuffix')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={t('shortcuts.pasteLastTranscript.title')}
                            shortcut={lastTranscriptShortcut}
                            saveShortcut={setLastTranscriptShortcut}
                            resetShortcut={resetLastTranscriptShortcut}
                        />
                    </SettingsUI.Item>
                </SettingsUI.Container>
            </div>
        </main>
    );
};
