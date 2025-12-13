/// Mapping of conventional commit types to their emoji representations
pub const EMOJI_MAP: &[(&str, &str, &str)] = &[
    ("feat", "✨", "New feature"),
    ("fix", "🐛", "Bug fix"),
    ("docs", "📚", "Documentation"),
    ("style", "💄", "Code style/formatting"),
    ("refactor", "♻️", "Code refactoring"),
    ("test", "✅", "Tests"),
    ("chore", "🔧", "Chores/maintenance"),
    ("perf", "⚡", "Performance"),
    ("ci", "👷", "CI/CD"),
    ("build", "📦", "Build system"),
    ("revert", "⏪", "Revert changes"),
    ("wip", "🚧", "Work in progress"),
    ("security", "🔒", "Security fix"),
    ("deps", "📌", "Dependencies"),
    ("release", "🚀", "Release"),
];

/// Get the emoji for a commit type
pub fn get_emoji(commit_type: &str) -> Option<&'static str> {
    let type_lower = commit_type.to_lowercase();
    EMOJI_MAP
        .iter()
        .find(|(t, _, _)| *t == type_lower)
        .map(|(_, emoji, _)| *emoji)
}

/// Extract the commit type from a conventional commit message
pub fn extract_type(message: &str) -> Option<&str> {
    let first_line = message.lines().next()?;

    // Find the position of the first special character (!, (, or :)
    let type_end = first_line
        .find(|c| c == '!' || c == '(' || c == ':')
        .unwrap_or(first_line.len());

    if type_end > 0 {
        Some(&first_line[..type_end])
    } else {
        None
    }
}

/// Add emoji prefix to a commit message
pub fn add_emoji_prefix(message: &str) -> String {
    if let Some(commit_type) = extract_type(message) {
        if let Some(emoji) = get_emoji(commit_type) {
            // Check if already has an emoji (avoid double-adding)
            let first_char = message.chars().next();
            if first_char.map(|c| c.is_ascii_alphabetic()).unwrap_or(false) {
                return format!("{} {}", emoji, message);
            }
        }
    }
    message.to_string()
}

/// Remove emoji prefix from a commit message
pub fn remove_emoji_prefix(message: &str) -> String {
    let trimmed = message.trim_start();

    // Check if starts with a known emoji
    for (_, emoji, _) in EMOJI_MAP {
        if trimmed.starts_with(emoji) {
            return trimmed[emoji.len()..].trim_start().to_string();
        }
    }

    message.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_emoji() {
        assert_eq!(get_emoji("feat"), Some("✨"));
        assert_eq!(get_emoji("fix"), Some("🐛"));
        assert_eq!(get_emoji("FEAT"), Some("✨"));
        assert_eq!(get_emoji("unknown"), None);
    }

    #[test]
    fn test_extract_type() {
        assert_eq!(extract_type("feat: add feature"), Some("feat"));
        assert_eq!(extract_type("fix(auth): fix bug"), Some("fix"));
        assert_eq!(extract_type("feat!: breaking"), Some("feat"));
    }

    #[test]
    fn test_add_emoji_prefix() {
        assert_eq!(add_emoji_prefix("feat: add feature"), "✨ feat: add feature");
        assert_eq!(add_emoji_prefix("fix(auth): fix bug"), "🐛 fix(auth): fix bug");
    }

    #[test]
    fn test_remove_emoji_prefix() {
        assert_eq!(remove_emoji_prefix("✨ feat: add feature"), "feat: add feature");
        assert_eq!(remove_emoji_prefix("feat: no emoji"), "feat: no emoji");
    }
}
