import type { Mode, ServerMessage, TranslationEntry, TranslationSide, ViewMode } from '../types';

const DEFAULT_MODES: Mode[] = [{ id: 'stt', name: 'STT' }];

export interface SmartMicState {
    isRecording: boolean;
    micLevel: number;
    transcriptions: string[];
    modes: Mode[];
    modeIndex: number;
    error: { title: string; message: string } | null;
    deviceConflict: string | null;
    viewMode: ViewMode;
    translationEntries: TranslationEntry[];
    recordingSide: TranslationSide | null;
}

export type SmartMicAction =
    | { type: 'server_message'; message: ServerMessage }
    | { type: 'rec_started' }
    | { type: 'rec_stopped' }
    | { type: 'set_error'; error: { title: string; message: string } }
    | { type: 'dismiss_error' }
    | { type: 'force_connect' }
    | { type: 'dismiss_conflict' }
    | { type: 'change_mode'; direction: 'prev' | 'next' }
    | { type: 'disconnected' }
    | { type: 'set_view_mode'; mode: ViewMode }
    | { type: 'translation_rec_started'; side: TranslationSide }
    | { type: 'clear_translation' };

export const initialState: SmartMicState = {
    isRecording: false,
    micLevel: 0,
    transcriptions: [],
    modes: DEFAULT_MODES,
    modeIndex: 0,
    error: null,
    deviceConflict: null,
    viewMode: 'remote',
    translationEntries: [],
    recordingSide: null,
};

export function smartMicReducer(state: SmartMicState, action: SmartMicAction): SmartMicState {
    switch (action.type) {
        case 'server_message':
            return handleServerMessage(state, action.message);
        case 'rec_started':
            return { ...state, isRecording: true };
        case 'rec_stopped':
            return { ...state, isRecording: false, micLevel: 0 };
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
        case 'disconnected':
            return { ...state, isRecording: false, micLevel: 0 };
        case 'set_view_mode':
            return { ...state, viewMode: action.mode };
        case 'translation_rec_started':
            return { ...state, isRecording: true, recordingSide: action.side };
        case 'clear_translation':
            return { ...state, translationEntries: [] };
    }
}

function handleServerMessage(state: SmartMicState, msg: ServerMessage): SmartMicState {
    switch (msg.type) {
        case 'transcription': {
            const text = msg.text || '';
            const limit = state.viewMode === 'remote' ? 3 : 50;
            const newTranscriptions = [text, ...state.transcriptions].slice(0, limit);

            if (state.recordingSide !== null) {
                const entry: TranslationEntry = { text, fromSide: state.recordingSide };
                return {
                    ...state,
                    transcriptions: newTranscriptions,
                    translationEntries: [...state.translationEntries, entry].slice(-50),
                    recordingSide: null,
                };
            }
            return { ...state, transcriptions: newTranscriptions };
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
        case 'force_disconnect':
            return { ...state, error: { title: 'Deconnecte', message: 'Un autre appareil a pris le controle.' } };
        case 'error':
            return { ...state, error: { title: 'Erreur', message: msg.message || 'Une erreur est survenue.' } };
        case 'status':
            if (typeof msg.recording === 'boolean' && !msg.recording) {
                return { ...state, isRecording: false, micLevel: 0 };
            }
            return state;
    }
}
