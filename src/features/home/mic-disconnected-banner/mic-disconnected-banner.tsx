import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from '@/i18n';
import { TriangleAlert } from 'lucide-react';
import type { MicInfo } from '@/features/settings/system/mic-settings/hooks/use-mic-state';

export const MicDisconnectedBanner = () => {
    const { t } = useTranslation();
    const [disconnectedMicLabel, setDisconnectedMicLabel] = useState<
        string | null
    >(null);

    useEffect(() => {
        const checkMicStatus = async () => {
            try {
                const [micId, micLabel] = await Promise.all([
                    invoke<string | null>('get_current_mic_id'),
                    invoke<string | null>('get_current_mic_label'),
                ]);
                if (!micId) {
                    setDisconnectedMicLabel(null);
                    return;
                }

                const devices = await invoke<MicInfo[]>('get_mic_list');
                const found = devices.find((d) => d.id === micId);

                if (found) {
                    setDisconnectedMicLabel(null);
                } else {
                    setDisconnectedMicLabel(micLabel ?? micId);
                }
            } catch {
                setDisconnectedMicLabel(null);
            }
        };

        checkMicStatus();
    }, []);

    const switchToAutomatic = async () => {
        try {
            await invoke('set_current_mic_id', {
                micId: null,
                micLabel: null,
            });
            setDisconnectedMicLabel(null);
        } catch (error) {
            console.error('Failed to switch to automatic mic', error);
        }
    };

    if (!disconnectedMicLabel) return null;

    return (
        <div className="flex items-center gap-2 rounded-md bg-destructive/15 border border-destructive/30 px-3 py-2 text-sm text-destructive">
            <TriangleAlert className="w-4 h-4 shrink-0" />
            <span className="flex-1">
                {t('Microphone "{{mic}}" is disconnected.', {
                    mic: disconnectedMicLabel,
                })}
            </span>
            <button
                type="button"
                onClick={() => void switchToAutomatic()}
                className="shrink-0 underline hover:no-underline cursor-pointer"
            >
                {t('Switch to automatic')}
            </button>
        </div>
    );
};
