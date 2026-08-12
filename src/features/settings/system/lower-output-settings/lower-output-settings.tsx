import { SettingsUI } from '@/components/settings-ui';
import { Slider } from '@/components/slider';
import { Switch } from '@/components/switch';
import { Typography } from '@/components/typography';
import { Volume1, Volume2 } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { UNSUPPORTED_REASONS } from './lower-output-settings.helpers';
import { useLowerOutputState } from './hooks/use-lower-output-state';

export const LowerOutputSettings = () => {
    const { t } = useTranslation();
    const { lowerOutput, volumePercent, unsupportedReason, handleToggle, handleVolumeChange } =
        useLowerOutputState();

    const isSupported = unsupportedReason === null;

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Volume1 className="w-4 h-4 text-muted-foreground" />
                        {t('Lower other audio while recording')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t(
                            'Turns your speakers down while you dictate, so the microphone picks up your voice and not what is playing. The volume is restored when recording stops.'
                        )}
                    </Typography.Paragraph>
                    {!isSupported ? (
                        <p className="text-xs text-yellow-400">
                            {t(
                                UNSUPPORTED_REASONS[unsupportedReason] ??
                                    UNSUPPORTED_REASONS.unsupported_platform
                            )}
                        </p>
                    ) : null}
                </SettingsUI.Description>
                <Switch
                    checked={lowerOutput && isSupported}
                    onCheckedChange={handleToggle}
                    disabled={!isSupported}
                    data-testid="lower-output-switch"
                />
            </SettingsUI.Item>
            {lowerOutput && isSupported && (
                <>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title className="flex items-center gap-2">
                                <Volume2 className="w-4 h-4 text-muted-foreground" />
                                {t('Volume while recording')}
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('How much of your current volume is kept while you dictate.')}
                            </Typography.Paragraph>
                            {volumePercent === 0 ? (
                                <p className="text-xs text-yellow-400">
                                    {t("Murmure's own start and stop sounds will also be silent.")}
                                </p>
                            ) : null}
                        </SettingsUI.Description>
                        <Slider
                            value={[volumePercent]}
                            onValueChange={([percent]) => handleVolumeChange(percent)}
                            min={0}
                            max={80}
                            step={10}
                            showValue
                            formatValue={(percent) => (percent === 0 ? t('Muted') : `${percent}%`)}
                            className="w-[180px]"
                            data-testid="output-volume-slider"
                        />
                    </SettingsUI.Item>
                </>
            )}
        </>
    );
};
