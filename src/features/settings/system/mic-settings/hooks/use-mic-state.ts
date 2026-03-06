import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

const AUTOMATIC_MIC_ID = 'automatic';

interface MicInfo {
    id: string;
    label: string;
}

export function useMicState() {
    const { t } = useTranslation();
    const automaticLabel = t('Automatic');

    const [micList, setMicList] = useState([
        { id: AUTOMATIC_MIC_ID, label: automaticLabel },
    ]);
    const [currentMic, setCurrentMic] = useState(AUTOMATIC_MIC_ID);
    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        async function loadCurrent() {
            try {
                const id = await invoke<string | null>('get_current_mic_id');
                const micId = id ?? AUTOMATIC_MIC_ID;
                setCurrentMic(micId);

                if (micId !== AUTOMATIC_MIC_ID) {
                    setMicList((prev) => {
                        for (const m of prev) {
                            if (m.id === micId) return prev;
                        }
                        return [...prev, { id: micId, label: micId }];
                    });
                }
            } catch (error) {
                console.error('Failed to load current mic', error);
            }
        }
        loadCurrent();
    }, []);

    useEffect(() => {
        setIsLoading(true);
        const timer = setTimeout(async () => {
            try {
                const devices = await invoke<MicInfo[]>('get_mic_list');
                const isCurrentMicFound = devices.some(
                    (d) => d.id === currentMic
                );

                setMicList((_) => {
                    const newList = [
                        { id: AUTOMATIC_MIC_ID, label: automaticLabel },
                        ...devices,
                    ];

                    if (currentMic !== AUTOMATIC_MIC_ID && !isCurrentMicFound) {
                        const disconnectedLabel = t('Disconnected');
                        newList.push({
                            id: currentMic,
                            label: `${currentMic} (${disconnectedLabel})`,
                        });
                    }

                    return newList;
                });
            } catch (error) {
                console.error('Failed to load mic list', error);
            } finally {
                setIsLoading(false);
            }
        }, 50);

        return () => clearTimeout(timer);
    }, [automaticLabel, currentMic]);

    useEffect(() => {
        const unlisten = listen<string>('recording-error', () => {
            toast.error(
                t('Microphone unavailable. Please check your device connection.')
            );
        });
        return () => {
            unlisten.then((fn) => fn());
        };
    }, [t]);

    async function setMic(id: string) {
        setCurrentMic(id);
        await invoke('set_current_mic_id', {
            micId: id === AUTOMATIC_MIC_ID ? null : id,
        });
    }

    return { micList, currentMic, setMic, isLoading };
}
