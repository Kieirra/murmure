use crate::settings;
use crate::shortcuts::helpers::parse_binding_keys;
use crate::shortcuts::types::{
    CommandShortcutKeys, LLMRecordShortcutKeys, LastTranscriptShortcutKeys, RecordShortcutKeys,
    LLMMode1ShortcutKeys, LLMMode2ShortcutKeys, LLMMode3ShortcutKeys, LLMMode4ShortcutKeys,
};
use tauri::{AppHandle, Manager};

pub fn initialize_shortcut_states(app_handle: &AppHandle) {
    let s = settings::load_settings(app_handle);
    let record_keys = parse_binding_keys(&s.record_shortcut);
    app_handle.manage(RecordShortcutKeys::new(record_keys));
    let last_transcript_keys = parse_binding_keys(&s.last_transcript_shortcut);
    app_handle.manage(LastTranscriptShortcutKeys::new(last_transcript_keys));
    let llm_record_keys = parse_binding_keys(&s.llm_record_shortcut);
    app_handle.manage(LLMRecordShortcutKeys::new(llm_record_keys));
    let command_keys = parse_binding_keys(&s.command_shortcut);
    app_handle.manage(CommandShortcutKeys::new(command_keys));
    
    let llm_mode_1_keys = parse_binding_keys(&s.llm_mode_1_shortcut);
    app_handle.manage(LLMMode1ShortcutKeys::new(llm_mode_1_keys));
    let llm_mode_2_keys = parse_binding_keys(&s.llm_mode_2_shortcut);
    app_handle.manage(LLMMode2ShortcutKeys::new(llm_mode_2_keys));
    let llm_mode_3_keys = parse_binding_keys(&s.llm_mode_3_shortcut);
    app_handle.manage(LLMMode3ShortcutKeys::new(llm_mode_3_keys));
    let llm_mode_4_keys = parse_binding_keys(&s.llm_mode_4_shortcut);
    app_handle.manage(LLMMode4ShortcutKeys::new(llm_mode_4_keys));
    
    app_handle.manage(crate::shortcuts::types::ShortcutState::new(
        false,
        s.record_mode == "toggle_to_talk",
        false,
    ));
}
