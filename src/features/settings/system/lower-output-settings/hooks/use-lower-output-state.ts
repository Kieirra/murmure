import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

export const useLowerOutputState = () => {
    const [lowerOutput, setLowerOutput] = useState<boolean>(false);
    const [volumePercent, setVolumePercent] = useState<number>(50);
    const [unsupportedReason, setUnsupportedReason] = useState<string | null>(null);
    const { t } = useTranslation();
    const showSaveError = () => toast.error(t('Failed to save audio setting'));

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            if (typeof settings.lower_output_while_recording === 'boolean') {
                setLowerOutput(settings.lower_output_while_recording);
            }
            if (typeof settings.output_volume_while_recording === 'number') {
                setVolumePercent(settings.output_volume_while_recording);
            }
        });
        invoke<string | null>('get_output_volume_unsupported_reason')
            .then(setUnsupportedReason)
            .catch(() => setUnsupportedReason('unsupported_platform'));
    }, []);

    const handleToggle = (enabled: boolean) => {
        setLowerOutput(enabled);
        invoke('set_lower_output_while_recording', { enabled }).catch(() => {
            showSaveError();
            setLowerOutput(!enabled);
        });
    };

    const handleVolumeChange = (percent: number) => {
        setVolumePercent(percent);
        invoke('set_output_volume_while_recording', { percent }).catch(showSaveError);
    };

    return {
        lowerOutput,
        volumePercent,
        unsupportedReason,
        handleToggle,
        handleVolumeChange,
    };
};
