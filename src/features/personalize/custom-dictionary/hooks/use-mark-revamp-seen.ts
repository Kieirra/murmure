import { useEffect, useRef } from 'react';
import { useDictionaryRevampSeen } from './use-dictionary-revamp-seen';

export const useMarkRevampSeen = () => {
    const { seen, markSeen } = useDictionaryRevampSeen();
    const showRevampNotice = useRef(!seen).current;

    useEffect(() => {
        markSeen();
    }, []);

    return { showRevampNotice };
};
