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

/// Parse raw process args into a CLI command.
///
/// - `Ok(Some(cmd))`: a recognised, valid command.
/// - `Ok(None)`: no recognised command, the app should boot normally.
/// - `Err(msg)`: a recognised command with invalid arguments. Cold path callers
///   should surface `msg` and exit; hot path callers should log and stay alive.
pub fn parse_raw_args(args: &[String]) -> Result<Option<CliCommand>, String> {
    if let Some(import_index) = args.iter().position(|a| a == "import") {
        let file_path = match args.get(import_index + 1) {
            Some(p) => p.clone(),
            None => return Ok(None),
        };

        if file_path.starts_with('-') {
            return Ok(None);
        }

        let mut strategy = ImportStrategy::Replace;

        let mut i = import_index + 2;
        while i < args.len() {
            if args[i] == "--strategy" || args[i] == "-s" {
                match args.get(i + 1) {
                    Some(val) => {
                        strategy = parse_strategy(val)?;
                        i += 2;
                    }
                    None => return Ok(None),
                }
            } else {
                i += 1;
            }
        }

        return Ok(Some(CliCommand::Import {
            file_path,
            strategy,
        }));
    }

    // Top-level action flags (first match wins, single-action contract).
    for (flag, command) in ACTION_FLAGS {
        if args.iter().any(|a| a.strip_prefix("--") == Some(*flag)) {
            return Ok(Some(command.clone()));
        }
    }
    if let Some(idx) = args.iter().position(|a| a == "--llm-mode") {
        let value = match args.get(idx + 1) {
            Some(v) => v,
            None => return Ok(None),
        };
        let n = parse_llm_mode(value)?;
        return Ok(Some(CliCommand::LlmMode(n)));
    }

    Ok(None)
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
        let result = parse_raw_args(&args).unwrap();
        match result {
            Some(CliCommand::Import {
                file_path,
                strategy,
            }) => {
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
        let result = parse_raw_args(&args).unwrap();
        match result {
            Some(CliCommand::Import {
                file_path,
                strategy,
            }) => {
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
        let result = parse_raw_args(&args).unwrap();
        match result {
            Some(CliCommand::Import {
                file_path,
                strategy,
            }) => {
                assert_eq!(file_path, "/tmp/config.murmure");
                assert_eq!(strategy, ImportStrategy::Replace);
            }
            other => panic!("expected Import, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_raw_args_no_import() {
        let args = vec!["murmure".to_string(), "--autostart".to_string()];
        let result = parse_raw_args(&args).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_raw_args_import_without_file() {
        let args = vec!["murmure".to_string(), "import".to_string()];
        let result = parse_raw_args(&args).unwrap();
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
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_raw_args_invalid_strategy_message() {
        // Locks the contract: invalid strategy returns Err with the user-facing
        // message that cold-path callers print to stderr before exiting.
        let args = vec![
            "murmure".to_string(),
            "import".to_string(),
            "/tmp/config.murmure".to_string(),
            "--strategy".to_string(),
            "typo".to_string(),
        ];
        let err = parse_raw_args(&args).unwrap_err();
        assert!(
            err.contains("typo"),
            "message should mention bad value: {err}"
        );
        assert!(
            err.contains("replace") && err.contains("merge"),
            "message should list valid strategies: {err}"
        );
    }

    #[test]
    fn test_parse_raw_args_strategy_without_value() {
        let args = vec![
            "murmure".to_string(),
            "import".to_string(),
            "/tmp/config.murmure".to_string(),
            "--strategy".to_string(),
        ];
        let result = parse_raw_args(&args).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_raw_args_file_starts_with_dash() {
        let args = vec![
            "murmure".to_string(),
            "import".to_string(),
            "--something".to_string(),
        ];
        let result = parse_raw_args(&args).unwrap();
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
            assert_eq!(
                parse_raw_args(&args).unwrap(),
                Some(expected.clone()),
                "flag={flag}"
            );
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
        assert_eq!(parse_raw_args(&args).unwrap(), Some(CliCommand::LlmMode(2)));
    }

    #[test]
    fn test_parse_raw_args_llm_mode_out_of_range() {
        for value in ["0", "5", "100"] {
            let args = vec![
                "murmure".to_string(),
                "--llm-mode".to_string(),
                value.to_string(),
            ];
            assert!(parse_raw_args(&args).is_err(), "value={value}");
        }
    }

    #[test]
    fn test_parse_raw_args_llm_mode_out_of_range_message() {
        // Locks the Err message for --llm-mode hors plage: shell consumers
        // (cold path) rely on this to print before exit(1).
        let args = vec![
            "murmure".to_string(),
            "--llm-mode".to_string(),
            "99".to_string(),
        ];
        let err = parse_raw_args(&args).unwrap_err();
        assert!(
            err.contains("99"),
            "message should mention bad value: {err}"
        );
        assert!(
            err.contains("1") && err.contains("4"),
            "message should mention valid range: {err}"
        );
    }

    #[test]
    fn test_parse_raw_args_llm_mode_missing_value() {
        let args = vec!["murmure".to_string(), "--llm-mode".to_string()];
        assert_eq!(parse_raw_args(&args).unwrap(), None);
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
        assert_eq!(
            parse_raw_args(&args).unwrap(),
            Some(CliCommand::Transcription)
        );
    }
}
