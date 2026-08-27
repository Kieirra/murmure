import { SettingsUI } from '@/components/settings-ui';
import { Slider } from '@/components/slider';
import { Switch } from '@/components/switch';
import { Typography } from '@/components/typography';
import { Music, Music2, Timer } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useSoundSettingsState } from './hooks/use-sound-settings-state';

export const SoundSettings = () => {
    const { t } = useTranslation();
    const {
        soundEnabled,
        soundVolume,
        releaseDelayMs,
        handleToggle,
        handleVolumeChange,
        handleReleaseDelayChange,
    } = useSoundSettingsState();

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Music className="w-4 h-4 text-muted-foreground" />
                        {t('Sound Effects')}
                    </Typography.Title>
                    <Typography.Paragraph>{t('Play a sound when recording starts and stops.')}</Typography.Paragraph>
                </SettingsUI.Description>
                <Switch checked={soundEnabled} onCheckedChange={handleToggle} />
            </SettingsUI.Item>
            {soundEnabled && (
                <>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title className="flex items-center gap-2">
                                <Music2 className="w-4 h-4 text-muted-foreground" />
                                {t('Sound effects volume')}
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('How loud the start and stop sounds are.')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <Slider
                            value={[soundVolume]}
                            onValueChange={([percent]) => handleVolumeChange(percent)}
                            min={10}
                            max={100}
                            step={10}
                            showValue
                            formatValue={(percent) => `${percent}%`}
                            className="w-[180px]"
                            data-testid="sound-volume-slider"
                        />
                    </SettingsUI.Item>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title className="flex items-center gap-2">
                                <Timer className="w-4 h-4 text-muted-foreground" />
                                {t('Audio device release delay')}
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t(
                                    'How long the audio output stays open after the last sound. A short delay lets a Bluetooth device go back to sleep sooner. A long one keeps it ready, which avoids a slow device having to wake up before every beep.'
                                )}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <Slider
                            value={[releaseDelayMs / 1000]}
                            onValueChange={([seconds]) => handleReleaseDelayChange(seconds * 1000)}
                            min={2}
                            max={60}
                            step={1}
                            showValue
                            formatValue={(seconds) => `${seconds}s`}
                            className="w-[180px]"
                            data-testid="output-release-delay-slider"
                        />
                    </SettingsUI.Item>
                </>
            )}
        </>
    );
};
