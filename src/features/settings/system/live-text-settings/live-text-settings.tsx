import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Switch } from '@/components/switch';
import { Slider } from '@/components/slider';
import { AudioLines, Timer } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { useLiveTextState } from './hooks/use-live-text-state';

export const LiveTextSettings = () => {
    const { longDictationEnabled, setLongDictationEnabled, longDictationSilenceMs, setLongDictationSilenceMs } =
        useLiveTextState();
    const { t } = useTranslation();

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <AudioLines className="w-4 h-4 text-muted-foreground" />
                        {t('Live Text Mode')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t(
                            'Your words appear as you speak, written straight into the app. Turn off to insert the full text only at the end.'
                        )}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <Switch checked={longDictationEnabled} onCheckedChange={setLongDictationEnabled} />
            </SettingsUI.Item>
            {longDictationEnabled && (
                <>
                    <SettingsUI.Separator />
                    <SettingsUI.Item>
                        <SettingsUI.Description>
                            <Typography.Title className="flex items-center gap-2">
                                <Timer className="w-4 h-4 text-muted-foreground" />
                                {t('Pause length')}
                            </Typography.Title>
                            <Typography.Paragraph>
                                {t('How long to wait after you stop talking before the text is written')}
                            </Typography.Paragraph>
                        </SettingsUI.Description>
                        <Slider
                            value={[longDictationSilenceMs]}
                            onValueChange={([value]) => setLongDictationSilenceMs(value)}
                            min={250}
                            max={3000}
                            step={50}
                            showValue
                            formatValue={(v) => `${v}ms`}
                            className="w-[180px]"
                        />
                    </SettingsUI.Item>
                </>
            )}
        </>
    );
};
