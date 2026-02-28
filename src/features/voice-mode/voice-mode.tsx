import { Typography } from '@/components/typography';
import { SettingsUI } from '@/components/settings-ui';
import { Page } from '@/components/page';
import { Switch } from '@/components/switch';
import { Mic, Shield, AudioWaveform, Send } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useWakeWordEnabled } from './hooks/use-wake-word-enabled';
import { useAutoEnter } from './hooks/use-auto-enter';
import { useWakeWord, WAKE_WORD_CONFIGS } from './hooks/use-wake-word';
import { VoiceTriggerItem } from './voice-trigger-item/voice-trigger-item';

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
        wakeWord: llmWakeWord,
        setWakeWord: setLlmWakeWord,
        handleBlur: handleLlmBlur,
        isEnabled: llmEnabled,
        toggleEnabled: toggleLlm,
        defaultWord: llmDefault,
        resetToDefault: resetLlm,
    } = useWakeWord(WAKE_WORD_CONFIGS.llm);
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
                    <Typography.MainTitle
                        data-testid="voice-mode-title"
                        className="flex items-center gap-2"
                    >
                        {t('Voice Mode')}
                        <code className="text-amber-300 text-[10px]">
                            {t('Experimental')}
                        </code>
                    </Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        {t(
                            'Control Murmure without touching your keyboard. Say a trigger word and start recording instantly.'
                        )}
                    </Typography.Paragraph>
                </Page.Header>

                <section>
                    <SettingsUI.Container>
                        <SettingsUI.Item>
                            <SettingsUI.Description>
                                <Typography.Title className="flex items-center gap-2">
                                    <Mic className="w-4 h-4 text-zinc-400" />
                                    {t('Enable Voice Mode')}
                                </Typography.Title>
                                <Typography.Paragraph>
                                    {t(
                                        'Listens for your trigger words using voice activity detection (VAD).'
                                    )}
                                </Typography.Paragraph>
                            </SettingsUI.Description>
                            <Switch
                                checked={enabled}
                                onCheckedChange={setEnabled}
                                data-testid="voice-mode-toggle"
                            />
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
                                    description={t(
                                        'Say the trigger word to start recording'
                                    )}
                                    wakeWord={recordWakeWord}
                                    onWakeWordChange={setRecordWakeWord}
                                    onBlur={handleRecordBlur}
                                    placeholder="ok murmure"
                                    disabled={false}
                                    dataTestId="wake-word-record-input"
                                    isEnabled={recordEnabled}
                                    onToggleEnabled={toggleRecord}
                                    defaultWord={recordDefault}
                                    onReset={resetRecord}
                                />
                                <SettingsUI.Separator />
                                <VoiceTriggerItem
                                    title={t('LLM Connect')}
                                    description={t(
                                        'Say the trigger word to record with LLM'
                                    )}
                                    wakeWord={llmWakeWord}
                                    onWakeWordChange={setLlmWakeWord}
                                    onBlur={handleLlmBlur}
                                    placeholder="alix"
                                    disabled={false}
                                    dataTestId="wake-word-llm-input"
                                    isEnabled={llmEnabled}
                                    onToggleEnabled={toggleLlm}
                                    defaultWord={llmDefault}
                                    onReset={resetLlm}
                                />
                                <SettingsUI.Separator />
                                <VoiceTriggerItem
                                    title={t('Command')}
                                    description={t(
                                        'Say the trigger word for voice commands'
                                    )}
                                    wakeWord={commandWakeWord}
                                    onWakeWordChange={setCommandWakeWord}
                                    onBlur={handleCommandBlur}
                                    placeholder="commande"
                                    disabled={false}
                                    dataTestId="wake-word-command-input"
                                    isEnabled={commandEnabled}
                                    onToggleEnabled={toggleCommand}
                                    defaultWord={commandDefault}
                                    onReset={resetCommand}
                                />
                                <SettingsUI.Separator />
                                <VoiceTriggerItem
                                    title={t('Cancel')}
                                    description={t(
                                        'Say the trigger word to cancel the current recording'
                                    )}
                                    wakeWord={cancelWakeWord}
                                    onWakeWordChange={setCancelWakeWord}
                                    onBlur={handleCancelBlur}
                                    placeholder="cancel"
                                    disabled={false}
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
                                        'Say the trigger word to press Enter'
                                    )}
                                    wakeWord={validateWakeWord}
                                    onWakeWordChange={setValidateWakeWord}
                                    onBlur={handleValidateBlur}
                                    placeholder="validate"
                                    disabled={false}
                                    dataTestId="wake-word-validate-input"
                                    isEnabled={validateEnabled}
                                    onToggleEnabled={toggleValidate}
                                    defaultWord={validateDefault}
                                    onReset={resetValidate}
                                />
                            </SettingsUI.Container>
                        </section>

                        <section>
                            <Typography.Title
                                data-testid="behavior-title"
                                className="p-2 font-semibold text-sky-400!"
                            >
                                {t('Behavior')}
                            </Typography.Title>
                            <SettingsUI.Container>
                                <SettingsUI.Item>
                                    <SettingsUI.Description>
                                        <Typography.Title>
                                            {t('Auto-press Enter')}
                                        </Typography.Title>
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
                    <section data-testid="voice-mode-cta">
                        <div className="space-y-4">
                            <div className="bg-sky-950/20 border border-sky-900/30 p-5 rounded-xl flex items-start gap-4">
                                <div className="bg-sky-950 p-2.5 rounded-lg shrink-0">
                                    <Mic className="w-5 h-5 text-sky-400" />
                                </div>
                                <div className="space-y-1">
                                    <h3 className="font-semibold text-zinc-100 text-sm">
                                        {t('Hands-free recording')}
                                    </h3>
                                    <p className="text-sm text-zinc-400 leading-relaxed">
                                        {t(
                                            'Say a trigger word and start recording instantly, no keyboard needed.'
                                        )}
                                    </p>
                                </div>
                            </div>

                            <div className="bg-emerald-950/20 border border-emerald-900/30 p-5 rounded-xl flex items-start gap-4">
                                <div className="bg-emerald-950 p-2.5 rounded-lg shrink-0">
                                    <Shield className="w-5 h-5 text-emerald-400" />
                                </div>
                                <div className="space-y-1">
                                    <h3 className="font-semibold text-zinc-100 text-sm">
                                        {t('Privacy safe')}
                                    </h3>
                                    <p className="text-sm text-zinc-400 leading-relaxed">
                                        {t(
                                            'Only short audio buffers are analyzed in memory, then immediately discarded. No audio is ever recorded or stored.'
                                        )}
                                    </p>
                                </div>
                            </div>

                            <div className="bg-violet-950/20 border border-violet-900/30 p-5 rounded-xl flex items-start gap-4">
                                <div className="bg-violet-950 p-2.5 rounded-lg shrink-0">
                                    <AudioWaveform className="w-5 h-5 text-violet-400" />
                                </div>
                                <div className="space-y-1">
                                    <h3 className="font-semibold text-zinc-100 text-sm">
                                        {t('Fully customizable')}
                                    </h3>
                                    <p className="text-sm text-zinc-400 leading-relaxed">
                                        {t(
                                            'Choose a different trigger word for transcription, LLM processing, and voice commands.'
                                        )}
                                    </p>
                                </div>
                            </div>

                            <div className="bg-amber-950/20 border border-amber-900/30 p-5 rounded-xl flex items-start gap-4">
                                <div className="bg-amber-950 p-2.5 rounded-lg shrink-0">
                                    <Send className="w-5 h-5 text-amber-400" />
                                </div>
                                <div className="space-y-1">
                                    <h3 className="font-semibold text-zinc-100 text-sm">
                                        {t('Auto-send ready')}
                                    </h3>
                                    <p className="text-sm text-zinc-400 leading-relaxed">
                                        {t(
                                            'Enable auto-press Enter and your transcriptions go straight into the conversation. Perfect for Claude Code, ChatGPT, or any chat app.'
                                        )}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </section>
                )}
            </div>
        </main>
    );
};
