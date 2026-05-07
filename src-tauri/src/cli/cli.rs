use log::warn;
use tauri::AppHandle;
use tauri_plugin_cli::CliExt;

use super::helpers::{parse_llm_mode, parse_strategy};
use super::types::{CliCommand, ImportStrategy};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Single-action flags: first match wins. Order is significant and must match
/// the original sequential checks. `LlmMode` is handled separately because it
/// carries a value.
const ACTION_FLAGS: &[(&str, CliCommand)] = &[
    ("transcription", CliCommand::Transcription),
    ("transcription-llm", CliCommand::TranscriptionLlm),
    ("transcription-command", CliCommand::TranscriptionCommand),
    ("paste-last", CliCommand::PasteLast),
    ("cancel", CliCommand::Cancel),
    ("voice-mode", CliCommand::VoiceMode),
];

/// Returns true if args were handled (caller should return from main without booting Tauri).
pub fn try_handle_early_args() -> bool {
    let args: Vec<String> = std::env::args().collect();
    let has_help = args.iter().any(|a| a == "--help" || a == "-h");
    let has_version = args.iter().any(|a| a == "--version" || a == "-V");

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
    murmure [OPTIONS]
    murmure import <FILE> [IMPORT_OPTIONS]

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

IMPORT:
    Import a .murmure configuration file.

    USAGE:
        murmure import <FILE> [OPTIONS]

    ARGS:
        <FILE>    Path to the .murmure file to import

    IMPORT_OPTIONS:
        -s, --strategy <STRATEGY>    Import strategy: replace (default) or merge

EXAMPLES:
    murmure --transcription
    murmure --paste-last
    murmure --llm-mode 2
    murmure import config.murmure
    murmure import config.murmure --strategy merge
    murmure import config.murmure -s replace",
        VERSION
    );
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

    for (flag, command) in ACTION_FLAGS {
        if is_present(flag) {
            return Some(command.clone());
        }
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
    for (flag, command) in ACTION_FLAGS {
        if args.iter().any(|a| a.strip_prefix("--") == Some(*flag)) {
            return Some(command.clone());
        }
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
    fn test_parse_raw_args_action_flags() {
        let cases: &[(&str, CliCommand)] = &[
            ("--transcription", CliCommand::Transcription),
            ("--transcription-llm", CliCommand::TranscriptionLlm),
            ("--transcription-command", CliCommand::TranscriptionCommand),
            ("--paste-last", CliCommand::PasteLast),
            ("--cancel", CliCommand::Cancel),
            ("--voice-mode", CliCommand::VoiceMode),
        ];

        for (flag, expected) in cases {
            let args = vec!["murmure".to_string(), flag.to_string()];
            assert_eq!(parse_raw_args(&args), Some(expected.clone()), "flag={flag}");
        }
    }

    #[test]
    fn test_parse_raw_args_llm_mode_valid() {
        // Smoke test: parse_raw_args wires --llm-mode <N> to CliCommand::LlmMode.
        // Full range coverage lives in cli::helpers::tests::test_parse_llm_mode_valid.
        let args = vec![
            "murmure".to_string(),
            "--llm-mode".to_string(),
            "2".to_string(),
        ];
        assert_eq!(parse_raw_args(&args), Some(CliCommand::LlmMode(2)));
    }

    #[test]
    fn test_parse_raw_args_llm_mode_out_of_range() {
        for value in ["0", "5", "100"] {
            let args = vec![
                "murmure".to_string(),
                "--llm-mode".to_string(),
                value.to_string(),
            ];
            assert_eq!(parse_raw_args(&args), None, "value={value}");
        }
    }

    #[test]
    fn test_parse_raw_args_llm_mode_missing_value() {
        let args = vec!["murmure".to_string(), "--llm-mode".to_string()];
        assert_eq!(parse_raw_args(&args), None);
    }

    #[test]
    fn test_parse_raw_args_single_action_first_match_wins() {
        // Contract: when several action flags are passed, the first match in
        // ACTION_FLAGS wins. Guards the single-action invariant.
        let args = vec![
            "murmure".to_string(),
            "--transcription".to_string(),
            "--paste-last".to_string(),
        ];
        assert_eq!(parse_raw_args(&args), Some(CliCommand::Transcription));
    }
}
