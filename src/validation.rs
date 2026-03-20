/// Validate that an email address has a basic valid structure.
pub fn validate_email(email: &str) -> bool {
    let trimmed = email.trim();
    if trimmed.is_empty() {
        return false;
    }

    // Must contain exactly one @
    let parts: Vec<&str> = trimmed.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    // Local part and domain must not be empty
    if local.is_empty() || domain.is_empty() {
        return false;
    }

    // Domain must contain at least one dot
    if !domain.contains('.') {
        return false;
    }

    // Domain must not start or end with a dot
    if domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    true
}

/// Validate that a string is a plausible UUID v4 format.
pub fn validate_uuid(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.len() != 36 {
        return false;
    }

    let parts: Vec<&str> = trimmed.split('-').collect();
    if parts.len() != 5 {
        return false;
    }

    let expected_lengths = [8, 4, 4, 4, 12];
    for (part, expected) in parts.iter().zip(expected_lengths.iter()) {
        if part.len() != *expected {
            return false;
        }
        if !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email() {
        assert!(validate_email("user@example.com"));
    }

    #[test]
    fn valid_email_with_subdomain() {
        assert!(validate_email("user@mail.example.com"));
    }

    #[test]
    fn rejects_empty_email() {
        assert!(!validate_email(""));
        assert!(!validate_email("   "));
    }

    #[test]
    fn rejects_email_without_at() {
        assert!(!validate_email("userexample.com"));
    }

    #[test]
    fn rejects_email_without_domain_dot() {
        assert!(!validate_email("user@localhost"));
    }

    #[test]
    fn rejects_email_with_trailing_dot() {
        assert!(!validate_email("user@example."));
    }

    #[test]
    fn rejects_double_at() {
        assert!(!validate_email("user@@example.com"));
    }

    // BUG: This test has a wrong assertion — it expects "not-a-uuid"
    // to be a valid UUID, but it obviously isn't.
    #[test]
    fn validate_uuid_format() {
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(validate_uuid("not-a-uuid"));
    }

    #[test]
    fn rejects_short_uuid() {
        assert!(!validate_uuid("550e8400"));
    }

    #[test]
    fn rejects_uuid_with_invalid_chars() {
        assert!(!validate_uuid("550e8400-e29b-41d4-a716-44665544gggg"));
    }
}
