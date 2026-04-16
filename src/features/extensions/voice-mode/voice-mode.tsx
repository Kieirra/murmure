import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { Switch } from '@/components/switch';
import { Slider } from '@/components/slider';
import { Mic } from 'lucide-react';
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
                        {t('Keep your hands free while Murmure types.')}
                    </Typography.Paragraph>
                </Page.Header>

                {enabled ? (
                    <>
                        <section>
                            <SettingsUI.Container className="border-emerald-400/40 bg-linear-to-r from-emerald-900/20 to-transparent">
                                <SettingsUI.Item>
                                    <SettingsUI.Description>
                                        <Typography.Title className="flex items-center gap-2">
                                            <Mic className="w-4 h-4 text-emerald-400" />
                                            {t('Voice Mode is listening')}
                                        </Typography.Title>
                                    </SettingsUI.Description>
                                    <Switch
                                        checked={enabled}
                                        onCheckedChange={setEnabled}
                                        data-testid="voice-mode-toggle"
                                    />
                                </SettingsUI.Item>
                            </SettingsUI.Container>
                        </section>

                        <section>
                            <Typography.Title
                                data-testid="voice-triggers-title"
                                className="p-2 font-semibold text-sky-400!"
                            >
                                {t('Trigger words')}
                            </Typography.Title>
                            <SettingsUI.Container>
                                <VoiceTriggerItem
                                    title={t('Transcription')}
                                    description={t('Start recording')}
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
                                            description={t('Run a voice command')}
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
                                    description={t('Cancel current recording')}
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
                                    description={t('Stop and transcribe')}
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
                                    description={t('Transcribe and send')}
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
                                {t('Listening behavior')}
                            </Typography.Title>
                            <SettingsUI.Container>
                                <SettingsUI.Item>
                                    <SettingsUI.Description>
                                        <Typography.Title>{t('Silence timeout')}</Typography.Title>
                                        <Typography.Paragraph>
                                            {t('Stops recording after silence of:')}
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
                    <VoiceModeCta onEnable={() => setEnabled(true)} />
                )}
            </div>
        </main>
    );
};
