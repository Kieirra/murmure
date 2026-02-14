import { useState, useRef, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

const KEY_MAP: Record<string, string> = {
    Meta: 'win',
    Control: 'ctrl',
    Alt: 'alt',
    Shift: 'shift',
    ' ': 'space',
    Enter: 'enter',
    Escape: 'escape',
    Tab: 'tab',
    Backspace: 'backspace',
    Delete: 'delete',
    Insert: 'insert',
    Home: 'home',
    End: 'end',
    PageUp: 'pageup',
    PageDown: 'pagedown',
    ArrowUp: 'arrowup',
    ArrowDown: 'arrowdown',
    ArrowLeft: 'arrowleft',
    ArrowRight: 'arrowright',
};

export const useShortcutInteractions = (
    shortcut: string,
    saveShortcut: (shortcut: string) => void,
    resetShortcut: () => void
) => {
    const [isRecording, setIsRecording] = useState(false);
    const [binding, setBinding] = useState(shortcut);
    const currentBindingRef = useRef('');
    const pressedKeysRef = useRef<Set<string>>(new Set());

    const normalizeKey = (key: string): string => {
        if (KEY_MAP[key]) return KEY_MAP[key];
        if (key.length === 1) return key.toLowerCase();
        if (key.startsWith('F') && key.length <= 3) return key.toLowerCase();
        if (key.startsWith('Digit')) return key.replace('Digit', '');
        if (key.startsWith('Key')) return key.replace('Key', '').toLowerCase();
        return key.toLowerCase();
    };

    const updateBinding = () => {
        const keys = Array.from(pressedKeysRef.current);
        const modifierOrder = ['win', 'ctrl', 'alt', 'shift'];
        const sorted = keys.sort((a, b) => {
            const aIdx = modifierOrder.indexOf(a);
            const bIdx = modifierOrder.indexOf(b);
            if (aIdx !== -1 && bIdx !== -1) return aIdx - bIdx;
            if (aIdx !== -1) return -1;
            if (bIdx !== -1) return 1;
            return a.localeCompare(b);
        });
        const newBinding = sorted.join('+');
        currentBindingRef.current = newBinding;
        setBinding(newBinding || '');
    };

    const onKeyDown = (e: KeyboardEvent) => {
        e.preventDefault();
        e.stopPropagation();

        if (e.key === 'Enter') {
            if (currentBindingRef.current) {
                saveShortcut(currentBindingRef.current);
            }
            pressedKeysRef.current.clear();
            setIsRecording(false);
            return;
        }

        if (e.key === 'Escape') {
            pressedKeysRef.current.clear();
            currentBindingRef.current = '';
            setBinding(shortcut);
            setIsRecording(false);
            return;
        }

        const normalizedKey = normalizeKey(e.key);
        if (
            normalizedKey &&
            normalizedKey !== 'enter' &&
            normalizedKey !== 'escape' &&
            !pressedKeysRef.current.has(normalizedKey)
        ) {
            pressedKeysRef.current.add(normalizedKey);
            updateBinding();
        }
    };

    const onKeyUp = (e: KeyboardEvent) => {
        e.preventDefault();
        e.stopPropagation();
    };

    const onMouseDown = (e: MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();

        const mouseButtonMap: Record<number, string> = {
            0: 'mouse1',
            1: 'mouse3',
            2: 'mouse2',
            3: 'mouse4',
            4: 'mouse5',
        };

        const mouseButton = mouseButtonMap[e.button];
        if (mouseButton && !pressedKeysRef.current.has(mouseButton)) {
            pressedKeysRef.current.add(mouseButton);
            updateBinding();
        }
    };

    const onContextMenu = (e: MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();
    };

    useEffect(() => {
        if (!isRecording) return;

        invoke('suspend_transcription').catch(() => {});

        globalThis.addEventListener('keydown', onKeyDown, { capture: true });
        globalThis.addEventListener('keyup', onKeyUp, { capture: true });
        globalThis.addEventListener('mousedown', onMouseDown, { capture: true });
        globalThis.addEventListener('contextmenu', onContextMenu, { capture: true });

        return () => {
            globalThis.removeEventListener('keydown', onKeyDown, { capture: true });
            globalThis.removeEventListener('keyup', onKeyUp, { capture: true });
            globalThis.removeEventListener('mousedown', onMouseDown, { capture: true });
            globalThis.removeEventListener('contextmenu', onContextMenu, { capture: true });
            invoke('resume_transcription').catch(() => {});
        };
    }, [isRecording]);

    return {
        binding,
        isRecording,
        resetRecording: () => {
            resetShortcut();
            setIsRecording(false);
        },
        startRecording: (open: boolean) => {
            setIsRecording(open);
            if (open) {
                setBinding('');
                currentBindingRef.current = '';
                pressedKeysRef.current.clear();
            }
        },
    };
};
