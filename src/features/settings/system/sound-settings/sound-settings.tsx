import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { SettingsUI } from '@/components/settings-ui';
import { Switch } from '@/components/switch';
import { Typography } from '@/components/typography';
import { Music } from 'lucide-react';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export function SoundSettings() {
    const [soundEnabled, setSoundEnabled] = useState(true);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            setSoundEnabled(settings.sound_enabled);
        });
    }, []);

    const handleToggle = (checked: boolean) => {
        setSoundEnabled(checked);
        invoke('set_sound_enabled', { enabled: checked }).catch(() => {
            toast.error(t('Failed to save sound setting'));
        });
    };

    return (
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
    );
}
