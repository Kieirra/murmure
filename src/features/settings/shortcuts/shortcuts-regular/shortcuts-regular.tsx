import { Lightbulb } from 'lucide-react';
import { Typography } from '@/components/typography';
import { ShortcutButton } from './shortcut-button/shortcut-button';
import { RenderKeys } from '@/components/render-keys.tsx';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { useShortcut, SHORTCUT_CONFIGS } from '../hooks/use-shortcut';
import { useTranslation } from '@/i18n';
import { useRecordModeState } from '@/features/settings/system/record-mode-settings/hooks/use-record-mode-state';
import { useLlmOnboardingCompleted } from '@/features/extensions/llm-connect/hooks/use-llm-onboarding-completed';

export const ShortcutsRegular = () => {
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
        shortcut: cancelShortcut,
        setShortcut: setCancelShortcut,
        resetShortcut: resetCancelShortcut,
    } = useShortcut(SHORTCUT_CONFIGS.cancel);

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

    const {
        shortcut: voiceModeToggleShortcut,
        setShortcut: setVoiceModeToggleShortcut,
        resetShortcut: resetVoiceModeToggleShortcut,
    } = useShortcut(SHORTCUT_CONFIGS.voiceModeToggle);

    const llmOnboardingCompleted = useLlmOnboardingCompleted();

    const isPushToTalk = recordMode === 'push_to_talk';
    const recordTitle = isPushToTalk ? t('Push to talk') : t('Toggle to talk');
    const recordTestId = isPushToTalk ? 'push-to-talk-button' : 'toggle-to-talk-button';

    const recordVerb = isPushToTalk ? t('Hold') : t('Toggle');
    const recordDescription = isPushToTalk ? t(' to record, release to transcribe.') : t(' to start/stop recording');

    const allShortcuts = [
        { id: 'record', name: recordTitle, value: recordShortcut },
        { id: 'lastTranscript', name: t('Paste last transcript'), value: lastTranscriptShortcut },
        { id: 'cancel', name: t('Cancel recording'), value: cancelShortcut },
        { id: 'voiceModeToggle', name: t('Toggle Voice Mode'), value: voiceModeToggleShortcut },
        { id: 'command', name: t('Command'), value: commandShortcut },
        { id: 'llmMode1', name: `${t('Transcribe with LLM')} 1`, value: llmMode1Shortcut },
        { id: 'llmMode2', name: `${t('Transcribe with LLM')} 2`, value: llmMode2Shortcut },
        { id: 'llmMode3', name: `${t('Transcribe with LLM')} 3`, value: llmMode3Shortcut },
        { id: 'llmMode4', name: `${t('Transcribe with LLM')} 4`, value: llmMode4Shortcut },
    ];

    const othersOf = (id: string) =>
        allShortcuts.filter((entry) => entry.id !== id).map(({ name, value }) => ({ name, value }));

    return (
        <main>
            <div className="space-y-4">
                <Page.Header>
                    <Typography.MainTitle data-testid="shortcuts-title">{t('Shortcuts')}</Typography.MainTitle>
                    <Typography.Paragraph className="text-muted-foreground">
                        {t('Improve your workflow by setting up keyboard shortcuts.')}
                    </Typography.Paragraph>
                </Page.Header>

                <section>
                    <Typography.Title data-testid="general-title" className="p-2 font-semibold text-sky-400!">
                        {t('General')}
                    </Typography.Title>
                    <SettingsUI.Container>
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title>{recordTitle}</Typography.Title>
                                <Typography.Paragraph>
                                    {recordVerb} <RenderKeys keyString={recordShortcut} />
                                    {recordDescription}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <ShortcutButton
                                keyName={recordTitle}
                                shortcut={recordShortcut}
                                saveShortcut={setRecordShortcut}
                                resetShortcut={resetRecordShortcut}
                                dataTestId={recordTestId}
                                existingShortcuts={othersOf('record')}
                            />
                        </SettingsUI.Item>
                        <SettingsUI.Separator />
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title>{t('Paste last transcript')}</Typography.Title>
                                <Typography.Paragraph>
                                    {t('Press ')}
                                    <RenderKeys keyString={lastTranscriptShortcut} />
                                    {t(' to paste the last transcript.')}
                                </Typography.Paragraph>
                                <div className="mt-2 flex items-start gap-2 rounded-md bg-muted/40 px-2.5 py-2 text-xs text-muted-foreground">
                                    <Lightbulb className="w-4 h-4 mt-0.5 shrink-0 text-cyan-400" />
                                    <span>
                                        {t(
                                            'Useful when you forgot to select an input field when you started recording.'
                                        )}
                                    </span>
                                </div>
                            </SettingsUI.Description>
                            <ShortcutButton
                                keyName={t('Paste last transcript')}
                                shortcut={lastTranscriptShortcut}
                                saveShortcut={setLastTranscriptShortcut}
                                resetShortcut={resetLastTranscriptShortcut}
                                dataTestId="paste-transcript-button"
                                existingShortcuts={othersOf('lastTranscript')}
                            />
                        </SettingsUI.Item>
                        <SettingsUI.Separator />
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title>{t('Cancel recording')}</Typography.Title>
                                <Typography.Paragraph>
                                    {t('Press ')}
                                    <RenderKeys keyString={cancelShortcut} />
                                    {t(' to cancel the current recording.')}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <ShortcutButton
                                keyName={t('Cancel recording')}
                                shortcut={cancelShortcut}
                                saveShortcut={setCancelShortcut}
                                resetShortcut={resetCancelShortcut}
                                dataTestId="cancel-recording-button"
                                existingShortcuts={othersOf('cancel')}
                            />
                        </SettingsUI.Item>
                    </SettingsUI.Container>
                </section>

                <section>
                    <Typography.Title data-testid="voice-mode-title" className="p-2 font-semibold text-sky-400!">
                        {t('Voice Mode')}
                    </Typography.Title>
                    <SettingsUI.Container>
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title>{t('Toggle Voice Mode')}</Typography.Title>
                                <Typography.Paragraph>
                                    {t('Press ')}
                                    <RenderKeys keyString={voiceModeToggleShortcut} />
                                    {t(' to mute or unmute Voice Mode listening.')}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <ShortcutButton
                                keyName={t('Toggle Voice Mode')}
                                shortcut={voiceModeToggleShortcut}
                                saveShortcut={setVoiceModeToggleShortcut}
                                resetShortcut={resetVoiceModeToggleShortcut}
                                dataTestId="voice-mode-toggle-button"
                                existingShortcuts={othersOf('voiceModeToggle')}
                            />
                        </SettingsUI.Item>
                    </SettingsUI.Container>
                </section>

                {llmOnboardingCompleted && (
                    <section>
                        <Typography.Title data-testid="llm-connect-title" className="p-2 font-semibold text-sky-400!">
                            {t('LLM Connect')}
                        </Typography.Title>
                        <SettingsUI.Container className="mb-4">
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>{t('Command')}</Typography.Title>
                                    <Typography.Paragraph>
                                        {t('Press')} <RenderKeys keyString={commandShortcut} />
                                        {t(' to execute a voice command on selected text.')}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <ShortcutButton
                                    keyName={t('Command')}
                                    shortcut={commandShortcut}
                                    saveShortcut={setCommandShortcut}
                                    resetShortcut={resetCommandShortcut}
                                    dataTestId="command-button"
                                    existingShortcuts={othersOf('command')}
                                />
                            </SettingsUI.Item>
                        </SettingsUI.Container>
                        <SettingsUI.Container>
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>{t('Transcribe with LLM')} 1</Typography.Title>
                                    <Typography.Paragraph>
                                        {t('Press')} <RenderKeys keyString={llmMode1Shortcut} />
                                        {t(' to start LLM recording with prompt 1.')}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <ShortcutButton
                                    keyName={`${t('Transcribe with LLM')} 1`}
                                    shortcut={llmMode1Shortcut}
                                    saveShortcut={setLLMMode1Shortcut}
                                    resetShortcut={resetLLMMode1Shortcut}
                                    dataTestId="llm-mode-1-button"
                                    existingShortcuts={othersOf('llmMode1')}
                                />
                            </SettingsUI.Item>
                            <SettingsUI.Separator />
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>{t('Transcribe with LLM')} 2</Typography.Title>
                                    <Typography.Paragraph>
                                        {t('Press')} <RenderKeys keyString={llmMode2Shortcut} />
                                        {t(' to start LLM recording with prompt 2.')}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <ShortcutButton
                                    keyName={`${t('Transcribe with LLM')} 2`}
                                    shortcut={llmMode2Shortcut}
                                    saveShortcut={setLLMMode2Shortcut}
                                    resetShortcut={resetLLMMode2Shortcut}
                                    dataTestId="llm-mode-2-button"
                                    existingShortcuts={othersOf('llmMode2')}
                                />
                            </SettingsUI.Item>
                            <SettingsUI.Separator />
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>{t('Transcribe with LLM')} 3</Typography.Title>
                                    <Typography.Paragraph>
                                        {t('Press')} <RenderKeys keyString={llmMode3Shortcut} />
                                        {t(' to start LLM recording with prompt 3.')}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <ShortcutButton
                                    keyName={`${t('Transcribe with LLM')} 3`}
                                    shortcut={llmMode3Shortcut}
                                    saveShortcut={setLLMMode3Shortcut}
                                    resetShortcut={resetLLMMode3Shortcut}
                                    dataTestId="llm-mode-3-button"
                                    existingShortcuts={othersOf('llmMode3')}
                                />
                            </SettingsUI.Item>
                            <SettingsUI.Separator />
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>{t('Transcribe with LLM')} 4</Typography.Title>
                                    <Typography.Paragraph>
                                        {t('Press')} <RenderKeys keyString={llmMode4Shortcut} />
                                        {t(' to start LLM recording with prompt 4.')}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <ShortcutButton
                                    keyName={`${t('Transcribe with LLM')} 4`}
                                    shortcut={llmMode4Shortcut}
                                    saveShortcut={setLLMMode4Shortcut}
                                    resetShortcut={resetLLMMode4Shortcut}
                                    dataTestId="llm-mode-4-button"
                                    existingShortcuts={othersOf('llmMode4')}
                                />
                            </SettingsUI.Item>
                        </SettingsUI.Container>
                    </section>
                )}
            </div>
        </main>
    );
};
