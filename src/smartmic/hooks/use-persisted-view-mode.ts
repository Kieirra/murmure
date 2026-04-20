import { type Dispatch, type SetStateAction, useState } from 'react';
import type { ViewMode } from '../smartmic.types';

const STORAGE_KEY = 'smartmic_view_mode';

const isViewMode = (value: string | null): value is ViewMode =>
    value === 'remote' || value === 'transcription' || value === 'translation';

const readInitial = (): ViewMode => {
    const saved = localStorage.getItem(STORAGE_KEY);
    return isViewMode(saved) ? saved : 'remote';
};

export const usePersistedViewMode = (): [ViewMode, Dispatch<SetStateAction<ViewMode>>] => {
    const [viewMode, setViewMode] = useState<ViewMode>(readInitial);

    const persistingSetter: Dispatch<SetStateAction<ViewMode>> = (next) => {
        setViewMode((prev) => {
            const resolved = typeof next === 'function' ? next(prev) : next;
            localStorage.setItem(STORAGE_KEY, resolved);
            return resolved;
        });
    };

    return [viewMode, persistingSetter];
};
