import { Typography } from '@/components/typography';
import { ShortcutButton } from './shortcut-button/shortcut-button';
import { RenderKeys } from '../../../components/render-keys';
import { SettingsUI } from '@/components/settings-ui';
import { useRecordShortcutState } from './hooks/use-record-shortcut-state';
import { Page } from '@/components/page';
import { useLastTranscriptShortcutState } from './hooks/use-last_transcript-shortcut-state';
import { useKeyboardModeState } from './hooks/use-keyboard-mode-state';
import { useStartRecordingShortcutState } from './hooks/use-start-recording-shortcut-state';
import { useStopRecordingShortcutState } from './hooks/use-stop-recording-shortcut-state';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/select';

interface ShortcutsProps {}

export const Shortcuts = ({}: ShortcutsProps) => {
    const { recordShortcut, setRecordShortcut, resetRecordShortcut } =
        useRecordShortcutState();
    const {
        lastTranscriptShortcut,
        setLastTranscriptShortcut,
        resetLastTranscriptShortcut,
    } = useLastTranscriptShortcutState();
    const { keyboardMode, setKeyboardMode } = useKeyboardModeState();
    const {
        startRecordingShortcut,
        setStartRecordingShortcut,
        resetStartRecordingShortcut,
    } = useStartRecordingShortcutState();
    const {
        stopRecordingShortcut,
        setStopRecordingShortcut,
        resetStopRecordingShortcut,
    } = useStopRecordingShortcutState();

    return (
        <main>
            <div className="space-y-8">
                <Page.Header>
                    <Typography.MainTitle>Shortcuts</Typography.MainTitle>
                    <Typography.Paragraph className="text-zinc-400">
                        Improve your workflow by setting up keyboard shortcuts.
                    </Typography.Paragraph>
                </Page.Header>

                <SettingsUI.Container>
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>Keyboard Mode</Typography.Title>
                            <Typography.Paragraph>
                                Choose how you want to control recording with
                                keyboard shortcuts.
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <Select
                            value={keyboardMode}
                            onValueChange={(value) =>
                                setKeyboardMode(
                                    value as 'push-to-talk' | 'toggle'
                                )
                            }
                        >
                            <SelectTrigger className="w-48">
                                <SelectValue placeholder="Select mode" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="push-to-talk">
                                    Push to Talk
                                </SelectItem>
                                <SelectItem value="toggle">Toggle</SelectItem>
                            </SelectContent>
                        </Select>
                    </SettingsUI.Item>
                    <SettingsUI.Separator />

                    {keyboardMode === 'push-to-talk' && (
                        <>
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>
                                        Push to talk
                                    </Typography.Title>
                                    <Typography.Paragraph>
                                        Hold{' '}
                                        <RenderKeys
                                            keyString={recordShortcut}
                                        />{' '}
                                        to record, release to transcribe.
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <ShortcutButton
                                    keyName="Push to talk"
                                    shortcut={recordShortcut}
                                    saveShortcut={setRecordShortcut}
                                    resetShortcut={resetRecordShortcut}
                                />
                            </SettingsUI.Item>
                            <SettingsUI.Separator />
                        </>
                    )}

                    {keyboardMode === 'toggle' && (
                        <>
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>
                                        Start Recording
                                    </Typography.Title>
                                    <Typography.Paragraph>
                                        Press{' '}
                                        {startRecordingShortcut ? (
                                            <RenderKeys
                                                keyString={
                                                    startRecordingShortcut
                                                }
                                            />
                                        ) : (
                                            <span className="text-zinc-500">
                                                Not configured
                                            </span>
                                        )}{' '}
                                        to start recording.
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <ShortcutButton
                                    keyName="Start Recording"
                                    shortcut={startRecordingShortcut || ''}
                                    saveShortcut={setStartRecordingShortcut}
                                    resetShortcut={resetStartRecordingShortcut}
                                />
                            </SettingsUI.Item>
                            <SettingsUI.Separator />
                            <SettingsUI.Item>
                                <SettingsUI.Description>
                                    <Typography.Title>
                                        Stop Recording
                                    </Typography.Title>
                                    <Typography.Paragraph>
                                        Press{' '}
                                        {stopRecordingShortcut ? (
                                            <RenderKeys
                                                keyString={
                                                    stopRecordingShortcut
                                                }
                                            />
                                        ) : (
                                            <span className="text-zinc-500">
                                                Not configured
                                            </span>
                                        )}{' '}
                                        to stop recording.
                                    </Typography.Paragraph>
                                </SettingsUI.Description>
                                <ShortcutButton
                                    keyName="Stop Recording"
                                    shortcut={stopRecordingShortcut || ''}
                                    saveShortcut={setStopRecordingShortcut}
                                    resetShortcut={resetStopRecordingShortcut}
                                />
                            </SettingsUI.Item>
                            <SettingsUI.Separator />
                        </>
                    )}

                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title>
                                Paste last transcript
                            </Typography.Title>
                            <Typography.Paragraph>
                                Press{' '}
                                <RenderKeys
                                    keyString={lastTranscriptShortcut}
                                />{' '}
                                to paste the last transcript. Useful when you
                                forgot to select an input field when you started
                                recording.
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <ShortcutButton
                            keyName="Paste last transcript"
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
