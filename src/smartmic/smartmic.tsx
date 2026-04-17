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
import type { TranslationSide, ViewMode } from './types';

export const SmartMic = () => {
    const [token] = useState<string | null>(() => getToken());
    const { connected, sendJson, sendBinary, lastMessage } = useSmartMicWebSocket(token);
    const [state, dispatch] = useReducer(smartMicReducer, initialState);
    const { isRecording, micLevel, transcriptions, modes, modeIndex, error, deviceConflict, viewMode, translationEntries, recordingSide } = state;

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

    // Handle server messages
    useEffect(() => {
        if (!lastMessage) return;
        dispatch({ type: 'server_message', message: lastMessage });
    }, [lastMessage]);

    const handleToggleRec = useCallback(async () => {
        if (isRecordingRef.current) {
            navigator.vibrate?.(50);
            dispatch({ type: 'rec_stopped' });
            sendJson({ type: 'rec_stop' });
            cleanupAudio();
        } else {
            if (!connected) return;
            try {
                await initAudio();
            } catch (err: unknown) {
                let message = "Impossible d'acceder au micro.";
                if (err instanceof Error && err.name === 'NotAllowedError') {
                    message =
                        "Acces au micro refuse. Veuillez autoriser l'acces dans les parametres de votre navigateur.";
                } else if (err instanceof Error) {
                    message = `Impossible d'acceder au micro: ${err.message}`;
                }
                dispatch({ type: 'set_error', error: { title: 'Erreur', message } });
                return;
            }
            navigator.vibrate?.(50);
            dispatch({ type: 'rec_started' });
            const shouldPaste = viewMode === 'remote';
            sendJson({ type: 'rec_start', mode: modes[modeIndex].id, paste: shouldPaste });
        }
    }, [connected, sendJson, initAudio, cleanupAudio, modes, modeIndex, viewMode]);

    const handleModeChange = useCallback((direction: 'prev' | 'next') => {
        if (isRecordingRef.current) return;
        dispatch({ type: 'change_mode', direction });
    }, []);

    const handleTranslationToggleRec = useCallback(async (side: TranslationSide, sourceLang: string, targetLang: string) => {
        if (isRecordingRef.current) {
            navigator.vibrate?.(50);
            dispatch({ type: 'rec_stopped' });
            sendJson({ type: 'rec_stop' });
            cleanupAudio();
        } else {
            if (!connected) return;
            try {
                await initAudio();
            } catch (err: unknown) {
                let message = "Impossible d'acceder au micro.";
                if (err instanceof Error && err.name === 'NotAllowedError') {
                    message = "Acces au micro refuse.";
                } else if (err instanceof Error) {
                    message = `Impossible d'acceder au micro: ${err.message}`;
                }
                dispatch({ type: 'set_error', error: { title: 'Erreur', message } });
                return;
            }
            navigator.vibrate?.(50);
            dispatch({ type: 'translation_rec_started', side });
            sendJson({ type: 'rec_start', mode: 'translation', paste: false, source_lang: sourceLang, target_lang: targetLang });
        }
    }, [connected, sendJson, initAudio, cleanupAudio]);

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
        if (saved !== null && (saved === 'remote' || saved === 'transcription' || saved === 'translation')) {
            dispatch({ type: 'set_view_mode', mode: saved });
        }
    }, []);

    const handleViewModeChange = useCallback((mode: ViewMode) => {
        dispatch({ type: 'set_view_mode', mode });
        localStorage.setItem('smartmic_view_mode', mode);
    }, []);

    const statusText = connected ? 'Connecte' : 'Connexion...';
    const pcName = connected ? location.hostname : '';

    // Register service worker
    useEffect(() => {
        if ('serviceWorker' in navigator) {
            navigator.serviceWorker.register('./sw.js').catch(() => {
                // SW registration failed, not critical
            });
        }
    }, []);

    return (
        <div className="w-full h-dvh flex flex-col bg-[#0a0a0a] text-[#e5e5e5] font-sans select-none pt-[env(safe-area-inset-top)] pb-[env(safe-area-inset-bottom)]">
            <ModeTabs activeMode={viewMode} onModeChange={handleViewModeChange} />
            <StatusBar connected={connected} statusText={statusText} pcName={pcName} />
            {viewMode === 'remote' && (
                <>
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
                    <Trackpad onMove={handleMove} onScroll={handleScroll} onTap={handleLeftClick} onLongPress={handleRightClick} />
                    <EnterButton onPress={() => handleKeyPress('Return')} onBackspace={() => handleKeyPress('BackSpace')} />
                </>
            )}
            {viewMode === 'transcription' && (
                <TranscriptionMode transcriptions={transcriptions} />
            )}
            {viewMode === 'translation' && (
                <TranslationMode
                    isRecording={isRecording}
                    recordingSide={recordingSide}
                    micLevel={micLevel}
                    translationEntries={translationEntries}
                    onToggleRec={handleTranslationToggleRec}
                />
            )}
            {viewMode !== 'translation' && (
                <RecArea
                    isRecording={isRecording}
                    currentMode={modes[modeIndex]}
                    micLevel={micLevel}
                    onToggleRec={handleToggleRec}
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
