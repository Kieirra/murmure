const LEGACY_CSV_HEADERS: [&str; 7] = ["word", "words", "term", "terms", "mot", "mots", "termes"];

fn extract_quoted_field(line: &str) -> Option<String> {
    let mut characters = line.strip_prefix('"')?.chars().peekable();
    let mut field = String::new();

    while let Some(character) = characters.next() {
        match (character, characters.peek()) {
            ('"', Some('"')) => {
                field.push('"');
                characters.next();
            }
            ('"', _) => return Some(field),
            _ => field.push(character),
        }
    }

    None
}

fn extract_legacy_csv_field(line: &str) -> String {
    let field = match extract_quoted_field(line) {
        Some(quoted) => quoted,
        None => match line.find([',', ';']) {
            Some(separator) => &line[..separator],
            None => line,
        }
        .to_string(),
    };

    field.trim().to_string()
}

pub fn normalize_import_content(raw: &str, is_legacy_csv: bool) -> Vec<String> {
    let content = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    let mut entries: Vec<String> = content
        .lines()
        .map(|line| match is_legacy_csv {
            true => extract_legacy_csv_field(line.trim()),
            false => line.trim().to_string(),
        })
        .filter(|entry| !entry.is_empty())
        .collect();

    let starts_with_header = is_legacy_csv
        && entries.len() >= 2
        && LEGACY_CSV_HEADERS
            .iter()
            .any(|header| header.eq_ignore_ascii_case(&entries[0]));
    if starts_with_header {
        entries.remove(0);
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_strips_utf8_bom() {
        assert_eq!(
            normalize_import_content("\u{feff}Kubernetes\nParakeet", false),
            vec!["Kubernetes", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_handles_crlf() {
        assert_eq!(
            normalize_import_content("Kubernetes\r\nParakeet\r\n", false),
            vec!["Kubernetes", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_trims_and_skips_empty_lines() {
        assert_eq!(
            normalize_import_content("  Kubernetes  \n\n  Parakeet\n", false),
            vec!["Kubernetes", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_txt_keeps_commas() {
        assert_eq!(
            normalize_import_content("New York,en\nParakeet", false),
            vec!["New York,en", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_legacy_splits_on_comma() {
        assert_eq!(
            normalize_import_content("New York,en\nParakeet,fr", true),
            vec!["New York", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_legacy_splits_on_semicolon() {
        assert_eq!(
            normalize_import_content("Kubernetes;fr\nParakeet;en", true),
            vec!["Kubernetes", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_legacy_unquotes_field() {
        assert_eq!(
            normalize_import_content("\"New York\",en\nParakeet", true),
            vec!["New York", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_legacy_unescapes_doubled_quotes() {
        assert_eq!(
            normalize_import_content("\"L\"\"Oreal\",fr\nParakeet", true),
            vec!["L\"Oreal", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_legacy_keeps_separator_inside_quoted_field() {
        assert_eq!(
            normalize_import_content("\"New York, NY\",en\nParakeet", true),
            vec!["New York, NY", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_legacy_unescapes_quotes_inside_quoted_field() {
        assert_eq!(
            normalize_import_content("\"L\"\"Oreal, SA\",fr", true),
            vec!["L\"Oreal, SA"]
        );
    }

    #[test]
    fn test_normalize_legacy_falls_back_on_unbalanced_quote() {
        assert_eq!(
            normalize_import_content("\"Kubernetes,fr", true),
            vec!["\"Kubernetes"]
        );
    }

    #[test]
    fn test_normalize_legacy_trims_dequoted_field() {
        assert_eq!(
            normalize_import_content("\" Kubernetes \",fr", true),
            vec!["Kubernetes"]
        );
    }

    #[test]
    fn test_normalize_legacy_trims_field_around_separator() {
        assert_eq!(
            normalize_import_content("Kubernetes , en\nParakeet ; fr", true),
            vec!["Kubernetes", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_legacy_unquotes_field_with_spaces() {
        assert_eq!(
            normalize_import_content("\"New York\" ,en\nParakeet", true),
            vec!["New York", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_legacy_drops_known_header() {
        assert_eq!(
            normalize_import_content("word\nKubernetes\nParakeet", true),
            vec!["Kubernetes", "Parakeet"]
        );
    }

    #[test]
    fn test_normalize_legacy_drops_header_case_insensitive() {
        assert_eq!(
            normalize_import_content("Mots\nKubernetes", true),
            vec!["Kubernetes"]
        );
    }

    #[test]
    fn test_normalize_legacy_keeps_single_header_like_entry() {
        assert_eq!(normalize_import_content("word", true), vec!["word"]);
    }

    #[test]
    fn test_normalize_legacy_ignores_header_in_txt_mode() {
        assert_eq!(
            normalize_import_content("word\nKubernetes", false),
            vec!["word", "Kubernetes"]
        );
    }

    #[test]
    fn test_normalize_empty_content() {
        assert!(normalize_import_content("", false).is_empty());
    }

    #[test]
    fn test_normalize_only_empty_lines() {
        assert!(normalize_import_content("\n  \n\n", false).is_empty());
    }
}
