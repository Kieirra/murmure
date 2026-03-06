import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from '@/i18n';
import { TriangleAlert } from 'lucide-react';

interface MicInfo {
    id: string;
    label: string;
}

export const MicDisconnectedBanner = () => {
    const { t } = useTranslation();
    const [disconnectedMicLabel, setDisconnectedMicLabel] = useState<
        string | null
    >(null);

    useEffect(() => {
        async function checkMicStatus() {
            try {
                const micId = await invoke<string | null>(
                    'get_current_mic_id'
                );
                if (!micId) {
                    setDisconnectedMicLabel(null);
                    return;
                }

                const devices = await invoke<MicInfo[]>('get_mic_list');
                const found = devices.find((d) => d.id === micId);

                if (found) {
                    setDisconnectedMicLabel(null);
                } else {
                    setDisconnectedMicLabel(
                        devices.find((d) => d.label === micId)?.label ?? micId
                    );
                }
            } catch {
                setDisconnectedMicLabel(null);
            }
        }

        checkMicStatus();
    }, []);

    if (!disconnectedMicLabel) return null;

    return (
        <div className="flex items-center gap-2 rounded-md bg-destructive/15 border border-destructive/30 px-3 py-2 text-sm text-destructive">
            <TriangleAlert className="w-4 h-4 shrink-0" />
            <span>
                {t(
                    'Microphone "{{mic}}" is disconnected.',
                    { mic: disconnectedMicLabel }
                )}
            </span>
        </div>
    );
};
