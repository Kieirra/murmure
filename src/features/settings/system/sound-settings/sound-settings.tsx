import { SettingsUI } from '@/components/settings-ui';
import { Slider } from '@/components/slider';
import { Switch } from '@/components/switch';
import { Typography } from '@/components/typography';
import { Music, Music2 } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useSoundSettingsState } from './hooks/use-sound-settings-state';

export const SoundSettings = () => {
    const { t } = useTranslation();
    const { soundEnabled, soundVolume, handleToggle, handleVolumeChange } = useSoundSettingsState();

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
                </>
            )}
        </>
    );
};
