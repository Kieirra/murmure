import { listen } from '@tauri-apps/api/event';
import { useState, useEffect, useRef } from 'react';

export const useLLMState = () => {
    const [isProcessing, setIsProcessing] = useState(false);
    const timeoutRef = useRef<number | null>(null);

    useEffect(() => {
        const unlistenStart = listen('llm-processing-start', () => {
            // Set a delay before showing the animation
            // This prevents flashing on fast operations (< 400ms)
            timeoutRef.current = window.setTimeout(() => {
                setIsProcessing(true);
                timeoutRef.current = null;
            }, 400);
        });

        const unlistenEnd = listen('llm-processing-end', () => {
            // Clear the timeout if it hasn't fired yet
            if (timeoutRef.current !== null) {
                clearTimeout(timeoutRef.current);
                timeoutRef.current = null;
            }
            // Set to false regardless (in case it was already showing)
            setIsProcessing(false);
        });

        return () => {
            // Clean up timeout on unmount
            if (timeoutRef.current !== null) {
                clearTimeout(timeoutRef.current);
            }
            unlistenStart.then((un) => un());
            unlistenEnd.then((un) => un());
        };
    }, []);

    return { isProcessing };
};
