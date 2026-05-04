export interface ModeFlashPayload {
    text: string;
}

export interface ModeFlashState {
    text: string;
    fadingOut: boolean;
}

export const isModeFlashPayload = (value: unknown): value is ModeFlashPayload =>
    value != null &&
    typeof value === 'object' &&
    typeof (value as { text?: unknown }).text === 'string';
