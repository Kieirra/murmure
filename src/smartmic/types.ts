export interface Mode {
    id: string;
    name: string;
}

export type ViewMode = 'remote' | 'transcription' | 'translation';

export type TranslationSide = 'top' | 'bottom';

export interface TranslationEntry {
    text: string;
    fromSide: TranslationSide;
}

export type ClientMessage =
    | { type: 'pair'; token: string }
    | { type: 'rec_start'; mode: string; paste?: boolean; source_lang?: string; target_lang?: string }
    | { type: 'rec_stop' }
    | { type: 'rec_cancel' }
    | { type: 'mouse_move'; dx: number; dy: number }
    | { type: 'click'; button: 'left' | 'right' }
    | { type: 'scroll'; dy: number }
    | { type: 'key_press'; key: string }
    | { type: 'force_connect' };

export type ServerMessage =
    | { type: 'transcription'; text: string }
    | { type: 'status'; recording: boolean }
    | { type: 'mic_level'; level: number }
    | { type: 'modes'; modes: string[] }
    | { type: 'error'; message: string }
    | { type: 'device_already_connected'; device_name: string }
    | { type: 'force_disconnect' };
