import { type Dispatch, useCallback, useEffect, useRef } from 'react';
import { useAudioCapture } from './use-audio-capture';
import type { ClientMessage, ViewMode } from '../smartmic.types';
import type { SmartMicAction, SmartMicState } from '../store/smartmic-reducer';
import { t } from '../i18n';

interface UseRecordingControlParams {
    connected: boolean;
    sendJson: (msg: ClientMessage) => void;
    sendBinary: (data: ArrayBuffer) => void;
    state: SmartMicState;
    dispatch: Dispatch<SmartMicAction>;
    viewMode: ViewMode;
}

export interface RecordingControl {
    toggle: () => Promise<void>;
    cancel: () => void;
    changeMode: (direction: 'prev' | 'next') => void;
    translationToggle: (langA: string, langB: string) => Promise<void>;
}

export const useRecordingControl = ({
    connected,
    sendJson,
    sendBinary,
    state,
    dispatch,
    viewMode,
}: UseRecordingControlParams): RecordingControl => {
    // Async audio callbacks must read the latest recording state without
    // closing over a stale reducer snapshot.
    const isRecordingRef = useRef(false);
    isRecordingRef.current = state.isRecording;

    const onPcmChunk = (buffer: ArrayBuffer) => {
        if (!isRecordingRef.current) return;
        const header = new Uint8Array([0x01]);
        const payload = new Uint8Array(buffer);
        const message = new Uint8Array(header.length + payload.length);
        message.set(header, 0);
        message.set(payload, header.length);
        sendBinary(message.buffer);
    };

    const { init: initAudio, cleanup: cleanupAudio } = useAudioCapture({ onPcmChunk });

    const stopMic = useCallback(() => {
        navigator.vibrate?.(50);
        dispatch({ type: 'rec_stopped' });
        sendJson({ type: 'rec_stop' });
        cleanupAudio();
    }, [sendJson, cleanupAudio, dispatch]);

    const cancelMic = useCallback(() => {
        navigator.vibrate?.([30, 50, 30]);
        dispatch({ type: 'rec_cancelled' });
        sendJson({ type: 'rec_cancel' });
        cleanupAudio();
    }, [sendJson, cleanupAudio, dispatch]);

    const startMic = useCallback(
        async (onStart: () => void, recStartPayload: ClientMessage): Promise<void> => {
            try {
                await initAudio();
            } catch (err: unknown) {
                let message = t('errors.micGeneric');
                if (err instanceof Error && err.name === 'NotAllowedError') {
                    message = t('errors.micDenied');
                } else if (err instanceof Error) {
                    message = t('errors.micError', { err: err.message });
                }
                dispatch({ type: 'set_error', error: { title: t('errors.title'), message } });
                return;
            }
            navigator.vibrate?.(50);
            onStart();
            sendJson(recStartPayload);
        },
        [initAudio, sendJson, dispatch]
    );

    const toggle = useCallback(async () => {
        if (isRecordingRef.current) {
            stopMic();
            return;
        }
        if (!connected) return;
        const shouldPaste = viewMode === 'remote';
        await startMic(
            () => dispatch({ type: 'rec_started' }),
            { type: 'rec_start', mode: state.modes[state.modeIndex].id, paste: shouldPaste }
        );
    }, [connected, startMic, stopMic, viewMode, state.modes, state.modeIndex, dispatch]);

    const cancel = useCallback(() => {
        if (!isRecordingRef.current) return;
        cancelMic();
    }, [cancelMic]);

    const changeMode = (direction: 'prev' | 'next') => {
        if (isRecordingRef.current) return;
        dispatch({ type: 'change_mode', direction });
    };

    const translationToggle = useCallback(
        async (langA: string, langB: string) => {
            if (isRecordingRef.current) {
                stopMic();
                return;
            }
            if (!connected) return;
            await startMic(
                () => dispatch({ type: 'translation_rec_started', pair: { a: langA, b: langB } }),
                { type: 'rec_start', mode: 'translation', paste: false, lang_a: langA, lang_b: langB }
            );
        },
        [connected, startMic, stopMic, dispatch]
    );

    useEffect(() => {
        if (!connected && isRecordingRef.current) {
            dispatch({ type: 'disconnected' });
            cleanupAudio();
        }
    }, [connected, cleanupAudio, dispatch]);

    return { toggle, cancel, changeMode, translationToggle };
};
