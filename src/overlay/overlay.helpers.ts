import { BufferedPhase } from './use-buffered-phase';

export const bufferedStatusLabel = (phase: BufferedPhase, isDone: boolean) => {
    if (isDone) return 'Done';
    return phase === BufferedPhase.Inserting ? 'Inserting…' : 'Transcribing…';
};
