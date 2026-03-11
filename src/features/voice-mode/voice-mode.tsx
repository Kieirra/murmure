import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { Switch } from '@/components/switch';
import { Mic } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useWakeWordEnabled } from './hooks/use-wake-word-enabled';
import { useAutoEnter } from './hooks/use-auto-enter';
import { useWakeWord, WAKE_WORD_CONFIGS } from './hooks/use-wake-word';
import { VoiceTriggerItem } from './voice-trigger-item/voice-trigger-item';
import { VoiceModeCta } from './voice-mode-cta/voice-mode-cta';
import { LlmConnectTriggers } from './llm-connect-triggers/llm-connect-triggers';

export const VoiceMode = () => {
    const { t } = useTranslation();
    const { enabled, setEnabled } = useWakeWordEnabled();
    const { autoEnter, setAutoEnter } = useAutoEnter();

    const {
        wakeWord: recordWakeWord,
        setWakeWord: setRecordWakeWord,
        handleBlur: handleRecordBlur,
        isEnabled: recordEnabled,
        toggleEnabled: toggleRecord,
        defaultWord: recordDefault,
        resetToDefault: resetRecord,
    } = useWakeWord(WAKE_WORD_CONFIGS.record);
    const {
        wakeWord: commandWakeWord,
        setWakeWord: setCommandWakeWord,
        handleBlur: handleCommandBlur,
        isEnabled: commandEnabled,
        toggleEnabled: toggleCommand,
        defaultWord: commandDefault,
        resetToDefault: resetCommand,
    } = useWakeWord(WAKE_WORD_CONFIGS.command);
    const {
        wakeWord: cancelWakeWord,
        setWakeWord: setCancelWakeWord,
        handleBlur: handleCancelBlur,
        isEnabled: cancelEnabled,
        toggleEnabled: toggleCancel,
        defaultWord: cancelDefault,
        resetToDefault: resetCancel,
    } = useWakeWord(WAKE_WORD_CONFIGS.cancel);
    const {
        wakeWord: validateWakeWord,
        setWakeWord: setValidateWakeWord,
        handleBlur: handleValidateBlur,
        isEnabled: validateEnabled,
        toggleEnabled: toggleValidate,
        defaultWord: validateDefault,
        resetToDefault: resetValidate,
    } = useWakeWord(WAKE_WORD_CONFIGS.validate);

    return (
        <main>
            <div className="space-y-6">
                <Page.Header>
                    <Typography.MainTitle data-testid="voice-mode-title">{t('Voice Mode')}</Typography.MainTitle>
                    <Typography.Paragraph className="text-muted-foreground">
                        {t(
                            'Control Murmure without touching your keyboard. Say a trigger word and let the magic happen.'
                        )}
                    </Typography.Paragraph>
                </Page.Header>

                <section>
                    <SettingsUI.Container
                        className={
                            enabled
                                ? 'border-emerald-400/60 bg-gradient-to-r from-cyan-800/40 to-emerald-700/50'
                                : 'border-sky-400/60 bg-gradient-to-r from-sky-800/50 to-indigo-800/40'
                        }
                    >
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title className="flex items-center gap-2">
                                    <Mic className="w-4 h-4 text-muted-foreground" />
                                    {t('Enable Voice Mode')}
                                </Typography.Title>
                                <Typography.Paragraph>
                                    {t('Listens for your trigger words using voice activity detection (VAD).')}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch checked={enabled} onCheckedChange={setEnabled} data-testid="voice-mode-toggle" />
                        </SettingsUI.Item>
                    </SettingsUI.Container>
                </section>

                {enabled ? (
                    <>
                        <section>
                            <Typography.Title
                                data-testid="voice-triggers-title"
                                className="p-2 font-semibold text-sky-400!"
                            >
                                {t('Voice Triggers')}
                            </Typography.Title>
                            <SettingsUI.Container>
                                <VoiceTriggerItem
                                    title={t('Transcription')}
                                    description={t('Say the trigger word to start recording')}
                                    wakeWord={recordWakeWord}
                                    onWakeWordChange={setRecordWakeWord}
                                    onBlur={handleRecordBlur}
                                    placeholder="ok alix"
                                    dataTestId="wake-word-record-input"
                                    isEnabled={recordEnabled}
                                    onToggleEnabled={toggleRecord}
                                    defaultWord={recordDefault}
                                    onReset={resetRecord}
                                />
                                <SettingsUI.Separator />
                                <VoiceTriggerItem
                                    title={t('Command')}
                                    description={t('Say the trigger word for voice commands')}
                                    wakeWord={commandWakeWord}
                                    onWakeWordChange={setCommandWakeWord}
                                    onBlur={handleCommandBlur}
                                    placeholder="alix command"
                                    dataTestId="wake-word-command-input"
                                    isEnabled={commandEnabled}
                                    onToggleEnabled={toggleCommand}
                                    defaultWord={commandDefault}
                                    onReset={resetCommand}
                                />
                                <SettingsUI.Separator />
                                <VoiceTriggerItem
                                    title={t('Cancel')}
                                    description={t('Say the trigger word to cancel the current recording')}
                                    wakeWord={cancelWakeWord}
                                    onWakeWordChange={setCancelWakeWord}
                                    onBlur={handleCancelBlur}
                                    placeholder="alix cancel"
                                    dataTestId="wake-word-cancel-input"
                                    isEnabled={cancelEnabled}
                                    onToggleEnabled={toggleCancel}
                                    defaultWord={cancelDefault}
                                    onReset={resetCancel}
                                />
                                <SettingsUI.Separator />
                                <VoiceTriggerItem
                                    title={t('Validate')}
                                    description={t(
                                        'Say the trigger word to stop recording, transcribe, and press Enter'
                                    )}
                                    wakeWord={validateWakeWord}
                                    onWakeWordChange={setValidateWakeWord}
                                    onBlur={handleValidateBlur}
                                    placeholder="alix validate"
                                    dataTestId="wake-word-validate-input"
                                    isEnabled={validateEnabled}
                                    onToggleEnabled={toggleValidate}
                                    defaultWord={validateDefault}
                                    onReset={resetValidate}
                                />
                            </SettingsUI.Container>
                        </section>

                        <LlmConnectTriggers />

                        <section>
                            <Typography.Title data-testid="behavior-title" className="p-2 font-semibold text-sky-400!">
                                {t('Behavior')}
                            </Typography.Title>
                            <SettingsUI.Container>
                                <SettingsUI.Item>
                                    <SettingsUI.Description>
                                        <Typography.Title>{t('Auto-press Enter')}</Typography.Title>
                                        <Typography.Paragraph>
                                            {t(
                                                'Automatically press Enter after pasting the transcription. Useful for chat apps and search bars.'
                                            )}
                                        </Typography.Paragraph>
                                    </SettingsUI.Description>
                                    <Switch
                                        checked={autoEnter}
                                        onCheckedChange={setAutoEnter}
                                        data-testid="auto-enter-toggle"
                                    />
                                </SettingsUI.Item>
                            </SettingsUI.Container>
                        </section>
                    </>
                ) : (
                    <VoiceModeCta />
                )}
            </div>
        </main>
    );
};
