import { Typography } from '@/components/typography';
import { ShortcutButton } from './shortcut-button/shortcut-button';
import { RenderKeys } from '@/components/render-keys.tsx';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { useShortcut, SHORTCUT_CONFIGS } from './hooks/use-shortcut';
import { useTranslation } from '@/i18n';
import { useRecordModeState } from '@/features/settings/system/record-mode-settings/hooks/use-record-mode-state';

export const Shortcuts = () => {
    const { t } = useTranslation();
    const { recordMode } = useRecordModeState();

    const {
        shortcut: recordShortcut,
        setShortcut: setRecordShortcut,
        resetShortcut: resetRecordShortcut,
    } = useShortcut(SHORTCUT_CONFIGS.record);

    const {
        shortcut: lastTranscriptShortcut,
        setShortcut: setLastTranscriptShortcut,
        resetShortcut: resetLastTranscriptShortcut,
    } = useShortcut(SHORTCUT_CONFIGS.lastTranscript);

    const {
        shortcut: llmShortcut,
        setShortcut: setLLMShortcut,
        resetShortcut: resetLLMShortcut,
    } = useShortcut(SHORTCUT_CONFIGS.llm);

    const isPushToTalk = recordMode === 'push_to_talk';
    const recordTitle = isPushToTalk ? t('Push to talk') : t('Toggle to talk');
    const recordTestId = isPushToTalk
        ? 'push-to-talk-button'
        : 'toggle-to-talk-button';

    const recordVerb = isPushToTalk ? t('Hold') : t('Toggle');
    const recordDescription = isPushToTalk
        ? t(' to record, release to transcribe.')
        : t(' to start/stop recording');

    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle data-testid="shortcuts-title">
                        {t('Shortcuts')}
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        {t(
                            'Improve your workflow by setting up keyboard shortcuts.'
                        )}
                    </Typography.Paragraph>
                </Page.Header>

                <SettingsUI.Container>
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>{recordTitle}</Typography.Title>
                            <Typography.Paragraph>
                                {recordVerb}{' '}
                                <RenderKeys keyString={recordShortcut} />
                                {recordDescription}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={recordTitle}
                            shortcut={recordShortcut}
                            saveShortcut={setRecordShortcut}
                            resetShortcut={resetRecordShortcut}
                            dataTestId={recordTestId}
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>
                                {t('Paste last transcript')}
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('Press ')}
                                <RenderKeys
                                    keyString={lastTranscriptShortcut}
                                />
                                {t(
                                    ' to paste the last transcript. Useful when you forgot to select an input field when you started recording.'
                                )}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={t('Paste last transcript')}
                            shortcut={lastTranscriptShortcut}
                            saveShortcut={setLastTranscriptShortcut}
                            resetShortcut={resetLastTranscriptShortcut}
                            dataTestId="paste-transcript-button"
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>
                                {t('LLM Record')}
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('Hold')}{' '}
                                <RenderKeys keyString={llmShortcut} />
                                {t(' to record and process with LLM.')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={t('LLM Record')}
                            shortcut={llmShortcut}
                            saveShortcut={setLLMShortcut}
                            resetShortcut={resetLLMShortcut}
                            dataTestId="llm-record-button"
                        />
                    </SettingsUI.Item>
                </SettingsUI.Container>
            </div>
        </main>
    );
};
