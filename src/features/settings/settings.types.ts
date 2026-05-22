export interface SystemSettings {
    record_mode: string;
    overlay_mode: string;
    overlay_position: string;
    api_enabled: boolean;
    api_port: number;
    copy_to_clipboard: boolean;
    paste_method: string;
    persist_history: boolean;
    transcription_finalization_strategy: string;
    language: string;
    sound_enabled: boolean;
    log_level: string;
    show_in_dock: boolean;
    streaming_preview: boolean;
    overlay_size: string;
    streaming_text_width: number;
    streaming_font_size: number;
    streaming_max_lines: number;
    use_wayland_portal: boolean;
    wayland_notice_dismissed: boolean;
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
    voice_mode_toggle_shortcut: string;
    cancel_shortcut: string;
}

export interface VoiceModeSettings {
    wake_word_enabled: boolean;
    wake_word_record: string;
    wake_word_command: string;
    wake_word_cancel: string;
    wake_word_validate: string;
    wake_word_submit: string;
    auto_enter_after_wake_word: boolean;
    silence_timeout_ms: number;
}

export interface SmartMicSettings {
    smartmic_enabled: boolean;
    smartmic_port: number;
    smartmic_relay_enabled: boolean;
    smartmic_relay_url: string | null;
    smartmic_machine_id_enabled: boolean;
    smartmic_machine_id: string | null;
    smartmic_token_ttl_hours: number | null;
    smartmic_bind_address: string | null;
}

export interface AppSettings extends SystemSettings, ShortcutSettings, VoiceModeSettings, SmartMicSettings {}
