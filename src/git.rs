use std::process::Command;

pub type GitResult<T> = Result<T, GitError>;

#[derive(Debug)]
pub enum GitError {
    NotARepository,
    NoStagedChanges,
    CommandFailed(String),
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotARepository => write!(f, "Not in a git repository"),
            Self::NoStagedChanges => write!(f, "No staged changes to commit"),
            Self::CommandFailed(msg) => write!(f, "Git command failed: {}", msg),
        }
    }
}

impl std::error::Error for GitError {}

/// Check if current directory is inside a git repository
pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get the last N commit messages (oneline format)
pub fn get_commit_history(count: usize) -> GitResult<String> {
    let output = Command::new("git")
        .args(["log", "--oneline", &format!("-{}", count)])
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Get the actual diff content of staged changes
pub fn get_staged_diff(max_lines: usize) -> GitResult<String> {
    let output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    let diff = String::from_utf8_lossy(&output.stdout);

    if diff.trim().is_empty() {
        return Err(GitError::NoStagedChanges);
    }

    let lines: Vec<&str> = diff.lines().collect();

    if lines.len() <= max_lines {
        Ok(diff.to_string())
    } else {
        Ok(format!(
            "{}\n\n... [TRUNCATED: {} more lines not shown] ...",
            lines[..max_lines].join("\n"),
            lines.len() - max_lines
        ))
    }
}

/// Get a statistical summary of the diff
pub fn get_diff_stat() -> String {
    Command::new("git")
        .args(["diff", "--cached", "--stat"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

/// Count the number of staged files
pub fn count_staged_files() -> usize {
    Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().count())
        .unwrap_or(0)
}

/// Commit staged changes with the given message
pub fn commit(message: &str) -> GitResult<()> {
    let status = Command::new("git")
        .args(["commit", "-m", message])
        .status()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        Err(GitError::CommandFailed("Commit failed".into()))
    }
}

/// Push to the default remote
pub fn push() -> GitResult<()> {
    let status = Command::new("git")
        .arg("push")
        .status()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        Err(GitError::CommandFailed("Push failed".into()))
    }
}
