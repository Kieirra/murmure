import { useCallback, useEffect, useRef, useState } from 'react';
import { useSmartMicWebSocket, getToken } from './hooks/use-smartmic-websocket';
import { useAudioCapture } from './hooks/use-audio-capture';
import { StatusBar } from './components/status-bar';
import { TranscriptionZone } from './components/transcription-zone';
import { Trackpad } from './components/trackpad';
import { EnterButton } from './components/enter-button';
import { RecArea } from './components/rec-area';
import { ErrorOverlay } from './components/error-overlay';
import { AudioVisualizer } from '@/features/home/audio-visualizer/audio-visualizer';
import type { Mode, ServerMessage } from './types';

const DEFAULT_MODES: Mode[] = [{ id: 'stt', name: 'STT' }];

export const SmartMic = () => {
    const [token] = useState<string | null>(() => getToken());
    const { connected, sendJson, sendBinary, lastMessage } = useSmartMicWebSocket(token);
    const [isRecording, setIsRecording] = useState(false);
    const [micLevel, setMicLevel] = useState(0);
    const [transcriptions, setTranscriptions] = useState<string[]>([]);
    const [modes, setModes] = useState<Mode[]>(DEFAULT_MODES);
    const [modeIndex, setModeIndex] = useState(0);
    const [error, setError] = useState<{ title: string; message: string } | null>(null);

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
        const msg: ServerMessage = lastMessage;

        switch (msg.type) {
            case 'transcription': {
                const text = msg.text || '';
                setTranscriptions((prev) => [text, ...prev].slice(0, 3));
                break;
            }
            case 'mic_level': {
                if (typeof msg.level === 'number') {
                    setMicLevel(msg.level);
                }
                break;
            }
            case 'modes': {
                if (Array.isArray(msg.modes)) {
                    const newModes: Mode[] = [{ id: 'stt', name: 'STT' }];
                    msg.modes.forEach((name, i) => {
                        newModes.push({ id: `llm_${i}`, name });
                    });
                    setModes(newModes);
                    setModeIndex(0);
                }
                break;
            }
            case 'error': {
                setError({ title: 'Erreur', message: msg.message || 'Une erreur est survenue.' });
                break;
            }
            case 'status': {
                if (typeof msg.recording === 'boolean' && !msg.recording) {
                    setIsRecording(false);
                    setMicLevel(0);
                }
                break;
            }
        }
    }, [lastMessage]);

    const handleToggleRec = useCallback(async () => {
        if (isRecordingRef.current) {
            navigator.vibrate?.(50);
            setIsRecording(false);
            setMicLevel(0);
            sendJson({ type: 'rec_stop' });
        } else {
            if (!connected) return;
            try {
                await initAudio();
            } catch (err: unknown) {
                const message =
                    err instanceof Error && err.name === 'NotAllowedError'
                        ? "Acces au micro refuse. Veuillez autoriser l'acces dans les parametres de votre navigateur."
                        : err instanceof Error
                          ? `Impossible d'acceder au micro: ${err.message}`
                          : "Impossible d'acceder au micro.";
                setError({ title: 'Erreur', message });
                return;
            }
            navigator.vibrate?.(50);
            setIsRecording(true);
            sendJson({ type: 'rec_start', mode: modes[modeIndex].id });
        }
    }, [connected, sendJson, initAudio, modes, modeIndex]);

    const handleModeChange = useCallback(
        (direction: 'prev' | 'next') => {
            if (isRecordingRef.current) return;
            setModeIndex((prev) => {
                if (direction === 'prev') {
                    return (prev - 1 + modes.length) % modes.length;
                }
                return (prev + 1) % modes.length;
            });
        },
        [modes.length]
    );

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
        setError(null);
    }, []);

    // Cleanup audio on disconnect
    useEffect(() => {
        if (!connected && isRecordingRef.current) {
            setIsRecording(false);
            setMicLevel(0);
            cleanupAudio();
        }
    }, [connected, cleanupAudio]);

    const statusText = connected ? 'Connecte' : 'Connexion...';
    const pcName = connected ? location.hostname : '';

    // Register service worker
    useEffect(() => {
        if ('serviceWorker' in navigator) {
            navigator.serviceWorker.register('/sw.js').catch(() => {
                // SW registration failed, not critical
            });
        }
    }, []);

    return (
        <div className="w-full h-dvh flex flex-col bg-[#0a0a0a] text-[#e5e5e5] font-sans select-none pt-[env(safe-area-inset-top)] pb-[env(safe-area-inset-bottom)]">
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
            <Trackpad onMove={handleMove} onScroll={handleScroll} onTap={handleLeftClick} onLongPress={handleRightClick} />
            <EnterButton onPress={() => handleKeyPress('Return')} onBackspace={() => handleKeyPress('BackSpace')} />
            <RecArea
                isRecording={isRecording}
                currentMode={modes[modeIndex]}
                modes={modes}
                micLevel={micLevel}
                onToggleRec={handleToggleRec}
                onModeChange={handleModeChange}
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
