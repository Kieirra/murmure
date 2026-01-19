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

    const {
        shortcut: commandShortcut,
        setShortcut: setCommandShortcut,
        resetShortcut: resetCommandShortcut,
    } = useShortcut(SHORTCUT_CONFIGS.command);

    const {
        shortcut: llmMode1Shortcut,
        setShortcut: setLLMMode1Shortcut,
        resetShortcut: resetLLMMode1Shortcut,
    } = useShortcut(SHORTCUT_CONFIGS.llmMode1);

    const {
        shortcut: llmMode2Shortcut,
        setShortcut: setLLMMode2Shortcut,
        resetShortcut: resetLLMMode2Shortcut,
    } = useShortcut(SHORTCUT_CONFIGS.llmMode2);

    const {
        shortcut: llmMode3Shortcut,
        setShortcut: setLLMMode3Shortcut,
        resetShortcut: resetLLMMode3Shortcut,
    } = useShortcut(SHORTCUT_CONFIGS.llmMode3);

    const {
        shortcut: llmMode4Shortcut,
        setShortcut: setLLMMode4Shortcut,
        resetShortcut: resetLLMMode4Shortcut,
    } = useShortcut(SHORTCUT_CONFIGS.llmMode4);

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
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>{t('Command')}</Typography.Title>
                            <Typography.Paragraph>
                                {t('Press')}{' '}
                                <RenderKeys keyString={commandShortcut} />
                                {t(
                                    ' to execute a voice command on selected text.'
                                )}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={t('Command')}
                            shortcut={commandShortcut}
                            saveShortcut={setCommandShortcut}
                            resetShortcut={resetCommandShortcut}
                            dataTestId="command-button"
                        />
                    </SettingsUI.Item>
                </SettingsUI.Container>

                <SettingsUI.Container className="mb-6">
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>
                                {t('LLM Mode')} 1
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('Press')}{' '}
                                <RenderKeys keyString={llmMode1Shortcut} />
                                {t(' to switch to LLM mode 1.')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={`${t('LLM Mode')} 1`}
                            shortcut={llmMode1Shortcut}
                            saveShortcut={setLLMMode1Shortcut}
                            resetShortcut={resetLLMMode1Shortcut}
                            dataTestId="llm-mode-1-button"
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>
                                {t('LLM Mode')} 2
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('Press')}{' '}
                                <RenderKeys keyString={llmMode2Shortcut} />
                                {t(' to switch to LLM mode 2.')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={`${t('LLM Mode')} 2`}
                            shortcut={llmMode2Shortcut}
                            saveShortcut={setLLMMode2Shortcut}
                            resetShortcut={resetLLMMode2Shortcut}
                            dataTestId="llm-mode-2-button"
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>
                                {t('LLM Mode')} 3
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('Press')}{' '}
                                <RenderKeys keyString={llmMode3Shortcut} />
                                {t(' to switch to LLM mode 3.')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={`${t('LLM Mode')} 3`}
                            shortcut={llmMode3Shortcut}
                            saveShortcut={setLLMMode3Shortcut}
                            resetShortcut={resetLLMMode3Shortcut}
                            dataTestId="llm-mode-3-button"
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>
                                {t('LLM Mode')} 4
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('Press')}{' '}
                                <RenderKeys keyString={llmMode4Shortcut} />
                                {t(' to switch to LLM mode 4.')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName={`${t('LLM Mode')} 4`}
                            shortcut={llmMode4Shortcut}
                            saveShortcut={setLLMMode4Shortcut}
                            resetShortcut={resetLLMMode4Shortcut}
                            dataTestId="llm-mode-4-button"
                        />
                    </SettingsUI.Item>
                </SettingsUI.Container>
            </div>
        </main>
    );
};
