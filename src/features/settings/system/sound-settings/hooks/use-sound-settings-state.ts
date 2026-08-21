import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useSoundSettingsState = () => {
    const [soundEnabled, setSoundEnabled] = useState<boolean>(true);
    const [soundVolume, setSoundVolume] = useState<number>(80);
    const { t } = useTranslation();
    const showSaveError = () => toast.error(t('Failed to save sound setting'));

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            setSoundEnabled(settings.sound_enabled);
            if (typeof settings.sound_volume === 'number') {
                setSoundVolume(settings.sound_volume);
            }
        });
    }, []);

    const handleToggle = (checked: boolean) => {
        setSoundEnabled(checked);
        invoke('set_sound_enabled', { enabled: checked }).catch(() => {
            showSaveError();
            setSoundEnabled(!checked);
        });
    };

    const handleVolumeChange = (percent: number) => {
        setSoundVolume(percent);
        invoke('set_sound_volume', { percent }).catch(showSaveError);
    };

    return { soundEnabled, soundVolume, handleToggle, handleVolumeChange };
};
