import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Slider } from '@/components/slider';
import { AudioLines, Mic, Timer, Zap } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { PALETTES } from '@/components/lib/color-scheme';
import { useLlmExtensionsEnabled } from '@/features/extensions/llm-connect/hooks/use-llm-extensions-enabled';
import { ExtensionPageLayout } from '@/features/extensions/extension-page-layout/extension-page-layout';
import { ExtensionHeader } from '@/features/extensions/extension-header/extension-header';
import { useWakeWordEnabled } from './hooks/use-wake-word-enabled';
import { useSilenceTimeout } from './hooks/use-silence-timeout';
import { useWakeWord, WAKE_WORD_CONFIGS } from './hooks/use-wake-word';
import { VoiceTriggerItem } from './voice-trigger-item/voice-trigger-item';
import { VoiceModeCta } from './voice-mode-cta/voice-mode-cta';
import { LlmConnectTriggers } from './llm-connect-triggers/llm-connect-triggers';

export const VoiceMode = () => {
    const { t, i18n } = useTranslation();
    const submitDefaultWord = i18n.language?.startsWith('fr') ? 'merci alix' : 'thank you alix';
    const { commandModeEnabled } = useLlmExtensionsEnabled();
    const { enabled, setEnabled } = useWakeWordEnabled();
    const { silenceTimeoutMs, setSilenceTimeoutMs } = useSilenceTimeout();

    const isLoaded = enabled !== null;

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

    if (!isLoaded) {
        return null;
    }

    if (!enabled) {
        return (
            <ExtensionPageLayout colorScheme="standard" isActive={false}>
                <ExtensionHeader
                    icon={Mic}
                    title={t('Voice Mode')}
                    colorScheme="standard"
                    titleTestId="voice-mode-title"
                    subtitle={t('Keep your hands free while Murmure types.')}
                />
                <VoiceModeCta onEnable={() => setEnabled(true)} recordWord={recordDefault} submitWord={submitDefault} />
            </ExtensionPageLayout>
        );
    }

    return (
        <ExtensionPageLayout colorScheme="standard" isActive>
            <ExtensionHeader
                icon={Mic}
                title={t('Voice Mode')}
                colorScheme="standard"
                titleTestId="voice-mode-title"
                statusLabel={t('Listening')}
                toggle={{
                    checked: enabled,
                    onCheckedChange: setEnabled,
                    ariaLabel: t('Enable Voice Mode'),
                    testId: 'voice-mode-toggle',
                }}
            />

            <SettingsUI.Section title={t('Trigger words')} icon={AudioLines} titleTestId="voice-triggers-title">
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
                {commandModeEnabled && (
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
                            icon={Zap}
                            iconColor={PALETTES.command.accent}
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
            </SettingsUI.Section>

            <LlmConnectTriggers />

            <SettingsUI.Section title={t('Listening behavior')} icon={Timer} titleTestId="behavior-title">
                <SettingsUI.Item>
                    <SettingsUI.Description>
                        <Typography.Title>{t('Silence timeout')}</Typography.Title>
                        <Typography.Paragraph>{t('Stops recording after silence of:')}</Typography.Paragraph>
                    </SettingsUI.Description>
                    <Slider
                        value={[silenceTimeoutMs === 0 ? 5500 : silenceTimeoutMs]}
                        onValueChange={([value]) => setSilenceTimeoutMs(value > 5000 ? 0 : value)}
                        min={500}
                        max={5500}
                        step={100}
                        showValue
                        formatValue={(v) => (v > 5000 ? t('Indefinite') : `${(v / 1000).toFixed(1)}s`)}
                        className="w-28"
                        data-testid="silence-timeout-slider"
                    />
                </SettingsUI.Item>
            </SettingsUI.Section>
        </ExtensionPageLayout>
    );
};
