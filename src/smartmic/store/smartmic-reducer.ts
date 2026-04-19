import type { Mode, ServerMessage, TranscriptionEntry, TranslationEntry } from '../smartmic.types';

const DEFAULT_MODES: Mode[] = [{ id: 'stt', name: 'STT' }];

export interface TranslationPair {
    a: string;
    b: string;
}

export interface SmartMicState {
    isRecording: boolean;
    isTranslating: boolean;
    micLevel: number;
    transcriptions: TranscriptionEntry[];
    modes: Mode[];
    modeIndex: number;
    error: { title: string; message: string } | null;
    deviceConflict: string | null;
    translationEntries: TranslationEntry[];
    pendingTranslationPair: TranslationPair | null;
}

export type SmartMicAction =
    | { type: 'server_message'; message: ServerMessage }
    | { type: 'rec_started' }
    | { type: 'rec_stopped' }
    | { type: 'rec_cancelled' }
    | { type: 'set_error'; error: { title: string; message: string } }
    | { type: 'dismiss_error' }
    | { type: 'force_connect' }
    | { type: 'dismiss_conflict' }
    | { type: 'change_mode'; direction: 'prev' | 'next' }
    | { type: 'disconnected' }
    | { type: 'translation_rec_started'; pair: TranslationPair }
    | { type: 'clear_transcriptions' };

export const initialState: SmartMicState = {
    isRecording: false,
    isTranslating: false,
    micLevel: 0,
    transcriptions: [],
    modes: DEFAULT_MODES,
    modeIndex: 0,
    error: null,
    deviceConflict: null,
    translationEntries: [],
    pendingTranslationPair: null,
};

export const smartMicReducer = (state: SmartMicState, action: SmartMicAction): SmartMicState => {
    switch (action.type) {
        case 'server_message':
            return handleServerMessage(state, action.message);
        case 'rec_started':
            return { ...state, isRecording: true };
        case 'rec_stopped': {
            const wasTranslation = state.pendingTranslationPair !== null;
            return {
                ...state,
                isRecording: false,
                micLevel: 0,
                isTranslating: wasTranslation ? true : state.isTranslating,
            };
        }
        case 'rec_cancelled':
        case 'disconnected':
            return {
                ...state,
                isRecording: false,
                micLevel: 0,
                isTranslating: false,
                pendingTranslationPair: null,
            };
        case 'set_error':
            return { ...state, error: action.error };
        case 'dismiss_error':
            return { ...state, error: null };
        case 'force_connect':
            return { ...state, deviceConflict: null };
        case 'dismiss_conflict':
            return { ...state, deviceConflict: null };
        case 'change_mode': {
            const len = state.modes.length;
            const next = action.direction === 'prev' ? (state.modeIndex - 1 + len) % len : (state.modeIndex + 1) % len;
            return { ...state, modeIndex: next };
        }
        case 'translation_rec_started':
            return {
                ...state,
                isRecording: true,
                pendingTranslationPair: action.pair,
            };
        case 'clear_transcriptions':
            return { ...state, transcriptions: [] };
    }
};

const handleServerMessage = (state: SmartMicState, msg: ServerMessage): SmartMicState => {
    switch (msg.type) {
        case 'transcription': {
            const text = msg.text || '';
            const now = Date.now();

            if (state.pendingTranslationPair !== null) {
                const pair = state.pendingTranslationPair;
                const detected = msg.detected_lang ?? null;
                const inferredTarget = detected === pair.a ? pair.b : pair.a;
                const entry: TranslationEntry = {
                    text,
                    detectedLang: detected,
                    translatedText: msg.translated_text ?? '',
                    targetLang: msg.target_lang ?? inferredTarget,
                    timestamp: now,
                };
                return {
                    ...state,
                    translationEntries: [...state.translationEntries, entry].slice(-50),
                    pendingTranslationPair: null,
                    isTranslating: false,
                };
            }

            const entry: TranscriptionEntry = { text, timestamp: now };
            return {
                ...state,
                transcriptions: [entry, ...state.transcriptions].slice(0, 50),
            };
        }
        case 'mic_level':
            return typeof msg.level === 'number' ? { ...state, micLevel: msg.level } : state;
        case 'modes': {
            if (!Array.isArray(msg.modes)) return state;
            const newModes: Mode[] = [{ id: 'stt', name: 'STT' }];
            msg.modes.forEach((name, i) => {
                newModes.push({ id: `llm_${i}`, name });
            });
            return { ...state, modes: newModes, modeIndex: 0 };
        }
        case 'device_already_connected':
            return { ...state, deviceConflict: msg.device_name };
        // `force_disconnect` and `error` are intercepted in smartmic.tsx to build
        // a localized message via `useI18n`, then dispatched as `set_error`.
        case 'force_disconnect':
        case 'error':
            return state;
        case 'status':
            if (typeof msg.recording === 'boolean' && !msg.recording) {
                return { ...state, isRecording: false, micLevel: 0 };
            }
            return state;
    }
};
