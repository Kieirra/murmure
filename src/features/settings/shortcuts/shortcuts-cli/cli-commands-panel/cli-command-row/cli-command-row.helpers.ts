// We use the standard browser clipboard API rather than the Tauri plugin
// to stay consistent with the rest of the app (history copy, smartmic copy).
// The Tauri webview enables clipboard write permissions by default.
export const copyCommandToClipboard = async (command: string): Promise<void> => {
    if (navigator.clipboard?.writeText === undefined) {
        throw new Error('Clipboard API unavailable');
    }
    await navigator.clipboard.writeText(command);
};

export const COPIED_FEEDBACK_DURATION_MS = 2000;
