import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { SettingsUI } from '@/components/settings-ui';
import { Switch } from '@/components/switch';
import { Typography } from '@/components/typography';
import { Music } from 'lucide-react';

export function SoundSettings() {
    const [soundEnabled, setSoundEnabled] = useState(true);

    useEffect(() => {
        invoke<boolean>('get_sound_enabled').then(setSoundEnabled);
    }, []);

    const handleToggle = (checked: boolean) => {
        setSoundEnabled(checked);
        invoke('set_sound_enabled', { enabled: checked });
    };

    return (
        <SettingsUI.Item>
            <SettingsUI.Description>
                <Typography.Title className="flex items-center gap-2">
                    <Music className="w-4 h-4 text-zinc-400" />
                    Sound Effects
                </Typography.Title>
                <Typography.Paragraph>
                    Play a sound when recording starts and stops.
                </Typography.Paragraph>
            </SettingsUI.Description>
            <Switch checked={soundEnabled} onCheckedChange={handleToggle} />
        </SettingsUI.Item>
    );
}
