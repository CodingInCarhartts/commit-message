/// A structured commit message with subject and optional body
#[derive(Debug, Clone, PartialEq)]
pub struct CommitMessage {
    pub subject: String,
    pub body: Option<String>,
}

impl CommitMessage {
    /// Create a new commit message with just a subject
    pub fn new(subject: String) -> Self {
        Self { subject, body: None }
    }

    /// Parse a message from AI response
    ///
    /// Expected format:
    /// ```
    /// SUBJECT: feat(scope): description
    /// BODY: detailed explanation (or "none")
    /// ```
    pub fn parse_from_ai_response(response: &str) -> Self {
        let response = response.trim();

        // Try to parse structured format
        if response.contains("SUBJECT:") {
            let mut subject = String::new();
            let mut body = String::new();
            let mut in_body = false;

            for line in response.lines() {
                let line = line.trim();
                if line.starts_with("SUBJECT:") {
                    subject = line.trim_start_matches("SUBJECT:").trim().to_string();
                } else if line.starts_with("BODY:") {
                    in_body = true;
                    let body_start = line.trim_start_matches("BODY:").trim();
                    if !body_start.is_empty() && body_start.to_lowercase() != "none" {
                        body = body_start.to_string();
                    }
                } else if in_body && !line.is_empty() {
                    if !body.is_empty() {
                        body.push('\n');
                    }
                    body.push_str(line);
                }
            }

            if !subject.is_empty() {
                return Self {
                    subject,
                    body: if body.is_empty() { None } else { Some(body) },
                };
            }
        }

        // Fallback: treat first line as subject, rest as body
        let mut lines = response.lines();
        let subject = lines.next().unwrap_or("").trim().to_string();
        let body: String = lines
            .skip_while(|l| l.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        Self {
            subject,
            body: if body.trim().is_empty() { None } else { Some(body) },
        }
    }

    /// Format as a git commit message (with blank line between subject and body)
    pub fn to_git_message(&self) -> String {
        match &self.body {
            Some(body) => format!("{}\n\n{}", self.subject, body),
            None => self.subject.clone(),
        }
    }

    /// Get a display string for the interactive prompt
    pub fn to_display_string(&self) -> String {
        self.to_git_message()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_structured_response() {
        let response = "SUBJECT: feat(auth): add JWT validation\nBODY: Implement token validation.";
        let msg = CommitMessage::parse_from_ai_response(response);
        assert_eq!(msg.subject, "feat(auth): add JWT validation");
        assert_eq!(msg.body, Some("Implement token validation.".to_string()));
    }

    #[test]
    fn test_parse_no_body() {
        let response = "SUBJECT: fix: resolve null pointer\nBODY: none";
        let msg = CommitMessage::parse_from_ai_response(response);
        assert_eq!(msg.subject, "fix: resolve null pointer");
        assert_eq!(msg.body, None);
    }

    #[test]
    fn test_to_git_message() {
        let msg = CommitMessage {
            subject: "feat: add feature".to_string(),
            body: Some("This is the body.".to_string()),
        };
        assert_eq!(msg.to_git_message(), "feat: add feature\n\nThis is the body.");
    }
}
