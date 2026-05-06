use log::warn;
use tauri::AppHandle;
use tauri_plugin_cli::CliExt;

use super::types::{CliCommand, ImportStrategy};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns true if args were handled (caller should return from main without booting Tauri).
pub fn try_handle_early_args() -> bool {
    let args: Vec<String> = std::env::args().collect();
    let has_help = args.iter().any(|a| a == "--help" || a == "-h");
    let has_version = args.iter().any(|a| a == "--version" || a == "-V");
    let has_import = args.iter().any(|a| a == "import");

    if has_import && has_help {
        print_import_help();
        return true;
    }

    if has_help {
        print_help();
        return true;
    }

    if has_version {
        println!("murmure {}", VERSION);
        return true;
    }

    false
}

fn print_help() {
    println!(
        "\
murmure {}
Murmure - Privacy-first speech-to-text

USAGE:
    murmure [SUBCOMMAND] [OPTIONS]

SUBCOMMANDS:
    import    Import a .murmure configuration file

OPTIONS:
    --transcription              Toggle standard transcription on/off
    --transcription-llm          Toggle transcription in LLM mode
    --transcription-command      Toggle transcription in command mode
    --paste-last                 Paste the last transcription
    --cancel                     Cancel the current recording
    --voice-mode                 Toggle Voice Mode on/off
    --llm-mode <N>               Switch to LLM mode N (1-4)
    -h, --help                   Print help information
    -V, --version                Print version information

EXAMPLES:
    murmure --transcription
    murmure --paste-last
    murmure --llm-mode 2
    murmure import config.murmure --strategy merge",
        VERSION
    );
}

fn print_import_help() {
    println!(
        "\
murmure import
Import a .murmure configuration file

USAGE:
    murmure import <FILE> [OPTIONS]

ARGS:
    <FILE>    Path to the .murmure file to import

OPTIONS:
    -s, --strategy <STRATEGY>    Import strategy: replace (default) or merge
    -h, --help                   Print help information

EXAMPLES:
    murmure import config.murmure
    murmure import config.murmure --strategy merge
    murmure import config.murmure -s replace"
    );
}

fn parse_strategy(value: &str) -> Result<ImportStrategy, String> {
    match value.to_lowercase().as_str() {
        "replace" => Ok(ImportStrategy::Replace),
        "merge" => Ok(ImportStrategy::Merge),
        other => Err(format!(
            "Error: Invalid strategy '{}'. Use 'replace' or 'merge'.",
            other
        )),
    }
}

pub fn parse_cli_matches(app: &AppHandle) -> Option<CliCommand> {
    let matches = match app.cli().matches() {
        Ok(m) => m,
        Err(e) => {
            warn!("Failed to parse CLI matches: {}", e);
            return None;
        }
    };

    if let Some(subcommand) = matches.subcommand.as_ref() {
        if subcommand.name == "import" {
            let sub = &subcommand.matches;

            let file_path = sub
                .args
                .get("file")
                .and_then(|arg| arg.value.as_str())
                .map(|s| s.to_string())?;

            let strategy = match sub.args.get("strategy") {
                Some(arg) => match arg.value.as_str() {
                    Some(val) if !val.is_empty() => match parse_strategy(val) {
                        Ok(s) => s,
                        Err(msg) => {
                            eprintln!("{}", msg);
                            std::process::exit(1);
                        }
                    },
                    _ => ImportStrategy::Replace,
                },
                None => ImportStrategy::Replace,
            };

            return Some(CliCommand::Import {
                file_path,
                strategy,
            });
        }
    }

    // Treat presence as truthy: the cli plugin's bool encoding for
    // `takesValue=false` flags has shifted between versions.
    let is_present = |key: &str| -> bool {
        matches
            .args
            .get(key)
            .map(|arg| arg.occurrences > 0 || arg.value.as_bool().unwrap_or(false))
            .unwrap_or(false)
    };

    if is_present("transcription") {
        return Some(CliCommand::Transcription);
    }
    if is_present("transcription-llm") {
        return Some(CliCommand::TranscriptionLlm);
    }
    if is_present("transcription-command") {
        return Some(CliCommand::TranscriptionCommand);
    }
    if is_present("paste-last") {
        return Some(CliCommand::PasteLast);
    }
    if is_present("cancel") {
        return Some(CliCommand::Cancel);
    }
    if is_present("voice-mode") {
        return Some(CliCommand::VoiceMode);
    }

    if let Some(arg) = matches.args.get("llm-mode") {
        if let Some(val) = arg.value.as_str() {
            if !val.is_empty() {
                match parse_llm_mode(val) {
                    Ok(n) => return Some(CliCommand::LlmMode(n)),
                    Err(msg) => {
                        eprintln!("{}", msg);
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    None
}

fn parse_llm_mode(value: &str) -> Result<u8, String> {
    match value.parse::<u8>() {
        Ok(n) if (1..=4).contains(&n) => Ok(n),
        _ => Err(format!(
            "Error: Invalid LLM mode '{}'. Must be 1, 2, 3, or 4.",
            value
        )),
    }
}

pub fn parse_raw_args(args: &[String]) -> Option<CliCommand> {
    if let Some(import_index) = args.iter().position(|a| a == "import") {
        let file_path = args.get(import_index + 1)?.clone();

        if file_path.starts_with('-') {
            return None;
        }

        let mut strategy = ImportStrategy::Replace;

        let mut i = import_index + 2;
        while i < args.len() {
            if args[i] == "--strategy" || args[i] == "-s" {
                if let Some(val) = args.get(i + 1) {
                    match parse_strategy(val) {
                        Ok(s) => strategy = s,
                        Err(msg) => {
                            log::error!("{}", msg);
                            return None;
                        }
                    }
                    i += 2;
                } else {
                    return None;
                }
            } else {
                i += 1;
            }
        }

        return Some(CliCommand::Import {
            file_path,
            strategy,
        });
    }

    // Top-level action flags (first match wins, single-action contract).
    if args.iter().any(|a| a == "--transcription") {
        return Some(CliCommand::Transcription);
    }
    if args.iter().any(|a| a == "--transcription-llm") {
        return Some(CliCommand::TranscriptionLlm);
    }
    if args.iter().any(|a| a == "--transcription-command") {
        return Some(CliCommand::TranscriptionCommand);
    }
    if args.iter().any(|a| a == "--paste-last") {
        return Some(CliCommand::PasteLast);
    }
    if args.iter().any(|a| a == "--cancel") {
        return Some(CliCommand::Cancel);
    }
    if args.iter().any(|a| a == "--voice-mode") {
        return Some(CliCommand::VoiceMode);
    }
    if let Some(idx) = args.iter().position(|a| a == "--llm-mode") {
        let value = args.get(idx + 1)?;
        match parse_llm_mode(value) {
            Ok(n) => return Some(CliCommand::LlmMode(n)),
            Err(msg) => {
                log::error!("{}", msg);
                return None;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw_args_basic_import() {
        let args = vec![
            "murmure".to_string(),
            "import".to_string(),
            "/tmp/config.murmure".to_string(),
        ];
        let result = parse_raw_args(&args);
        assert!(result.is_some());
        match result.unwrap() {
            CliCommand::Import {
                file_path,
                strategy,
            } => {
                assert_eq!(file_path, "/tmp/config.murmure");
                assert_eq!(strategy, ImportStrategy::Replace);
            }
            other => panic!("expected Import, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_raw_args_with_strategy_merge() {
        let args = vec![
            "murmure".to_string(),
            "import".to_string(),
            "/tmp/config.murmure".to_string(),
            "--strategy".to_string(),
            "merge".to_string(),
        ];
        let result = parse_raw_args(&args);
        assert!(result.is_some());
        match result.unwrap() {
            CliCommand::Import {
                file_path,
                strategy,
            } => {
                assert_eq!(file_path, "/tmp/config.murmure");
                assert_eq!(strategy, ImportStrategy::Merge);
            }
            other => panic!("expected Import (merge), got {:?}", other),
        }
    }

    #[test]
    fn test_parse_raw_args_with_short_strategy() {
        let args = vec![
            "murmure".to_string(),
            "import".to_string(),
            "/tmp/config.murmure".to_string(),
            "-s".to_string(),
            "replace".to_string(),
        ];
        let result = parse_raw_args(&args);
        assert!(result.is_some());
        match result.unwrap() {
            CliCommand::Import {
                file_path,
                strategy,
            } => {
                assert_eq!(file_path, "/tmp/config.murmure");
                assert_eq!(strategy, ImportStrategy::Replace);
            }
            other => panic!("expected Import, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_raw_args_no_import() {
        let args = vec!["murmure".to_string(), "--autostart".to_string()];
        let result = parse_raw_args(&args);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_raw_args_import_without_file() {
        let args = vec!["murmure".to_string(), "import".to_string()];
        let result = parse_raw_args(&args);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_raw_args_invalid_strategy() {
        let args = vec![
            "murmure".to_string(),
            "import".to_string(),
            "/tmp/config.murmure".to_string(),
            "--strategy".to_string(),
            "foo".to_string(),
        ];
        let result = parse_raw_args(&args);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_raw_args_strategy_without_value() {
        let args = vec![
            "murmure".to_string(),
            "import".to_string(),
            "/tmp/config.murmure".to_string(),
            "--strategy".to_string(),
        ];
        let result = parse_raw_args(&args);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_raw_args_file_starts_with_dash() {
        let args = vec![
            "murmure".to_string(),
            "import".to_string(),
            "--something".to_string(),
        ];
        let result = parse_raw_args(&args);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_strategy_valid() {
        assert_eq!(parse_strategy("replace").unwrap(), ImportStrategy::Replace);
        assert_eq!(parse_strategy("merge").unwrap(), ImportStrategy::Merge);
        assert_eq!(parse_strategy("Replace").unwrap(), ImportStrategy::Replace);
        assert_eq!(parse_strategy("MERGE").unwrap(), ImportStrategy::Merge);
    }

    #[test]
    fn test_parse_strategy_invalid() {
        assert!(parse_strategy("foo").is_err());
        assert!(parse_strategy("").is_err());
    }

    #[test]
    fn test_parse_raw_args_transcription_flag() {
        let args = vec!["murmure".to_string(), "--transcription".to_string()];
        assert_eq!(parse_raw_args(&args), Some(CliCommand::Transcription));
    }

    #[test]
    fn test_parse_raw_args_transcription_llm_flag() {
        let args = vec!["murmure".to_string(), "--transcription-llm".to_string()];
        assert_eq!(parse_raw_args(&args), Some(CliCommand::TranscriptionLlm));
    }

    #[test]
    fn test_parse_raw_args_transcription_command_flag() {
        let args = vec![
            "murmure".to_string(),
            "--transcription-command".to_string(),
        ];
        assert_eq!(
            parse_raw_args(&args),
            Some(CliCommand::TranscriptionCommand)
        );
    }

    #[test]
    fn test_parse_raw_args_paste_last_flag() {
        let args = vec!["murmure".to_string(), "--paste-last".to_string()];
        assert_eq!(parse_raw_args(&args), Some(CliCommand::PasteLast));
    }

    #[test]
    fn test_parse_raw_args_cancel_flag() {
        let args = vec!["murmure".to_string(), "--cancel".to_string()];
        assert_eq!(parse_raw_args(&args), Some(CliCommand::Cancel));
    }

    #[test]
    fn test_parse_raw_args_voice_mode_flag() {
        let args = vec!["murmure".to_string(), "--voice-mode".to_string()];
        assert_eq!(parse_raw_args(&args), Some(CliCommand::VoiceMode));
    }

    #[test]
    fn test_parse_raw_args_llm_mode_valid() {
        for n in 1u8..=4 {
            let args = vec![
                "murmure".to_string(),
                "--llm-mode".to_string(),
                n.to_string(),
            ];
            assert_eq!(parse_raw_args(&args), Some(CliCommand::LlmMode(n)));
        }
    }

    #[test]
    fn test_parse_raw_args_llm_mode_out_of_range() {
        let args = vec![
            "murmure".to_string(),
            "--llm-mode".to_string(),
            "5".to_string(),
        ];
        assert_eq!(parse_raw_args(&args), None);
    }

    #[test]
    fn test_parse_raw_args_llm_mode_zero() {
        let args = vec![
            "murmure".to_string(),
            "--llm-mode".to_string(),
            "0".to_string(),
        ];
        assert_eq!(parse_raw_args(&args), None);
    }

    #[test]
    fn test_parse_raw_args_llm_mode_missing_value() {
        let args = vec!["murmure".to_string(), "--llm-mode".to_string()];
        assert_eq!(parse_raw_args(&args), None);
    }

    #[test]
    fn test_parse_raw_args_unknown_flag() {
        let args = vec!["murmure".to_string(), "--unknown-flag".to_string()];
        assert_eq!(parse_raw_args(&args), None);
    }

    #[test]
    fn test_parse_llm_mode_valid() {
        assert_eq!(parse_llm_mode("1"), Ok(1));
        assert_eq!(parse_llm_mode("4"), Ok(4));
    }

    #[test]
    fn test_parse_llm_mode_invalid() {
        assert!(parse_llm_mode("0").is_err());
        assert!(parse_llm_mode("5").is_err());
        assert!(parse_llm_mode("abc").is_err());
        assert!(parse_llm_mode("").is_err());
    }
}
