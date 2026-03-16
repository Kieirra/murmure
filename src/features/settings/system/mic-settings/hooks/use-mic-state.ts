import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';

const AUTOMATIC_MIC_ID = 'automatic';

export interface MicInfo {
    id: string;
    label: string;
}

export const useMicState = () => {
    const { t } = useTranslation();
    const automaticLabel = t('Automatic');

    const [micList, setMicList] = useState([{ id: AUTOMATIC_MIC_ID, label: automaticLabel }]);
    const [currentMic, setCurrentMic] = useState(AUTOMATIC_MIC_ID);
    const [isLoading, setIsLoading] = useState(false);
    const lastKnownLabel = useRef<string | null>(null);

    const buildMicList = (devices: MicInfo[], selectedMicId: string) => {
        const currentDevice = devices.find((d) => d.id === selectedMicId);

        if (currentDevice) {
            lastKnownLabel.current = currentDevice.label;
        }

        const newList: MicInfo[] = [{ id: AUTOMATIC_MIC_ID, label: automaticLabel }, ...devices];

        if (selectedMicId !== AUTOMATIC_MIC_ID && !currentDevice) {
            const disconnectedSuffix = t('Disconnected');
            const friendlyName = lastKnownLabel.current ?? selectedMicId;
            newList.push({
                id: selectedMicId,
                label: `${friendlyName} (${disconnectedSuffix})`,
            });
        }

        return newList;
    };

    const addMicIfMissing = (micId: string, micLabel: string) => {
        setMicList((prev) => {
            if (prev.some((m) => m.id === micId)) return prev;
            return [...prev, { id: micId, label: micLabel }];
        });
    };

    useEffect(() => {
        const loadCurrent = async () => {
            try {
                const [id, label] = await Promise.all([
                    invoke<string | null>('get_current_mic_id'),
                    invoke<string | null>('get_current_mic_label'),
                ]);
                const micId = id || AUTOMATIC_MIC_ID;
                setCurrentMic(micId);

                if (label) {
                    lastKnownLabel.current = label;
                }

                if (micId !== AUTOMATIC_MIC_ID) {
                    addMicIfMissing(micId, label ?? micId);
                }
            } catch (error) {
                console.error('Failed to load current mic', error);
            }
        };
        loadCurrent();
    }, []);

    const refreshMicList = async () => {
        setIsLoading(true);
        try {
            const devices = await invoke<MicInfo[]>('get_mic_list');
            setMicList(buildMicList(devices, currentMic));
        } catch (error) {
            console.error('Failed to load mic list', error);
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        const timer = setTimeout(() => void refreshMicList(), 50);
        return () => clearTimeout(timer);
    }, [automaticLabel, currentMic]);

    const setMic = async (id: string) => {
        const mic = micList.find((m) => m.id === id);
        const label = mic && id !== AUTOMATIC_MIC_ID ? mic.label : null;
        if (label) {
            lastKnownLabel.current = label;
        }
        setCurrentMic(id);
        try {
            await invoke('set_current_mic_id', {
                micId: id === AUTOMATIC_MIC_ID ? null : id,
                micLabel: label,
            });
        } catch (error) {
            console.error('Failed to save microphone selection', error);
            toast.error(t('Failed to save microphone selection'));
        }
    };

    return { micList, currentMic, setMic, isLoading, refreshMicList };
};
