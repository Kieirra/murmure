export interface Mode {
    id: string;
    name: string;
}

export type ViewMode = 'remote' | 'transcription' | 'translation';

export interface TranscriptionEntry {
    text: string;
    timestamp: number;
}

export interface TranslationEntry {
    text: string;
    detectedLang: string | null;
    translatedText: string;
    targetLang: string;
    timestamp: number;
}

export type ClientMessage =
    | { type: 'pair'; token: string }
    | { type: 'rec_start'; mode: string; paste?: boolean; lang_a?: string; lang_b?: string }
    | { type: 'rec_stop' }
    | { type: 'rec_cancel' }
    | { type: 'mouse_move'; dx: number; dy: number }
    | { type: 'click'; button: 'left' | 'right' }
    | { type: 'scroll'; dy: number }
    | { type: 'key_press'; key: string }
    | { type: 'force_connect' };

export type ServerMessage =
    | {
          type: 'transcription';
          text: string;
          detected_lang?: string | null;
          translated_text?: string;
          target_lang?: string;
      }
    | { type: 'status'; recording: boolean }
    | { type: 'mic_level'; level: number }
    | { type: 'modes'; modes: string[] }
    | { type: 'error'; message: string }
    | { type: 'device_already_connected'; device_name: string }
    | { type: 'force_disconnect' };
