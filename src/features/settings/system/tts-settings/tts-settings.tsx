import { SettingsUI } from '@/components/settings-ui';
import { Typography } from '@/components/typography';
import { Volume2, Gauge } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/select';
import { Slider } from '@/components/slider';
import { useTranslation } from '@/i18n';
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export type TtsEngine = 'auto' | 'neural' | 'standard';

export const TtsSettings = () => {
    const { t } = useTranslation();
    const [engine, setEngine] = useState<TtsEngine>('auto');
    const [speed, setSpeed] = useState(1.0);

    useEffect(() => {
        const saved = localStorage.getItem('murmure_tts_engine') as TtsEngine;
        if (saved) {
            setEngine(saved);
        }
        
        // Load speed from backend settings
        invoke<number>('get_tts_speed').then((val) => {
            if (val) setSpeed(val);
        });
    }, []);

    const handleEngineChange = (value: TtsEngine) => {
        setEngine(value);
        localStorage.setItem('murmure_tts_engine', value);
        window.dispatchEvent(new Event('murmure_tts_settings_changed'));
    };

    const handleSpeedChange = (value: number) => {
        setSpeed(value);
        invoke('set_tts_speed', { speed: value });
    };

    return (
        <>
            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Volume2 className="w-4 h-4 text-muted-foreground" />
                        {t('Speech Synthesis')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t('Select the engine for text-to-speech output.')}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <Select value={engine} onValueChange={handleEngineChange}>
                    <SelectTrigger className="w-[180px]" data-testid="tts-engine-select">
                        <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="auto">{t('Auto (Default)')}</SelectItem>
                        <SelectItem value="neural">{t('Neural (Kokoro)')}</SelectItem>
                        <SelectItem value="standard">{t('Standard (Browser)')}</SelectItem>
                    </SelectContent>
                </Select>
            </SettingsUI.Item>

            <SettingsUI.Separator />

            <SettingsUI.Item>
                <SettingsUI.Description>
                    <Typography.Title className="flex items-center gap-2">
                        <Gauge className="w-4 h-4 text-muted-foreground" />
                        {t('Speech Speed')}
                    </Typography.Title>
                    <Typography.Paragraph>
                        {t('Adjust the speed of the neural voice.')}
                    </Typography.Paragraph>
                </SettingsUI.Description>
                <div className="flex items-center gap-4 w-[180px]">
                    <Slider
                        value={[speed]}
                        min={0.5}
                        max={2.0}
                        step={0.1}
                        onValueChange={(v) => handleSpeedChange(v[0])}
                        className="flex-1"
                    />
                    <span className="text-sm font-mono w-8 text-right">{speed.toFixed(1)}x</span>
                </div>
            </SettingsUI.Item>
        </>
    );
};
