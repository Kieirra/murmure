import { useEffect, useState } from 'react';

const STORAGE_KEY = 'murmure:dictionary-revamp-seen';
const UPDATED_EVENT = 'murmure:dictionary-revamp-seen-updated';

const readSeen = () => {
    if (typeof window === 'undefined') return false;
    return window.localStorage.getItem(STORAGE_KEY) === 'true';
};

export const useDictionaryRevampSeen = () => {
    const [seen, setSeen] = useState<boolean>(readSeen);

    useEffect(() => {
        const handleUpdated = () => setSeen(readSeen());
        window.addEventListener(UPDATED_EVENT, handleUpdated);
        return () => window.removeEventListener(UPDATED_EVENT, handleUpdated);
    }, []);

    const markSeen = () => {
        if (typeof window === 'undefined') return;
        window.localStorage.setItem(STORAGE_KEY, 'true');
        window.dispatchEvent(new CustomEvent(UPDATED_EVENT));
    };

    return { seen, markSeen };
};
