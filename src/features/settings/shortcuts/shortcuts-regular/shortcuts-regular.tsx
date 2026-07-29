import { Lightbulb } from 'lucide-react';
import { Typography } from '@/components/typography';
import { ShortcutButton } from './shortcut-button/shortcut-button';
import { LlmModeShortcuts } from './llm-mode-shortcuts/llm-mode-shortcuts';
import { RenderKeys } from '@/components/render-keys.tsx';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { useShortcut, SHORTCUT_CONFIGS } from '../hooks/use-shortcut';
import { useTranslation } from '@/i18n';
import { useRecordModeState } from '@/features/settings/system/record-mode-settings/hooks/use-record-mode-state';
import { useLlmOnboardingCompleted } from '@/features/extensions/llm-connect/hooks/use-llm-onboarding-completed';
import { useLlmModeNames } from '../hooks/use-llm-mode-names';

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

    const llmMode1 = useShortcut(SHORTCUT_CONFIGS.llmMode1);
    const llmMode2 = useShortcut(SHORTCUT_CONFIGS.llmMode2);
    const llmMode3 = useShortcut(SHORTCUT_CONFIGS.llmMode3);
    const llmMode4 = useShortcut(SHORTCUT_CONFIGS.llmMode4);

    const llmTransform1 = useShortcut(SHORTCUT_CONFIGS.llmTransform1);
    const llmTransform2 = useShortcut(SHORTCUT_CONFIGS.llmTransform2);
    const llmTransform3 = useShortcut(SHORTCUT_CONFIGS.llmTransform3);
    const llmTransform4 = useShortcut(SHORTCUT_CONFIGS.llmTransform4);

    const {
        shortcut: voiceModeToggleShortcut,
        setShortcut: setVoiceModeToggleShortcut,
        resetShortcut: resetVoiceModeToggleShortcut,
    } = useShortcut(SHORTCUT_CONFIGS.voiceModeToggle);

    const llmOnboardingCompleted = useLlmOnboardingCompleted();
    const llmModeNames = useLlmModeNames();
    const visibleModeCount = llmModeNames.length > 0 ? llmModeNames.length : 1;

    const llmModeGroups = [
        { dictateId: 'llmMode1', transformId: 'llmTransform1', dictate: llmMode1, transform: llmTransform1 },
        { dictateId: 'llmMode2', transformId: 'llmTransform2', dictate: llmMode2, transform: llmTransform2 },
        { dictateId: 'llmMode3', transformId: 'llmTransform3', dictate: llmMode3, transform: llmTransform3 },
        { dictateId: 'llmMode4', transformId: 'llmTransform4', dictate: llmMode4, transform: llmTransform4 },
    ];
    const visibleModeGroups = llmModeGroups.slice(0, visibleModeCount);

    const modeName = (index: number) => {
        const name = llmModeNames[index];
        return name != null && name.length > 0 ? name : `LLM ${index + 1}`;
    };

    const dictateLabel = (index: number) => t('Dictate with {{mode}}', { mode: modeName(index) });
    const transformLabel = (index: number) => t('Transform with {{mode}}', { mode: modeName(index) });

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
        { id: 'command', name: t('Command, free prompt'), value: commandShortcut },
        { id: 'llmMode1', name: dictateLabel(0), value: llmMode1.shortcut },
        { id: 'llmMode2', name: dictateLabel(1), value: llmMode2.shortcut },
        { id: 'llmMode3', name: dictateLabel(2), value: llmMode3.shortcut },
        { id: 'llmMode4', name: dictateLabel(3), value: llmMode4.shortcut },
        { id: 'llmTransform1', name: transformLabel(0), value: llmTransform1.shortcut },
        { id: 'llmTransform2', name: transformLabel(1), value: llmTransform2.shortcut },
        { id: 'llmTransform3', name: transformLabel(2), value: llmTransform3.shortcut },
        { id: 'llmTransform4', name: transformLabel(3), value: llmTransform4.shortcut },
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
                                    <Typography.Title>{t('Command, free prompt')}</Typography.Title>
                                    <Typography.Paragraph>
                                        {t('Press')} <RenderKeys keyString={commandShortcut} />
                                        {t(' to execute a voice command on selected text.')}
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <ShortcutButton
                                    keyName={t('Command, free prompt')}
                                    shortcut={commandShortcut}
                                    saveShortcut={setCommandShortcut}
                                    resetShortcut={resetCommandShortcut}
                                    dataTestId="command-button"
                                    existingShortcuts={othersOf('command')}
                                />
                            </SettingsUI.Item>
                        </SettingsUI.Container>
                        {visibleModeGroups.map(({ dictateId, transformId, dictate, transform }, index) => (
                            <LlmModeShortcuts
                                key={dictateId}
                                className={index < visibleModeGroups.length - 1 ? 'mb-4' : undefined}
                                dictate={{
                                    ...dictate,
                                    label: dictateLabel(index),
                                    dataTestId: `llm-mode-${index + 1}-button`,
                                    existingShortcuts: othersOf(dictateId),
                                }}
                                transform={{
                                    ...transform,
                                    label: transformLabel(index),
                                    dataTestId: `llm-transform-${index + 1}-button`,
                                    existingShortcuts: othersOf(transformId),
                                }}
                            />
                        ))}
                    </section>
                )}
            </div>
        </main>
    );
};
