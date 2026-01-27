import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Activity } from 'lucide-react';
import { useVisualizerSensitivity } from './hooks/use-visualizer-sensitivity';
import { Slider } from '@/components/slider';
import { useTranslation } from '@/i18n';

export function VisualizerSettings() {
    const { sensitivity, setSensitivity, isLoading } = useVisualizerSensitivity();
    const { t } = useTranslation();

    const handleChange = (value: number) => {
        setSensitivity(value);
    };

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Activity className="w-4 h-4 text-zinc-400" />
                    {t('Visualizer Sensitivity')}
                </Typography.Title>
                <Typography.Paragraph>
                    {t('Adjust how responsive the audio visualizer is to sound input.')}
                </Typography.Paragraph>
            </SettingsUI.Description>
            <div className="flex items-center gap-3 min-w-[180px]">
                <Slider
                    value={sensitivity}
                    min={1}
                    max={20}
                    step={0.5}
                    onChange={handleChange}
                    disabled={isLoading}
                    className="flex-1"
                />
                <span className="text-sm text-zinc-400 w-8 text-right">
                    {sensitivity.toFixed(1)}
                </span>
            </div>
        </SettingsUI.Item>
    );
}
