pub fn normalize_result_panel_mode(mode: &str) -> &'static str {
    match mode {
        "off" => "off",
        "command_llm" => "command_llm",
        "all" => "all",
        _ => "command",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_result_panel_modes_are_kept() {
        for mode in ["off", "command", "command_llm", "all"] {
            assert_eq!(normalize_result_panel_mode(mode), mode);
        }
    }

    #[test]
    fn unknown_result_panel_modes_fall_back_to_command() {
        for mode in ["commands", "", "Commands"] {
            assert_eq!(normalize_result_panel_mode(mode), "command");
        }
    }
}
