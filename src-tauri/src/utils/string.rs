/// Counting by `chars()` keeps multi-byte UTF-8 sequences (e.g. accented
/// characters) atomic, which `&str[..n]` byte slicing does not.
pub fn truncate_chars(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        s.to_string()
    } else {
        chars[..max].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::truncate_chars;

    #[test]
    fn truncate_chars_under_max() {
        assert_eq!(truncate_chars("Polish", 6), "Polish");
    }

    #[test]
    fn truncate_chars_over_max() {
        assert_eq!(truncate_chars("Translate", 6), "Transl");
    }

    #[test]
    fn truncate_chars_unicode_safe() {
        assert_eq!(truncate_chars("résumé", 6), "résumé");
        assert_eq!(truncate_chars("résumé!", 6), "résumé");
    }

    #[test]
    fn truncate_chars_empty() {
        assert_eq!(truncate_chars("", 6), "");
    }

    #[test]
    fn truncate_chars_emoji_grapheme() {
        // A 4-byte emoji counts as a single char.
        assert_eq!(truncate_chars("a😀b", 6), "a😀b");
    }
}
