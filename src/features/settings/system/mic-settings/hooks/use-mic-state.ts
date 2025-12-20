import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from '@/i18n';

const AUTOMATIC_MIC_ID = 'automatic';

export function useMicState() {
    const { t } = useTranslation();
    const automaticLabel = t('Automatic');
    const systemDefaultLabel = t('System Default');

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
                        const label =
                            micId === 'default' ? systemDefaultLabel : micId;
                        return [...prev, { id: micId, label }];
                    });
                }
            } catch (error) {
                console.error('Failed to load current mic', error);
            }
        }
        loadCurrent();
    }, [systemDefaultLabel]);

    useEffect(() => {
        setIsLoading(true);
        const timer = setTimeout(async () => {
            try {
                const devices = await invoke<string[]>('get_mic_list');
                const isCurrentMicFound = devices.includes(currentMic);
                const mapped = devices.map((label) => ({
                    id: label,
                    label: label === 'default' ? systemDefaultLabel : label,
                }));

                setMicList((_) => {
                    const newList = [
                        { id: AUTOMATIC_MIC_ID, label: automaticLabel },
                        ...mapped,
                    ];

                    // Ensure currently selected mic is kept in the list if not found
                    if (currentMic !== AUTOMATIC_MIC_ID && !isCurrentMicFound) {
                        const missingLabel =
                            currentMic === 'default'
                                ? systemDefaultLabel
                                : currentMic;
                        newList.push({ id: currentMic, label: missingLabel });
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
    }, [automaticLabel, currentMic, systemDefaultLabel]);

    async function setMic(id: string) {
        setCurrentMic(id);
        await invoke('set_current_mic_id', {
            micId: id === AUTOMATIC_MIC_ID ? null : id,
        });
    }

    return { micList, currentMic, setMic, isLoading };
}
