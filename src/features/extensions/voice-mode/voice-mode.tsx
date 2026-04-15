import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { Switch } from '@/components/switch';
import { Slider } from '@/components/slider';
import { Lightbulb, Lock, Mic } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useLlmOnboardingCompleted } from '@/features/extensions/llm-connect/hooks/use-llm-onboarding-completed';
import { useWakeWordEnabled } from './hooks/use-wake-word-enabled';
import { useSilenceTimeout } from './hooks/use-silence-timeout';
import { useWakeWord, WAKE_WORD_CONFIGS } from './hooks/use-wake-word';
import { VoiceTriggerItem } from './voice-trigger-item/voice-trigger-item';
import { VoiceModeCta } from './voice-mode-cta/voice-mode-cta';
import { LlmConnectTriggers } from './llm-connect-triggers/llm-connect-triggers';

export const VoiceMode = () => {
    const { t, i18n } = useTranslation();
    const submitDefaultWord = i18n.language?.startsWith('fr') ? 'merci alix' : 'thank you alix';
    const llmOnboardingCompleted = useLlmOnboardingCompleted();
    const { enabled, setEnabled } = useWakeWordEnabled();
    const { silenceTimeoutMs, setSilenceTimeoutMs } = useSilenceTimeout();

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
    const {
        wakeWord: submitWakeWord,
        setWakeWord: setSubmitWakeWord,
        handleBlur: handleSubmitBlur,
        isEnabled: submitEnabled,
        toggleEnabled: toggleSubmit,
        defaultWord: submitDefault,
        resetToDefault: resetSubmit,
    } = useWakeWord({ ...WAKE_WORD_CONFIGS.submit, defaultWord: submitDefaultWord });

    return (
        <main>
            <div className="space-y-4">
                <Page.Header>
                    <Typography.MainTitle data-testid="voice-mode-title">
                        {t('Voice Mode')}
                    </Typography.MainTitle>
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
                                ? 'border-emerald-400/60 bg-linear-to-r from-cyan-800/40 to-emerald-700/50'
                                : 'border-sky-400/60 bg-linear-to-r from-sky-800/50 to-indigo-800/40'
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

                <div className="flex items-center gap-2 text-xs text-muted-foreground px-2">
                    <Lock className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                    {t(
                        '100% on-device privacy. Listens locally to detect your voice, but audio is never saved or sent anywhere.'
                    )}
                </div>

                <div className="flex items-center gap-2 text-xs text-muted-foreground px-2">
                    <Lightbulb className="w-3.5 h-3.5 text-sky-400 shrink-0" />
                    {t('Tip: For best performance, use Voice Mode in a quiet environment.')}
                </div>

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
                                {llmOnboardingCompleted && (
                                    <>
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
                                    </>
                                )}
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
                                        'Say the trigger word to stop recording and transcribe'
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
                                <SettingsUI.Separator />
                                <VoiceTriggerItem
                                    title={t('Submit')}
                                    description={t(
                                        'Say the trigger word to stop recording, transcribe, and press Enter'
                                    )}
                                    wakeWord={submitWakeWord}
                                    onWakeWordChange={setSubmitWakeWord}
                                    onBlur={handleSubmitBlur}
                                    placeholder="thank you alix"
                                    dataTestId="wake-word-submit-input"
                                    isEnabled={submitEnabled}
                                    onToggleEnabled={toggleSubmit}
                                    defaultWord={submitDefault}
                                    onReset={resetSubmit}
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
                                        <Typography.Title>{t('Silence timeout')}</Typography.Title>
                                        <Typography.Paragraph>
                                            {t(
                                                'Duration of silence before automatically stopping voice-triggered recording.'
                                            )}
                                        </Typography.Paragraph>
                                    </SettingsUI.Description>
                                    <Slider
                                        value={[silenceTimeoutMs === 0 ? 5500 : silenceTimeoutMs]}
                                        onValueChange={([value]) =>
                                            setSilenceTimeoutMs(value > 5000 ? 0 : value)
                                        }
                                        min={500}
                                        max={5500}
                                        step={100}
                                        showValue
                                        formatValue={(v) =>
                                            v > 5000 ? t('Indefinite') : `${(v / 1000).toFixed(1)}s`
                                        }
                                        className="w-28"
                                        data-testid="silence-timeout-slider"
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
