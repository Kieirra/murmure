use log::warn;
use tauri::AppHandle;
use tauri_plugin_cli::CliExt;

use super::types::{CliCommand, ImportStrategy};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handles --help, --version and import --help before Tauri starts.
/// Returns true if args were handled (caller should return from main).
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
    murmure [SUBCOMMAND]

SUBCOMMANDS:
    import    Import a .murmure configuration file

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

EXAMPLES:
    murmure import config.murmure
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

    let subcommand = matches.subcommand.as_ref()?;
    if subcommand.name != "import" {
        return None;
    }

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

    Some(CliCommand::Import {
        file_path,
        strategy,
    })
}

pub fn parse_raw_args(args: &[String]) -> Option<CliCommand> {
    let import_index = args.iter().position(|a| a == "import")?;
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

    Some(CliCommand::Import {
        file_path,
        strategy,
    })
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
}
