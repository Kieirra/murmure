export interface SystemSettings {
    record_mode: string;
    overlay_mode: string;
    overlay_position: string;
    api_enabled: boolean;
    api_port: number;
    copy_to_clipboard: boolean;
    paste_method: string;
    persist_history: boolean;
    language: string;
    sound_enabled: boolean;
    log_level: string;
    show_in_dock: boolean;
}

export interface ShortcutSettings {
    record_shortcut: string;
    last_transcript_shortcut: string;
    llm_record_shortcut: string;
    command_shortcut: string;
    llm_mode_1_shortcut: string;
    llm_mode_2_shortcut: string;
    llm_mode_3_shortcut: string;
    llm_mode_4_shortcut: string;
    cancel_shortcut: string;
}

export interface AppSettings extends SystemSettings, ShortcutSettings {}
