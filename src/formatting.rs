/// Format a UUID for display with a prefix.
pub fn display_id(prefix: &str, id: &uuid::Uuid) -> String {
    format!("{prefix}-{id}")
}

/// Truncate a string to a maximum length, appending "..." if
/// truncated.
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_owned();
    }
    let truncated = &s[..max_len.saturating_sub(3)];
    format!("{truncated}...")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_id_with_prefix() {
        let id = uuid::Uuid::nil();
        let result = display_id("item", &id);
        assert!(result.starts_with("item-"));
    }

    #[test]
    fn truncate_short_string() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_long_string() {
        let result = truncate("this is a long string", 10);
        assert_eq!(result, "this is...");
    }

    #[test]
    fn truncate_exact_length() {
        assert_eq!(truncate("12345", 5), "12345");
    }
}
