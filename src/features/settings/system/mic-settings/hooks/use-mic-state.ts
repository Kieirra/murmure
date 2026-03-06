import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

const AUTOMATIC_MIC_ID = 'automatic';

interface MicInfo {
    id: string;
    label: string;
}

export const useMicState = () => {
    const { t } = useTranslation();
    const automaticLabel = t('Automatic');

    const [micList, setMicList] = useState([
        { id: AUTOMATIC_MIC_ID, label: automaticLabel },
    ]);
    const [currentMic, setCurrentMic] = useState(AUTOMATIC_MIC_ID);
    const [isLoading, setIsLoading] = useState(false);
    // Store the last known friendly label for the selected mic
    const lastKnownLabel = useRef<string | null>(null);

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
                const currentDevice = devices.find(
                    (d) => d.id === currentMic
                );

                // Update last known label if the device is currently connected
                if (currentDevice) {
                    lastKnownLabel.current = currentDevice.label;
                }

                setMicList((_) => {
                    const newList = [
                        { id: AUTOMATIC_MIC_ID, label: automaticLabel },
                        ...devices,
                    ];

                    if (currentMic !== AUTOMATIC_MIC_ID && !currentDevice) {
                        const disconnectedSuffix = t('Disconnected');
                        const friendlyName =
                            lastKnownLabel.current ?? currentMic;
                        newList.push({
                            id: currentMic,
                            label: `${friendlyName} (${disconnectedSuffix})`,
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
        const mic = micList.find((m) => m.id === id);
        if (mic && id !== AUTOMATIC_MIC_ID) {
            lastKnownLabel.current = mic.label;
        }
        setCurrentMic(id);
        try {
            await invoke('set_current_mic_id', {
                micId: id === AUTOMATIC_MIC_ID ? null : id,
            });
        } catch (error) {
            console.error('Failed to save microphone selection', error);
        }
    }

    async function refreshMicList() {
        setIsLoading(true);
        try {
            const devices = await invoke<MicInfo[]>('get_mic_list');
            const currentDevice = devices.find(
                (d) => d.id === currentMic
            );

            if (currentDevice) {
                lastKnownLabel.current = currentDevice.label;
            }

            setMicList((_) => {
                const newList = [
                    { id: AUTOMATIC_MIC_ID, label: automaticLabel },
                    ...devices,
                ];

                if (currentMic !== AUTOMATIC_MIC_ID && !currentDevice) {
                    const disconnectedSuffix = t('Disconnected');
                    const friendlyName =
                        lastKnownLabel.current ?? currentMic;
                    newList.push({
                        id: currentMic,
                        label: `${friendlyName} (${disconnectedSuffix})`,
                    });
                }

                return newList;
            });
        } catch (error) {
            console.error('Failed to refresh mic list', error);
        } finally {
            setIsLoading(false);
        }
    }

    return { micList, currentMic, setMic, isLoading, refreshMicList };
};
