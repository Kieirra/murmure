import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export enum BufferedPhase {
    Idle = 'idle',
    Transcribing = 'transcribing',
    Inserting = 'inserting',
}

export const useBufferedPhase = () => {
    const [phase, setPhase] = useState<BufferedPhase>(BufferedPhase.Idle);

    useEffect(() => {
        const unlistenTranscribing = listen('transcription-buffered', () => {
            setPhase(BufferedPhase.Transcribing);
        });
        const unlistenInserting = listen('transcription-inserting', () => {
            setPhase(BufferedPhase.Inserting);
        });
        const unlistenReset = listen('streaming-transcript', () => {
            setPhase(BufferedPhase.Idle);
        });
        return () => {
            unlistenTranscribing.then((u) => u());
            unlistenInserting.then((u) => u());
            unlistenReset.then((u) => u());
        };
    }, []);

    return phase;
};
