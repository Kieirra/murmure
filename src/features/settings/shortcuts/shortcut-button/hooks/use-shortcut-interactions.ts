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

const NUMPAD_CODE_MAP: Record<string, string> = {
    Numpad0: 'kp0',
    Numpad1: 'kp1',
    Numpad2: 'kp2',
    Numpad3: 'kp3',
    Numpad4: 'kp4',
    Numpad5: 'kp5',
    Numpad6: 'kp6',
    Numpad7: 'kp7',
    Numpad8: 'kp8',
    Numpad9: 'kp9',
    NumpadAdd: 'kpplus',
    NumpadSubtract: 'kpminus',
    NumpadMultiply: 'kpmultiply',
    NumpadDivide: 'kpdivide',
};

const SPECIAL_KEY_CODE_MAP: Record<string, string> = {
    Backquote: 'backquote',
    IntlBackslash: 'intlbackslash',
};

const MOUSE_BUTTON_MAP: Record<number, string> = {
    // 0: 'mousebutton1', // Left click — disabled: blocks dialog interaction
    1: 'mousebutton3', // Middle click
    2: 'mousebutton2', // Right click
    3: 'mousebutton4', // Back (XButton1)
    4: 'mousebutton5', // Forward (XButton2)
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

    const normalizeKey = (key: string, code: string): string => {
        if (KEY_MAP[key]) return KEY_MAP[key];
        // Numpad: use e.code to distinguish from row digits
        const numpadKey = NUMPAD_CODE_MAP[code];
        if (numpadKey != null) return numpadKey;
        // Special keys: use e.code for physical position (layout-independent)
        const specialKey = SPECIAL_KEY_CODE_MAP[code];
        if (specialKey != null) return specialKey;
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

        const normalizedKey = normalizeKey(e.key, e.code);
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
        const buttonName = MOUSE_BUTTON_MAP[e.button];
        if (!buttonName) return;

        e.preventDefault();
        e.stopPropagation();

        if (!pressedKeysRef.current.has(buttonName)) {
            pressedKeysRef.current.add(buttonName);
            updateBinding();
        }
    };

    const onMouseUp = (e: MouseEvent) => {
        if (MOUSE_BUTTON_MAP[e.button]) {
            e.preventDefault();
            e.stopPropagation();
        }
    };

    const onContextMenu = (e: Event) => {
        e.preventDefault();
        e.stopPropagation();
    };

    useEffect(() => {
        if (!isRecording) return;

        invoke('suspend_transcription').catch(() => {});

        window.addEventListener('keydown', onKeyDown, { capture: true });
        window.addEventListener('keyup', onKeyUp, { capture: true });
        window.addEventListener('mousedown', onMouseDown, { capture: true });
        window.addEventListener('mouseup', onMouseUp, { capture: true });
        window.addEventListener('contextmenu', onContextMenu, {
            capture: true,
        });

        return () => {
            window.removeEventListener('keydown', onKeyDown, { capture: true });
            window.removeEventListener('keyup', onKeyUp, { capture: true });
            window.removeEventListener('mousedown', onMouseDown, {
                capture: true,
            });
            window.removeEventListener('mouseup', onMouseUp, { capture: true });
            window.removeEventListener('contextmenu', onContextMenu, {
                capture: true,
            });
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
