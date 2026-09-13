import { useCallback, useEffect, useRef, useState } from 'react';
import type { ClientMessage, ServerMessage } from '../smartmic.types';

const MAX_RECONNECT_ATTEMPTS = 10;
const RECONNECT_INTERVAL_MS = 3000;
const UUID_V4_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
const HEX_DIGITS = '0123456789abcdef';

const canonicalizePairingToken = (value: string): string | null => {
    if (!UUID_V4_PATTERN.test(value)) {
        return null;
    }
    const canonical: string[] = [];
    for (const char of value.toLowerCase()) {
        if (char === '-') {
            canonical.push('-');
            continue;
        }
        const hexIndex = HEX_DIGITS.indexOf(char);
        if (hexIndex < 0) {
            return null;
        }
        canonical.push(HEX_DIGITS.charAt(hexIndex));
    }
    return canonical.join('');
};

const isValidToken = (token: string): boolean => canonicalizePairingToken(token) != null;

const stripTokenFromUrl = () => {
    const url = new URL(globalThis.location.href);
    url.searchParams.delete('token');
    const hashParams = new URLSearchParams(url.hash.replace(/^#/, ''));
    hashParams.delete('token');
    const remainingHash = hashParams.toString();
    url.hash = remainingHash.length > 0 ? remainingHash : '';
    globalThis.history.replaceState(null, '', `${url.pathname}${url.search}${url.hash}`);
};

export const getToken = (): string | null => {
    const hashParams = new URLSearchParams(globalThis.location.hash.replace(/^#/, ''));
    const queryParams = new URLSearchParams(globalThis.location.search);
    const rawToken = hashParams.get('token') ?? queryParams.get('token');
    const urlToken = rawToken != null ? canonicalizePairingToken(rawToken) : null;
    if (urlToken != null) {
        localStorage.setItem('smartmic_token', urlToken);
        stripTokenFromUrl();
        return urlToken;
    }
    const storedToken = localStorage.getItem('smartmic_token');
    if (storedToken != null) {
        return canonicalizePairingToken(storedToken);
    }
    return null;
};

export const useSmartMicWebSocket = (token: string | null) => {
    const wsRef = useRef<WebSocket | null>(null);
    const [connected, setConnected] = useState(false);
    const [lastMessage, setLastMessage] = useState<ServerMessage | null>(null);
    const reconnectAttemptsRef = useRef(0);
    const reconnectTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const tokenRef = useRef(token);
    tokenRef.current = token;

    const connect = useCallback(() => {
        const currentToken = tokenRef.current;
        if (!currentToken || !isValidToken(currentToken)) return;

        const attemptReconnect = () => {
            if (reconnectTimerRef.current !== null) return;
            if (reconnectAttemptsRef.current >= MAX_RECONNECT_ATTEMPTS) return;

            reconnectAttemptsRef.current++;
            reconnectTimerRef.current = setTimeout(() => {
                reconnectTimerRef.current = null;
                connect();
            }, RECONNECT_INTERVAL_MS);
        };

        // Close any previous socket cleanly before creating a new one.
        // Null handlers first so the orphan's onclose does not trigger a reconnect cascade.
        if (wsRef.current !== null) {
            const previous = wsRef.current;
            previous.onopen = null;
            previous.onmessage = null;
            previous.onclose = null;
            previous.onerror = null;
            previous.close();
            wsRef.current = null;
        }

        const basePath = location.pathname.replace(/\/$/, '');
        // Token is passed as WebSocket subprotocol to avoid leaking it in proxy
        // access logs. The server echoes it back via Sec-WebSocket-Protocol.
        const wsUrl = `wss://${location.host}${basePath}/ws`;

        try {
            const ws = new WebSocket(wsUrl, currentToken);
            ws.binaryType = 'arraybuffer';

            ws.onopen = () => {
                setConnected(true);
                reconnectAttemptsRef.current = 0;
                ws.send(JSON.stringify({ type: 'pair', token: currentToken }));
            };

            ws.onmessage = (event: MessageEvent) => {
                if (typeof event.data === 'string') {
                    try {
                        const msg = JSON.parse(event.data) as ServerMessage;
                        setLastMessage(msg);
                    } catch {
                        // Ignore parse errors
                    }
                }
            };

            ws.onclose = () => {
                setConnected(false);
                wsRef.current = null;
                attemptReconnect();
            };

            ws.onerror = () => {
                // onclose will fire after this
            };

            wsRef.current = ws;
        } catch {
            // Connection failed
        }
    }, []);

    const sendJson = useCallback((msg: ClientMessage) => {
        if (wsRef.current?.readyState === WebSocket.OPEN) {
            wsRef.current.send(JSON.stringify(msg));
        }
    }, []);

    const sendBinary = useCallback((data: ArrayBuffer) => {
        if (wsRef.current?.readyState === WebSocket.OPEN) {
            wsRef.current.send(data);
        }
    }, []);

    useEffect(() => {
        if (token) {
            connect();
        }

        return () => {
            if (reconnectTimerRef.current !== null) {
                clearTimeout(reconnectTimerRef.current);
                reconnectTimerRef.current = null;
            }
            if (wsRef.current) {
                wsRef.current.onclose = null;
                wsRef.current.close();
                wsRef.current = null;
            }
        };
    }, [token, connect]);

    useEffect(() => {
        const handleVisibilityChange = () => {
            if (document.visibilityState === 'visible') {
                // Page is back in foreground - check connection and reconnect if needed
                if (wsRef.current?.readyState !== WebSocket.OPEN) {
                    reconnectAttemptsRef.current = 0; // Reset attempts for fresh reconnect
                    connect();
                }
            } else {
                // Page going to background - close WebSocket cleanly
                // This prevents zombie connections
                if (wsRef.current) {
                    wsRef.current.onclose = null; // Prevent attemptReconnect in background
                    wsRef.current.close();
                    wsRef.current = null;
                    setConnected(false);
                }
                // Clear any pending reconnect timer
                if (reconnectTimerRef.current !== null) {
                    clearTimeout(reconnectTimerRef.current);
                    reconnectTimerRef.current = null;
                }
            }
        };

        document.addEventListener('visibilitychange', handleVisibilityChange);
        return () => document.removeEventListener('visibilitychange', handleVisibilityChange);
    }, [connect]);

    return { ws: wsRef, connected, sendJson, sendBinary, lastMessage };
};

