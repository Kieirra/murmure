import { useEffect, useRef, useState } from 'react';

const COPIED_FEEDBACK_DURATION_MS = 2000;

export const useCopyFeedback = (text: string) => {
    const [copied, setCopied] = useState(false);
    const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    useEffect(() => {
        return () => {
            if (timeoutRef.current !== null) {
                clearTimeout(timeoutRef.current);
            }
        };
    }, []);

    const handleCopy = async () => {
        try {
            await copyCommandToClipboard(text);
            setCopied(true);
            if (timeoutRef.current !== null) {
                clearTimeout(timeoutRef.current);
            }
            timeoutRef.current = setTimeout(() => {
                setCopied(false);
                timeoutRef.current = null;
            }, COPIED_FEEDBACK_DURATION_MS);
        } catch (err) {
            console.error('Failed to copy CLI command:', err);
        }
    };

    return { copied, handleCopy };
};

// We use the standard browser clipboard API rather than the Tauri plugin
// to stay consistent with the rest of the app (history copy, smartmic copy).
// The Tauri webview enables clipboard write permissions by default.
const copyCommandToClipboard = async (command: string): Promise<void> => {
    if (navigator.clipboard?.writeText === undefined) {
        throw new Error('Clipboard API unavailable');
    }
    await navigator.clipboard.writeText(command);
};
