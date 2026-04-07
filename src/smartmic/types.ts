export interface Mode {
    id: string;
    name: string;
}

export type ClientMessage =
    | { type: 'pair'; token: string }
    | { type: 'rec_start'; mode: string }
    | { type: 'rec_stop' }
    | { type: 'rec_cancel' }
    | { type: 'mouse_move'; dx: number; dy: number }
    | { type: 'click'; button: 'left' | 'right' }
    | { type: 'scroll'; dy: number };

export type ServerMessage =
    | { type: 'transcription'; text: string }
    | { type: 'status'; recording: boolean }
    | { type: 'mic_level'; level: number }
    | { type: 'modes'; modes: string[] }
    | { type: 'error'; message: string };
