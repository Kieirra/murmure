import { useCallback, useEffect, useReducer, useRef, useState } from 'react';
import { useSmartMicWebSocket, getToken } from './hooks/use-smartmic-websocket';
import { useAudioCapture } from './hooks/use-audio-capture';
import { StatusBar } from './components/status-bar';
import { TranscriptionZone } from './components/transcription-zone';
import { Trackpad } from './components/trackpad';
import { EnterButton } from './components/enter-button';
import { RecArea } from './components/rec-area';
import { ErrorOverlay } from './components/error-overlay';
import { DeviceConflictOverlay } from './components/device-conflict-overlay';
import { ModeTabs } from './components/mode-tabs';
import { TranscriptionMode } from './components/transcription-mode';
import { TranslationMode } from './components/translation-mode';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import { smartMicReducer, initialState } from './hooks/use-smartmic-reducer';
import type { ClientMessage, ViewMode } from './types';
import { useI18n } from './i18n/use-i18n';

export const SmartMic = () => {
    const { t } = useI18n();
    const [token] = useState<string | null>(() => getToken());
    const { connected, sendJson, sendBinary, lastMessage } = useSmartMicWebSocket(token);
    const [state, dispatch] = useReducer(smartMicReducer, initialState);
    const {
        isRecording,
        isTranslating,
        micLevel,
        transcriptions,
        modes,
        modeIndex,
        error,
        deviceConflict,
        viewMode,
        translationEntries,
        pendingTranslationPair,
    } = state;

    const isRecordingRef = useRef(false);
    isRecordingRef.current = isRecording;

    const onPcmChunk = useCallback(
        (buffer: ArrayBuffer) => {
            if (!isRecordingRef.current) return;
            const header = new Uint8Array([0x01]);
            const payload = new Uint8Array(buffer);
            const message = new Uint8Array(header.length + payload.length);
            message.set(header, 0);
            message.set(payload, header.length);
            sendBinary(message.buffer);
        },
        [sendBinary]
    );

    const { init: initAudio, cleanup: cleanupAudio } = useAudioCapture({ onPcmChunk });

    // Handle server messages: intercept error / force_disconnect to build a localized error.
    useEffect(() => {
        if (lastMessage === null) return;
        if (lastMessage.type === 'force_disconnect') {
            dispatch({
                type: 'set_error',
                error: { title: t('errors.disconnected'), message: t('errors.forceDisconnect') },
            });
            return;
        }
        if (lastMessage.type === 'error') {
            dispatch({
                type: 'set_error',
                error: { title: t('errors.title'), message: lastMessage.message || t('errors.micGeneric') },
            });
            return;
        }
        dispatch({ type: 'server_message', message: lastMessage });
    }, [lastMessage, t]);

    const stopMic = useCallback(() => {
        navigator.vibrate?.(50);
        dispatch({ type: 'rec_stopped' });
        sendJson({ type: 'rec_stop' });
        cleanupAudio();
    }, [sendJson, cleanupAudio]);

    const cancelMic = useCallback(() => {
        navigator.vibrate?.([30, 50, 30]);
        dispatch({ type: 'rec_cancelled' });
        sendJson({ type: 'rec_cancel' });
        cleanupAudio();
    }, [sendJson, cleanupAudio]);

    const startMic = useCallback(async (
        onStart: () => void,
        recStartPayload: ClientMessage,
    ): Promise<void> => {
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
    }, [initAudio, sendJson, t]);

    const handleToggleRec = useCallback(async () => {
        if (isRecordingRef.current) {
            stopMic();
            return;
        }
        if (!connected) return;
        const shouldPaste = viewMode === 'remote';
        await startMic(
            () => dispatch({ type: 'rec_started' }),
            { type: 'rec_start', mode: modes[modeIndex].id, paste: shouldPaste },
        );
    }, [connected, startMic, stopMic, modes, modeIndex, viewMode]);

    const handleCancelRec = useCallback(() => {
        if (!isRecordingRef.current) return;
        cancelMic();
    }, [cancelMic]);

    const handleModeChange = useCallback((direction: 'prev' | 'next') => {
        if (isRecordingRef.current) return;
        dispatch({ type: 'change_mode', direction });
    }, []);

    const handleTranslationToggleRec = useCallback(async (langA: string, langB: string) => {
        if (isRecordingRef.current) {
            stopMic();
            return;
        }
        if (!connected) return;
        await startMic(
            () => dispatch({ type: 'translation_rec_started', pair: { a: langA, b: langB } }),
            { type: 'rec_start', mode: 'translation', paste: false, lang_a: langA, lang_b: langB },
        );
    }, [connected, startMic, stopMic]);

    const handleMove = useCallback(
        (dx: number, dy: number) => {
            sendJson({ type: 'mouse_move', dx, dy });
        },
        [sendJson]
    );

    const handleScroll = useCallback(
        (dy: number) => {
            sendJson({ type: 'scroll', dy });
        },
        [sendJson]
    );

    const handleLeftClick = useCallback(() => {
        sendJson({ type: 'click', button: 'left' });
    }, [sendJson]);

    const handleRightClick = useCallback(() => {
        sendJson({ type: 'click', button: 'right' });
    }, [sendJson]);

    const handleKeyPress = useCallback(
        (key: string) => {
            sendJson({ type: 'key_press', key });
        },
        [sendJson]
    );

    const handleDismissError = useCallback(() => {
        dispatch({ type: 'dismiss_error' });
    }, []);

    const handleForceConnect = useCallback(() => {
        sendJson({ type: 'force_connect' });
        dispatch({ type: 'force_connect' });
    }, [sendJson]);

    const handleDismissConflict = useCallback(() => {
        dispatch({ type: 'dismiss_conflict' });
    }, []);

    const handleClearTranscriptions = useCallback(() => {
        dispatch({ type: 'clear_transcriptions' });
    }, []);

    // Cleanup audio on disconnect
    useEffect(() => {
        if (!connected && isRecordingRef.current) {
            dispatch({ type: 'disconnected' });
            cleanupAudio();
        }
    }, [connected, cleanupAudio]);

    // Restore view mode from localStorage
    useEffect(() => {
        const saved = localStorage.getItem('smartmic_view_mode') as ViewMode | null;
        if (saved === 'remote' || saved === 'transcription' || saved === 'translation') {
            dispatch({ type: 'set_view_mode', mode: saved });
        }
    }, []);

    const handleViewModeChange = useCallback((mode: ViewMode) => {
        dispatch({ type: 'set_view_mode', mode });
        localStorage.setItem('smartmic_view_mode', mode);
    }, []);

    const statusText = connected ? t('status.connected') : t('status.connecting');
    const pcName = connected ? location.hostname : '';

    // Register service worker
    useEffect(() => {
        if ('serviceWorker' in navigator) {
            navigator.serviceWorker.register('./sw.js').catch(() => {
                // SW registration failed, not critical
            });
        }
    }, []);

    // Prevent stale isTranslating if user hides the pending flag elsewhere.
    const translationRecordingActive = isRecording && pendingTranslationPair !== null;

    return (
        <div className="w-full h-dvh flex flex-col bg-[#0a0a0a] text-[#e5e5e5] font-sans select-none pt-[env(safe-area-inset-top)] pb-[env(safe-area-inset-bottom)]">
            <ModeTabs activeMode={viewMode} onModeChange={handleViewModeChange} />
            {viewMode === 'remote' && (
                <>
                    <StatusBar connected={connected} statusText={statusText} pcName={pcName} />
                    <TranscriptionZone transcriptions={transcriptions} />
                    <div className="h-24 px-3 flex items-center border-b border-[#222] shrink-0">
                        <AudioVisualizer
                            bars={28}
                            rows={16}
                            audioPixelWidth={6}
                            audioPixelHeight={3}
                            level={micLevel}
                            isProcessing={false}
                        />
                    </div>
                    <Trackpad
                        onMove={handleMove}
                        onScroll={handleScroll}
                        onTap={handleLeftClick}
                        onLongPress={handleRightClick}
                    />
                    <EnterButton
                        onPress={() => handleKeyPress('Return')}
                        onBackspace={() => handleKeyPress('BackSpace')}
                    />
                </>
            )}
            {viewMode === 'transcription' && (
                <TranscriptionMode
                    transcriptions={transcriptions}
                    onClearHistory={handleClearTranscriptions}
                />
            )}
            {viewMode === 'translation' && (
                <TranslationMode
                    isRecording={translationRecordingActive}
                    isTranslating={isTranslating}
                    micLevel={micLevel}
                    translationEntries={translationEntries}
                    onToggleRec={handleTranslationToggleRec}
                />
            )}
            {viewMode !== 'translation' && (
                <RecArea
                    isRecording={isRecording}
                    currentMode={modes[modeIndex]}
                    modeIndex={modeIndex}
                    totalModes={modes.length}
                    micLevel={micLevel}
                    onToggleRec={handleToggleRec}
                    onCancelRec={handleCancelRec}
                    onModeChange={handleModeChange}
                />
            )}
            <DeviceConflictOverlay
                deviceName={deviceConflict}
                onForceConnect={handleForceConnect}
                onDismiss={handleDismissConflict}
            />
            <ErrorOverlay
                visible={error !== null}
                title={error?.title ?? ''}
                message={error?.message ?? ''}
                onDismiss={handleDismissError}
            />
        </div>
    );
};
